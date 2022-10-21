use color_eyre::Result;
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use itertools::Itertools;
use select::{
    document::Document,
    predicate::{Attr, Child, Class, Descendant, Name},
};
use std::{collections::HashMap, error::Error, fs, path::Path, sync::Arc, time::Duration};

use crate::utils::{get_chapter_info, get_only_element, ChapterInfo};

pub fn create_fizika_tab() -> Result<(Arc<Tab>, Browser), Box<dyn Error>> {
    let options = LaunchOptionsBuilder::default()
        .headless(false)
        .idle_browser_timeout(Duration::from_secs(30 * 60))
        .build()?;

    let browser = Browser::new(options)?;
    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to("http://fizika.sc-nm.si/")?;
    tab.wait_until_navigated()?;
    Ok((tab, browser))
}

pub fn fix_courses() -> Result<()> {
    let fixes_in_files: HashMap<&str, Vec<(&str, &str)>> = HashMap::from([
        (
            "./courses/9/exercises/page_19.html",
            Vec::from([(
                "href=\"#resevanjeAVTOosi1\"",
                "href=\"#94c451e0d35ffc9530e5b98660250ae0\"",
            )]),
        ),
        (
            "./courses/27/exercises/page_103.html",
            Vec::from([
                (
                    "href=\"#resevanjevezaveCvec1\"",
                    "href=\"#897a79036e984377a709c192890cc547\"",
                ),
                (
                    "href=\"#resevanjevezaveCvec1\"",
                    "href=\"#6fb1b7ccd9db4229a0982b54c02f2898\"",
                ),
            ]),
        ),
        (
            "./courses/27/exercises/page_104.html",
            Vec::from([(
                "id=\"16c24c5bcf164994d97e797d0e801727\"",
                "id=\"897a79036e984377a709c192890cc547\"",
            )]),
        ),
        (
            "./courses/27/exercises/page_105.html",
            Vec::from([(
                "id=\"16c24c5bcf164994d97e797d0e801727\"",
                "id=\"6fb1b7ccd9db4229a0982b54c02f2898\"",
            )]),
        ),
        (
            "./courses/29/exercises/page_9.html",
            Vec::from([(
                "href=\"#resevanjeMOCvrovalke1\"",
                "href=\"#4b5c16ef569c72e06c764001bbe69ed4\"",
            )]),
        ),
    ]);

    for (file, fixes) in fixes_in_files {
        let mut file_str = fs::read_to_string(file)?;
        for fix in fixes {
            file_str = file_str.replace(fix.0, fix.1);
        }
        fs::write(file, file_str)?;
    }

    Ok(())
}

// previously 37 courses, now 39
pub fn get_links(html: &Document) -> Result<Vec<String>, Box<dyn Error>> {
    let mut links = Vec::new();

    let arr = html
        .find(Descendant(Descendant(Name("body"), Name("div")), Name("a")))
        .collect_vec();

    for element in arr {
        let on_click = element.attr("onclick").expect("No attribute onclick");

        let start = "window.open(\'".len();
        let end = on_click
            .chars()
            .skip(start)
            .position(|c| c == '\'')
            .unwrap();
        let tab_str = on_click.chars().skip(start).take(end).collect::<String>();
        links.push(tab_str);
    }

    Ok(links)
}

pub fn process_tab(
    course_document: &Document,
    dir_name: &Path,
    course_pos: usize,
) -> Result<ChapterInfo, Box<dyn Error>> {
    let pages = course_document
        .find(Child(Attr("id", "container"), Class("eplxSlide")))
        .collect_vec();

    let mut title_slides = course_document
        .find(Child(Attr("id", "container"), Class("eplxTitleslide")))
        .collect_vec();
    let title_slide = title_slides.remove(0);

    let mut chapter_info = get_chapter_info(title_slide)?;
    {
        // link[href='style-specific/screen.css'] + script
        let titles = course_document.find(Name("title")).collect_vec();

        let title = get_only_element(titles);
        let mut temp = title.next().unwrap();
        let mut count = 0;

        loop {
            if temp.is(Name("script")) {
                if count == 1 {
                    break;
                }

                count += 1;
            }

            temp = temp.next().unwrap();
        }

        fs::write(dir_name.join("../script.js"), temp.inner_html())?;
        chapter_info.javascript = Some(temp.inner_html());
    }

    let mut page_pos = 0;
    for page in pages.into_iter().skip(4) {
        // TODO: exercise double
        if course_pos == 24 && (page_pos >= 32 && page_pos <= 35) {
            page_pos = 36;
            continue;
        }

        let new_path = dir_name.join(format!("page_{}.html", page_pos));
        fs::write(&new_path, page.html().as_bytes())?;

        if course_pos == 27 && page_pos == 104 {
            // TODO: two popup links, both broken
            page_pos += 1;
            let new_path = dir_name.join(format!("page_{}.html", page_pos));
            fs::write(&new_path, page.html().as_bytes())?;
        }

        page_pos += 1;
    }

    Ok(chapter_info)
}
