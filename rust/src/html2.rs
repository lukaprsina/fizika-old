use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use color_eyre::Result;
use itertools::Itertools;
use select::{document::Document, predicate};
use serde::{Deserialize, Serialize};

use crate::utils::{get_only_element, uppercase_first_letter, ChapterInfo};

#[derive(Debug, Serialize, Deserialize)]
pub struct Script {
    pub slides: Vec<Slide>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Slide {
    pub skip: bool,
    #[serde(rename = "type")]
    pub slide_type: String,
    pub namespace: String,
    pub id: String,
    pub name: String,
    pub title: Option<String>,

    // unused
    pub next: Option<String>,
    pub prev: Option<String>,
    pub navigation: Option<String>,
}

#[tracing::instrument]
pub fn extract_html2() -> Result<()> {
    let courses_dir = Path::new("courses");
    let output_dir = Path::new("courses_output");

    if output_dir.exists() {
        fs::remove_dir_all(&output_dir)?;
    }

    let mut chapter_info_json = {
        let chapter_info_path = Path::new("chapter_infos.json");
        let chapter_info_string = fs::read_to_string(chapter_info_path)?;
        serde_json::from_str::<Vec<ChapterInfo>>(&chapter_info_string)?
    };

    let mut i = 0;
    loop {
        // skip quizes
        if i == 2 || i == 3 {
            i += 1;
            continue;
        }

        let mut missing_fs_html_count = 0;
        let mut missing_fs_html_string = String::new();

        let course_dir = courses_dir.join(i.to_string());
        let course_output_dir = output_dir.join(i.to_string());

        if !course_dir.is_dir() {
            break;
        }

        fs::create_dir_all(&course_output_dir)?;
        println!("{course_dir:#?}");

        let config_path = course_output_dir.join("config.json");
        write_config(&mut chapter_info_json, &config_path, i)?;

        let mut pages: HashMap<String, (Document, PathBuf, usize)> = HashMap::new();
        let mut j = 0;

        loop {
            if !read_pages_fs(&mut pages, &course_dir, j) {
                break;
            }

            j += 1;
        }

        let mut missing_slides = Vec::new();

        let script = fs::read_to_string(course_dir.join("output.json"))?;
        let script_rs = serde_json::from_str::<Script>(&script)?;

        let length = script_rs.slides.len();
        for (pos, slide) in script_rs.slides.iter().enumerate() {
            if pos == 0 || pos == length - 2 {
                continue;
            }

            match pages.remove(&slide.id) {
                Some((document, page_path, page_num)) => {
                    parse_page_js_and_fs(&slide, &document, &page_path);
                    let page_path = course_dir.join(format!("pages/page_{}.html", page_num));

                    let output_page_dir = course_output_dir.join("pages");
                    let output_page_path = output_page_dir.join(format!("page_{}", page_num));

                    fs::create_dir_all(&output_page_dir)?;

                    if output_page_path.is_file() {
                        panic!("{:#?} already exists", output_page_path);
                    } else if page_path.is_file() {
                        // parse_doc(&course_dir);
                        fs::write(&output_page_path, "contents".as_bytes())?;
                    } else {
                        panic!("{:#?} is not a file", page_path);
                    }
                }
                None => {
                    missing_fs_html_count += 1;
                    missing_fs_html_string += &format!("{} - {}\n", slide.name, slide.id);
                    if slide.name != "endslide" {
                        missing_slides.push(slide);
                    }
                }
            }
        }

        // end slide
        if pages.len() != 0 {
            if let Some(page) = pages.get("courses/8/pages/page_91.html") {
                panic!(
                    "There are more html pages than slides in js: {:#?}, {}",
                    page.1, page.2
                );
            }
        }

        dbg!(missing_fs_html_count);
        fs::write(
            course_dir.join("missing.txt"),
            missing_fs_html_string.as_bytes(),
        )?;

        if i != 24 {
            for missing_slide in missing_slides {
                let js_type = missing_slide.slide_type.clone();
                match js_type.as_str() {
                    "transition" => {
                        // TODO: transitions not handled
                    }
                    _ => panic!("{js_type}"),
                }
            }
        }

        i += 1;
    }

    Ok(())
}

fn write_config(
    chapter_info_json: &mut Vec<ChapterInfo>,
    config_path: &Path,
    i: usize,
) -> Result<()> {
    let mut config = &mut chapter_info_json[i];
    let heading = &mut config.heading.clone();
    let (number, name) = heading.split_at(3);

    config.heading = name.to_string();

    for text_maybe in [&mut config.author, &mut config.goals] {
        if let Some(text) = text_maybe {
            let new = text.trim();
            *text = uppercase_first_letter(new);
        }
    }
    let new = config.heading.trim();
    config.heading = uppercase_first_letter(new);

    let num_char = number.chars().next().unwrap();
    config.year = Some(char::to_digit(num_char, 10).unwrap());

    let config_json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, config_json.as_bytes())?;

    Ok(())
}

fn read_pages_fs(
    pages: &mut HashMap<String, (Document, PathBuf, usize)>,
    course_dir: &Path,
    page_num: usize,
) -> bool {
    let page_path = course_dir.join(format!("pages/page_{}.html", page_num));
    // println!("{}\n{:#?}", "-".repeat(50), page_path);

    if !page_path.exists() {
        return false;
    }

    let content = std::fs::read_to_string(&page_path).unwrap();
    let document = Document::from(tendril::StrTendril::from_str(&content).unwrap());

    let outer_divs = document.find(predicate::Class("eplxSlide")).collect_vec();
    let outer_div = get_only_element(outer_divs);

    assert!(outer_div.is(predicate::Name("div")));
    let html_id = outer_div.attr("id").expect("Page has no id");

    pages.insert(html_id.to_string(), (document, page_path, page_num));

    return true;
}

fn parse_page_js_and_fs(slide: &Slide, document: &Document, page_path: &Path) {
    let outer_divs = document.find(predicate::Class("eplxSlide")).collect_vec();
    let outer_div = get_only_element(outer_divs);

    let html_name = outer_div.attr("name").expect("Page has no name");
    assert_eq!(html_name, slide.name);

    let html_is_popup = outer_div.is(predicate::Class("popupImpl"));
    match slide.slide_type.as_str() {
        "popup" => assert!(html_is_popup),
        "slide" => assert!(!html_is_popup),
        _ => panic!("Type {} missing", slide.slide_type),
    }
}
