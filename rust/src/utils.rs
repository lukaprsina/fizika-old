use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use itertools::Itertools;
use select::{
    document::Document,
    node::Node,
    predicate::{Class, Descendant, Name},
};
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc, thread::sleep, time::Duration};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct ChapterInfo {
    pub heading: String,
    pub author: Option<String>,
    pub goals: Option<String>,
}

pub fn fix_formula(formula: &mut String) {
    let mut fixed = formula
        .replace("\\mbox", "\\,")
        .replace("{%}", "{ \\%}")
        .replace("{ %}", "{ \\%}")
        .replace("{ % }", "{ \\%}")
        .replace("y: F_{tn}-F_S$=0", "y: F_{tn}-F_S=0")
        .replace("^'", "^\\prime")
        .replace("\\frc", "\\frac")
        .replace("\\cdor", "\\cdot")
        .replace("\\codt", "\\cdot")
        .replace("\\epsilo", "\\epsilon")
        .replace("\\epsilonn", "\\epsilon");

    let long_replace = [
                        (
                            "$k=\\frac{\\Delta F}{\\Delta l}=\\frac{60 \\,{ kN}-0 \\,{ kN}}{0,40 \\,{ m}-0,20 \\,{ m}}=300 \\,{ kN/m}",
                            "k=\\frac{\\Delta F}{\\Delta l}=\\frac{60 \\,{ kN}-0 \\,{ kN}}{0,40 \\,{ m}-0,20 \\,{ m}}=300 \\,{ kN/m}"
                        ),
                        (
                            "P_p=\\frac{\\Delta m_p \\cdot q_{izp}}{t_1}= {\\Delta m_p \\cdot q_{izp} \\cdot f",
                            "P_p=\\frac{\\Delta m_p \\cdot q_{izp}}{t_1}= {\\Delta m_p \\cdot q_{izp} \\cdot f}"
                        ),
                        (
                            "v_0=$\\sqrt{2 \\cdot g \\cdot \\Delta h}=6,3 \\,{ m/s} =22,6 \\,{ km/h}",
                            "v_0=\\sqrt{2 \\cdot g \\cdot \\Delta h}=6,3 \\,{ m/s} =22,6 \\,{ km/h}"
                        ),
                        (
                            "\\Sigma F=m \\cdot a$: $-F_{vzmeti}=m \\cdot a",
                            "\\Sigma F=m \\cdot a -F_{vzmeti}=m \\cdot a"
                        )
                    ];

    for long in long_replace {
        fixed = fixed.replace(long.0, long.1)
    }

    *formula = fixed;
}

pub fn create_fizika_tab() -> Result<(Arc<Tab>, Browser), Box<dyn Error>> {
    let options = LaunchOptionsBuilder::default()
        .headless(false)
        .idle_browser_timeout(Duration::from_secs(10 * 60))
        .build()?;

    let browser = Browser::new(options)?;
    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to("http://fizika.sc-nm.si/")?;
    tab.wait_until_navigated()?;
    sleep(Duration::from_secs(1));
    Ok((tab, browser))
}
