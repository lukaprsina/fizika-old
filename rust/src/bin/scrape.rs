use std::{
    error::Error,
    fs::{create_dir, remove_dir_all, remove_file, File},
    io::Write,
    path::Path,
};

use fizika::utils::{get_chapter_info, get_links, ChapterInfo};
use itertools::Itertools;
use select::{
    document::Document,
    predicate::{Child, Class},
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
        let chapter_info = process_tab(course_document, dir_name.as_path(), pos)?;

        chapter_infos.push(chapter_info);
    }

    let chapter_info_dir = Path::new("chapter_infos.json");
    if chapter_info_dir.exists() {
        remove_file(&chapter_info_dir)?
    };

    let mut chapter_infos_file = File::create(&chapter_info_dir)?;
    let json = serde_json::ser::to_string_pretty(&chapter_infos)?;
    chapter_infos_file.write_all(json.as_bytes())?;

    eprintln!("Run \"python fix_courses.py\"");

    Ok(())
}

fn process_tab(
    course_document: Document,
    dir_name: &Path,
    course_pos: usize,
) -> Result<ChapterInfo, Box<dyn Error>> {
    let pages = course_document
        .find(Child(
            select::predicate::Attr("id", "container"),
            Class("eplxSlide"),
        ))
        .collect_vec();

    let mut title_slides = course_document
        .find(Child(
            select::predicate::Attr("id", "container"),
            Class("eplxTitleslide"),
        ))
        .collect_vec();
    let title_slide = title_slides.remove(0);

    let chapter_info = get_chapter_info(title_slide)?;

    let mut page_pos = 0;
    for page in pages.into_iter().skip(4) {
        // TODO: exercise double
        if course_pos == 24 && (page_pos >= 32 && page_pos <= 35) {
            page_pos = 36;
            continue;
        }

        let new_path = dir_name.join(format!("page_{}.html", page_pos));
        let mut f = File::create(new_path)?;
        f.write_all(page.html().as_bytes())?;

        if course_pos == 27 && page_pos == 104 {
            // TODO: two popup links, both broken
            page_pos += 1;
            let new_path = dir_name.join(format!("page_{}.html", page_pos));
            let mut f = File::create(new_path)?;
            f.write_all(page.html().as_bytes())?;
        }

        page_pos += 1;
    }

    Ok(chapter_info)
}
