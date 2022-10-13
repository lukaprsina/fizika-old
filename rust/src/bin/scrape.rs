use color_eyre::Result;
use fizika::scrape_utils::{fix_courses, get_links, process_tab};
use select::document::Document;
use std::{
    error::Error,
    fs::{create_dir, remove_dir_all, remove_file, File},
    io::Write,
    path::Path,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url_str = "http://fizika.sc-nm.si";
    let url = url::Url::parse(url_str)?;

    let resp = reqwest::get(url.clone()).await?;
    let html = resp.text().await?;
    let document = Document::from(html.as_str());

    let lines = get_links(&document)?;

    let pages_dir = Path::new("./courses");

    if pages_dir.exists() {
        remove_dir_all(&pages_dir)?;
    }

    create_dir(&pages_dir)?;

    let pages_dir = pages_dir.canonicalize()?;
    let mut chapter_infos = vec![];

    for (pos, line) in lines.into_iter().enumerate() {
        let new_address = url.clone().join(&line)?;
        let course_resp = reqwest::get(new_address).await?;
        let course_html = course_resp.text().await?;
        let course_document = Document::from(course_html.as_str());

        let dir_name = pages_dir.join(pos.to_string());
        create_dir(&dir_name)?;
        let dir_name = dir_name.canonicalize()?;
        let chapter_info = process_tab(&course_document, dir_name.as_path(), pos)?;

        chapter_infos.push(chapter_info);
    }

    let chapter_info_dir = Path::new("chapter_infos.json");
    if chapter_info_dir.exists() {
        remove_file(&chapter_info_dir)?
    };

    let mut chapter_infos_file = File::create(&chapter_info_dir)?;
    let json = serde_json::ser::to_string_pretty(&chapter_infos)?;
    chapter_infos_file.write_all(json.as_bytes())?;

    fix_courses()?;

    Ok(())
}
