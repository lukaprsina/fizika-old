use std::{
    error::Error,
    fs::{create_dir, remove_dir_all, remove_file, File},
    io::{stdin, BufRead, Write},
    path::Path,
    sync::Arc,
    thread::sleep,
    time::Duration,
};

use fizika::{create_fizika_tab, get_links, get_only_element};
use headless_chrome::{Element, Tab};
use itertools::Itertools;
use scraper::Html;
use select::{
    document::Document,
    node::Node,
    predicate::{Class, Name},
};
use serde::{Deserialize, Serialize};
use url::Url;

fn main() -> Result<(), Box<dyn Error>> {
    {
        println!("This will delete every course. Type \"yes\" if you want to proceed");

        let stdin = stdin();
        let mut iterator = stdin.lock().lines();
        let line = iterator.next().unwrap().unwrap();

        if line != "yes" {
            println!("Quitting");
            panic!();
        }
    }

    let (tab, _browser) = create_fizika_tab()?;

    let lines = get_links(Arc::clone(&tab))?;
    let url = Url::parse(&tab.get_url())?;
    let pages_dir = Path::new("./courses");

    if pages_dir.exists() {
        remove_dir_all(&pages_dir)?;
    }

    create_dir(&pages_dir)?;

    let pages_dir = pages_dir.canonicalize()?;
    let mut chapter_infos = vec![];

    for (pos, line) in lines.into_iter().enumerate() {
        let new_address = url.join(&line)?;
        tab.navigate_to(&new_address.to_string())?;
        tab.wait_until_navigated()?;

        let dir_name = pages_dir.join(pos.to_string());
        create_dir(&dir_name)?;
        create_dir(dir_name.join("exercises"))?;
        let dir_name = dir_name.canonicalize()?;

        let chapter_info = process_tab(Arc::clone(&tab), dir_name.as_path())?;
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

    Ok(())
}

fn process_tab(tab: Arc<Tab>, dir_name: &Path) -> Result<ChapterInfo, Box<dyn Error>> {
    let pages = tab.find_elements("#container > .eplxSlide")?;

    let mut title_slides = tab.find_elements("#container > .eplxTitleslide")?;
    let title_slide = title_slides.remove(0); // the second one is the ending

    let chapter_info = get_chapter_info(title_slide)?;

    let mut index = 1;

    for page in pages.iter().skip(4) {
        let html = page
            .call_js_fn("function() { return this.outerHTML; }", false)?
            .value
            .expect("Can't get HTML from div");

        /* sleep(Duration::from_secs(1));
        let mathjax = page.call_js_fn("function() { return MathJax.Hub; }", false)?;

        println!("{:#?}", mathjax);
        println!("{:#?}", mathjax.object_type);
        println!("{:#?}", mathjax.value);

        let mut math_file = File::create(dir_name.join("math.json"))?;
        math_file.write_all(mathjax.value.unwrap().to_string().as_bytes())?; */

        let data = Html::parse_fragment(html.as_str().expect("Can't parse HTML"));

        let new_path = dir_name.join(format!("exercises/page {}.html", index));
        let mut f = File::create(new_path).expect("Unable to create file");

        let html_string = data.root_element().html();
        f.write_all(html_string.as_bytes())
            .expect("Unable to write data");
        index += 1;
    }

    Ok(chapter_info)
}

#[derive(Deserialize, Serialize)]
struct ChapterInfo {
    heading: String,
    author: Option<String>,
    goals: Option<String>,
}

fn get_chapter_info(title_slide: Element) -> Result<ChapterInfo, Box<dyn Error>> {
    let html = title_slide
        .call_js_fn("function() { return this.innerHTML; }", false)?
        .value
        .expect("Can't get innerHTML on div");

    let document = Document::from(html.as_str().unwrap());
    let texts = document.find(Class("logo_txt")).collect_vec();
    let text = get_only_element(texts);

    let headings = text.find(Name("h1")).collect_vec();
    let heading = get_only_element(headings);

    let iter = text.find(Name("h3")).collect_vec();
    let author = iter.get(0);
    let goals = iter.get(1);

    Ok(ChapterInfo {
        heading: heading.inner_html().trim().to_string(),
        author: author.map(get_not_span),
        goals: goals.map(get_not_span),
    })
}

fn get_not_span(x: &Node) -> String {
    let mut result = String::new();

    for child in x.children() {
        if !child.is(Name("span")) {
            result = child.html()
        }
    }

    result
}
