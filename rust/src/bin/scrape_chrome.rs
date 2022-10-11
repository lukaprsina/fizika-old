use std::{
    error::Error,
    fs::{create_dir, remove_dir_all, remove_file, File},
    io::Write,
    path::Path,
    sync::Arc,
    thread::sleep,
    time::Duration,
};

use color_eyre::Result;
use fizika::scrape_utils::{create_fizika_tab, fix_courses, get_links, process_tab};
use headless_chrome::Tab;
use itertools::Itertools;
use select::document::Document;
use url::Url;

fn main() -> Result<(), Box<dyn Error>> {
    /* {
        println!("This will delete every course. Type \"yes\" if you want to proceed");

        let stdin = stdin();
        let mut iterator = stdin.lock().lines();
        let line = iterator.next().unwrap().unwrap();

        if line != "yes" {
            println!("Quitting");
            panic!();
        }
    } */

    let (tab, _browser) = create_fizika_tab()?;

    let document = get_html(Arc::clone(&tab));

    let lines = get_links(&document)?;
    let mut line_file = File::create("./links.txt")?;
    let url = Url::parse(&tab.get_url())?;
    line_file.write_all(
        lines
            .iter()
            .map(|line| url.join(&line).unwrap().to_string())
            .join("\n")
            .as_bytes(),
    )?;

    let pages_dir = Path::new("./courses");

    if pages_dir.exists() {
        remove_dir_all(&pages_dir)?;
    }

    create_dir(&pages_dir)?;

    let pages_dir = pages_dir.canonicalize()?;
    let mut chapter_infos = vec![];

    for (pos, line) in lines.into_iter().skip(0).enumerate() {
        let new_address = url.join(&line)?;
        tab.navigate_to(&new_address.to_string())?;
        tab.wait_until_navigated()?;

        let dir_name = pages_dir.join(pos.to_string());
        create_dir(&dir_name)?;
        create_dir(dir_name.join("exercises"))?;
        let dir_name = dir_name.canonicalize()?;

        let chapter_info = process_tab(get_html(Arc::clone(&tab)), dir_name.as_path(), pos)?;

        chapter_infos.push(chapter_info);
        sleep(Duration::from_millis(500));
    }

    let chapter_info_dir = Path::new("chapter_infos.txt");
    if chapter_info_dir.exists() {
        remove_file(&chapter_info_dir)?
    };

    let mut chapter_infos_file = File::create(&chapter_info_dir)?;
    let json = serde_json::ser::to_string_pretty(&chapter_infos)?;
    chapter_infos_file.write_all(json.as_bytes())?;

    fix_courses()?;

    Ok(())
}

fn get_html(tab: Arc<Tab>) -> Document {
    let html_value = tab
        .find_element("html")
        .unwrap()
        .call_js_fn("function () { return this.outerHTML; }", false)
        .unwrap();

    let course_html = html_value.value.unwrap();
    let course_str = course_html.as_str().unwrap();
    Document::from(course_str)
}
