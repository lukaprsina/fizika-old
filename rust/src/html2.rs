use std::{
    collections::HashMap,
    fs::{self, File},
    path::{Path, PathBuf},
    str::FromStr,
};

use color_eyre::Result;
use itertools::Itertools;
use select::{document::Document, predicate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use xml::EmitterConfig;

use crate::{
    parse_file::PageConfig,
    process_html::{process_exercise, process_popup, ExerciseError},
    recurse_node::recurse_node,
    utils::{get_only_element, uppercase_first_letter, ChapterInfo},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Script {
    pub slides: Vec<Slide>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

        if !process_chapter(i, courses_dir, output_dir, &mut chapter_info_json)? {
            break;
        }

        i += 1;
    }

    Ok(())
}

fn process_chapter(
    i: usize,
    courses_dir: &Path,
    output_dir: &Path,
    chapter_info_json: &mut Vec<ChapterInfo>,
) -> Result<bool> {
    let mut missing_fs_html_count = 0;
    let mut missing_fs_html_string = String::new();

    let course_dir = courses_dir.join(i.to_string());
    let course_output_dir = output_dir.join(i.to_string());

    if !course_dir.is_dir() {
        return Ok(false);
    }

    fs::create_dir_all(&course_output_dir)?;
    println!("{course_dir:#?}");

    let config_path = course_output_dir.join("config.json");
    write_config(chapter_info_json, &config_path, i)?;

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
    let mut exercises: Vec<Exercise> = vec![];

    for (pos, slide) in script_rs.slides.iter().enumerate() {
        if pos == 0 || pos == length - 2 {
            continue;
        }

        match pages.remove(&slide.id) {
            Some((document, page_path_js, page_num)) => {
                parse_page_js_and_fs(&slide, &document, &mut exercises);
                let input_page_path = course_dir.join(format!("pages/page_{}.html", page_num));
                assert_eq!(input_page_path, page_path_js);

                // parse_exercise(&document, &output_page_path, &slide)?;
                if !input_page_path.is_file() {
                    panic!("{:#?} is not a file", input_page_path);
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

    println!("{}", exercises.len());
    for (page_num, exercise) in exercises.into_iter().enumerate() {
        let output_page_dir = course_output_dir.join("pages");
        let output_page_path = output_page_dir.join(format!("page_{}", page_num));
        fs::create_dir_all(&output_page_path)?;
        parse_exercise2(exercise, &output_page_path)?;
    }

    Ok(true)
}

fn parse_exercise2(exercise: Exercise, output_page_path: &Path) -> Result<()> {
    match process_exercise(&exercise.document) {
        Ok(result) => {
            if let Some((area, subheading)) = result {
                let index_path = output_page_path.join("index.html");
                println!("{output_page_path:#?}");
                let index_file = File::create_new(&index_path)?;

                {
                    let config_path = output_page_path.join("config.json");
                    let config_json = serde_json::to_string_pretty(&PageConfig {
                        subheading: subheading.text(),
                    })?;
                    fs::write(&config_path, config_json.as_bytes())?;
                }

                let mut config = EmitterConfig::new().perform_indent(true);
                config.perform_escaping = false;
                config.write_document_declaration = false;

                write_node_to_file(index_file, area, config);
            }
        }
        Err(err) => {
            if err == ExerciseError::HiddenExercise {
                // TODO: popup_count += 1;
            }
        }
    };

    let popups_dir = output_page_path.join("popups");
    if !exercise.popups.is_empty() {
        fs::create_dir_all(&popups_dir)?;
    }

    for popup in exercise.popups {
        let (html_uuid, area) = process_popup(&popup.document)?;
        assert_eq!(html_uuid, popup.slide.id);
        let popup_dir = popups_dir.join(format!("{}.html", popup.slide.id));
        let file = File::create_new(&popup_dir)?;

        let mut config = EmitterConfig::new().perform_indent(true);
        config.perform_escaping = false;
        config.write_document_declaration = false;

        write_node_to_file(file, area, config);
    }

    Ok(())
}

fn write_node_to_file(file: File, area: select::node::Node, config: EmitterConfig) {
    let mut writer = config.create_writer(file);
    let mut parents: Vec<Option<String>> = Vec::new();
    let mut new_popups: HashMap<String, Uuid> = HashMap::new();
    let mut question_mark_course = 0;

    recurse_node(
        area,
        &mut parents,
        &mut new_popups,
        &mut writer,
        &mut question_mark_course,
    );
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

fn parse_page_js_and_fs(slide: &Slide, document: &Document, exercises: &mut Vec<Exercise>) {
    let outer_divs = document.find(predicate::Class("eplxSlide")).collect_vec();
    let outer_div = get_only_element(outer_divs);

    let html_name = outer_div.attr("name").expect("Page has no name");
    assert_eq!(html_name, slide.name);

    let html_is_popup = outer_div.is(predicate::Class("popupImpl"));
    match slide.slide_type.as_str() {
        "popup" => {
            assert!(html_is_popup);
            exercises
                .last_mut()
                .expect("Popup without a slide")
                .popups
                .push(Popup {
                    slide: slide.clone(),
                    document: document.clone(),
                });
        }
        "slide" => {
            assert!(!html_is_popup);
            exercises.push(Exercise {
                popups: vec![],
                slide: slide.clone(),
                document: document.clone(),
            });
        }
        _ => panic!("Type {} missing", slide.slide_type),
    }
}

pub struct Exercise {
    pub popups: Vec<Popup>,
    pub slide: Slide,
    pub document: Document,
}

pub struct Popup {
    pub slide: Slide,
    pub document: Document,
}
