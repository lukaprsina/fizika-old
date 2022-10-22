use color_eyre::Result;
use headless_chrome::{Element, Tab};
use itertools::Itertools;
use reqwest::Url;
use select::document::Document;
use std::{
    error::Error,
    fs::{self, create_dir, create_dir_all, remove_dir_all, remove_file},
    path::Path,
    sync::Arc,
    thread::sleep,
    time::Duration,
};

use crate::{
    scrape_utils::{create_fizika_tab, fix_courses, get_links, process_tab},
    MATH_NOT_RENDERED_COUNTER,
};

pub fn scrape_normal() -> Result<()> {
    let url_str = "http://fizika.sc-nm.si";
    let url = url::Url::parse(url_str)?;

    let resp = reqwest::blocking::get(url.clone())?;
    let html = resp.text()?;
    let document = Document::from(html.as_str());

    let lines = get_links(&document)?;

    let pages_dir = Path::new("courses");

    if pages_dir.exists() {
        remove_dir_all(&pages_dir)?;
    }

    create_dir(&pages_dir)?;

    let pages_dir = pages_dir.canonicalize()?;
    let mut chapter_infos = vec![];

    for (pos, line) in lines.into_iter().enumerate() {
        let new_address = url.clone().join(&line)?;
        let course_resp = reqwest::blocking::get(new_address)?;
        let course_html = course_resp.text()?;
        let course_document = Document::from(course_html.as_str());

        let dir_name = pages_dir.join(pos.to_string()).join("exercises");
        create_dir_all(&dir_name)?;
        fs::write(&dir_name.join("../index.html"), course_html.as_bytes())?;
        let chapter_info = process_tab(&course_document, &dir_name.as_path(), pos)?;

        chapter_infos.push(chapter_info);

        sleep(Duration::from_millis(200));
    }

    let chapter_info_dir = Path::new("chapter_infos.json");
    if chapter_info_dir.exists() {
        remove_file(&chapter_info_dir)?
    };

    let json = serde_json::ser::to_string_pretty(&chapter_infos)?;
    fs::write(&chapter_info_dir, json.as_bytes())?;

    fix_courses()?;

    Ok(())
}

pub fn scrape_chrome() -> Result<(), Box<dyn Error>> {
    let (tab, _browser) = create_fizika_tab()?;

    let document = get_html_from_element(&get_element_from_tab(&tab));

    let lines = get_links(&Document::from(document.as_str()))?;
    let url = Url::parse(&tab.get_url())?;
    fs::write(
        "links.txt",
        lines
            .iter()
            .map(|line| url.join(&line).unwrap().to_string())
            .join("\n")
            .as_bytes(),
    )?;

    let pages_dir = Path::new("courses");

    if pages_dir.exists() {
        remove_dir_all(&pages_dir)?;
    }

    create_dir_all(&pages_dir)?;

    let pages_dir = pages_dir.canonicalize()?;
    let mut chapter_infos = vec![];

    for (pos, line) in lines.into_iter().skip(0).enumerate() {
        let new_address = url.join(&line)?;
        tab.navigate_to(&new_address.to_string())?;
        tab.wait_until_navigated().unwrap();

        let document_element = get_element_from_tab(&tab);
        let document_html = get_html_from_element(&document_element);

        let dir_name = pages_dir.join(pos.to_string()).join("exercises");
        create_dir_all(&dir_name)?;

        let chapter_info = process_tab(
            &Document::from(document_html.as_str()),
            dir_name.as_path(),
            pos,
        )?;

        chapter_infos.push(chapter_info);
    }

    let chapter_info_dir = Path::new("chapter_infos.json");
    if chapter_info_dir.exists() {
        remove_file(&chapter_info_dir)?
    };

    let json = serde_json::ser::to_string_pretty(&chapter_infos)?;
    fs::write(&chapter_info_dir, json.as_bytes())?;

    fix_courses()?;

    unsafe {
        println!("Math not rendered: {}", MATH_NOT_RENDERED_COUNTER);
    }

    Ok(())
}

fn get_element_from_tab<'a>(tab: &'a Arc<Tab>) -> Element<'a> {
    tab.find_element("html").unwrap()
}

fn get_html_from_element(element: &Element) -> String {
    let html_value = element
        .call_js_fn("function () { return this.outerHTML; }", vec![], false)
        .unwrap();

    let course_html = html_value.value.unwrap();
    course_html.as_str().unwrap().to_string()
}
