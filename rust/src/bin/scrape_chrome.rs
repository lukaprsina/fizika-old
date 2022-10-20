use color_eyre::Result;
use fizika::{
    javascript::execute_js,
    scrape_utils::{create_fizika_tab, fix_courses, get_links, process_tab},
    MATH_NOT_RENDERED_COUNTER,
};
use headless_chrome::{Element, Tab};
use itertools::Itertools;
use select::document::Document;
use std::{
    error::Error,
    fs::{self, create_dir_all, remove_dir_all, remove_file},
    path::Path,
    sync::Arc,
};
use url::Url;

// wait for math render to complete
/* let promise = document_element.call_js_fn(
    r#"async function wait_for_math() {
        console.log("\"Wait for math\" function injected")
        return new Promise((resolve, reject) => {
            setTimeout(() => {
                console.log("Math can not be rendered")
                reject(false)
            }, 10_000)

            MathJax.Hub.Register.StartupHook("End",function () {
                console.log("Math rendered")
                resolve(true);
            });
        })
}"#,
    vec![],
    true,
)?;

match promise.value {
    Some(value) => {
        if !value.as_bool().unwrap() {
            unsafe { MATH_NOT_RENDERED_COUNTER += 1 }
        }
    }
    None => unreachable!(),
}
 */

fn main() -> Result<(), Box<dyn Error>> {
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

        let mut chapter_info = process_tab(
            &Document::from(document_html.as_str()),
            dir_name.as_path(),
            pos,
        )?;

        execute_js(&mut chapter_info.javascript)?;
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
