use itertools::Itertools;
use select::{
    document::Document,
    node::Node,
    predicate::{Class, Descendant, Name},
};
use serde::{Deserialize, Serialize};
use std::error::Error;

pub fn get_chapter_info(title_slide: Node) -> Result<ChapterInfo, Box<dyn Error>> {
    let html = title_slide.inner_html();

    let document = Document::from(html.as_str());
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

pub fn get_only_element<T>(mut elements: Vec<T>) -> T {
    assert_eq!(elements.len(), 1);
    elements.remove(0)
}

pub fn get_not_span(x: &Node) -> String {
    let mut result = String::new();

    for child in x.children() {
        if !child.is(Name("span")) {
            result = child.html()
        }
    }

    result
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

#[derive(Deserialize, Serialize)]
pub struct ChapterInfo {
    pub heading: String,
    pub author: Option<String>,
    pub goals: Option<String>,
}
