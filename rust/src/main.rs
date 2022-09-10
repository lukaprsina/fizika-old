use headless_chrome::Tab;
use scraper::{ElementRef, Html, Selector};
use std::{error::Error, sync::Arc};

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[derive(Default)]
struct Chapter {
    title: String,
    exercises: Vec<Exercise>,
}

enum MediaType {
    Text(String),
    Image,
    Video,
    Audio,
    Hint,
    Solution,
    Button,
}

struct Exercise {
    content: Vec<MediaType>,
    num_popups: usize,
}

struct Popup {
    content: Vec<MediaType>,
}

fn process_tab(tab: &Arc<Tab>) -> Result<Chapter, Box<dyn Error>> {
    let pages = tab.find_elements("#container > .eplxSlide")?;

    for page in pages.iter().skip(4) {
        let html = page
            .call_js_fn("function() { return this.innerHTML; }", false)?
            .value
            .expect("Can't get innerHTML on div");

        let attributes = page.get_attributes()?.expect("No attributes");
        let popup = attributes
            .get("class")
            .expect("No classes in page div")
            .contains("popupImpl");

        let fragment = Html::parse_fragment(html.as_str().expect("Can't parse HTML"));

        if popup {
            // parse_popup(fragment);
        } else {
            parse_exerice(fragment);
        }
    }

    Ok(Chapter {
        title: "".to_string(),
        exercises: Vec::new(),
    })
}

fn parse_exerice(fragment: Html) -> Exercise {
    let selector = Selector::parse("p.subheading, div.content>div.interactive-area")
        .expect("Can't parse selector");
    let subheading = Selector::parse("p.subheading").expect("Can't parse selector");
    let justified = Selector::parse("p.text-justified").expect("Can't parse selector");

    for element_ref in fragment.select(&selector) {
        let element = element_ref.value();
        println!("{}", element.name());
    }

    /* for element in fragment.select(&selector) {
        println!("{}\n\n{}\n\n", element.html(), "-".repeat(80));
        element.select(&subheading).for_each(|e| {
            println!("Subheading:\n{}", e.html());
        });
        element.select(&justified).for_each(|e| {
            println!("Text Justified:\n{}", e.html());
        });
    } */

    Exercise {
        content: Vec::new(),
        num_popups: 0,
    }
}

fn parse_popup(fragment: Html) -> Popup {
    let selector = Selector::parse("div.popupContent").expect("Can't parse popup selector");

    /* for element in fragment.select(&selector) {
        println!("{}", element.inner_html());
    } */

    Popup {
        content: Vec::new(),
    }
}

fn parse_element(element: ElementRef) {
    for item in element.traverse() {}
}
