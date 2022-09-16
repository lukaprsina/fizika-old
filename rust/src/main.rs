use fizika::utils::get_only_element;
use itertools::Itertools;
use select::{
    document::Document,
    node::Node,
    predicate::{Class, Name},
};
use std::{error::Error, path::Path, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let courses_dir = Path::new("courses");

    let mut i = 0;
    loop {
        let course_dir = courses_dir.join(i.to_string());
        if course_dir.is_dir() {
            let mut j = 1;
            loop {
                let exercise_dir = course_dir.join(format!("exercises/page {}.html", j));
                if exercise_dir.is_file() {
                    let file = exercise_dir.as_path();
                    let file = file.canonicalize()?;
                    let name = file.to_str().unwrap();

                    println!("{}\n{}\n", "-".repeat(50), name);
                    let content = std::fs::read_to_string(name)?;
                    let document = Document::from(tendril::StrTendril::from_str(&content).unwrap());
                    process_document(document)?;
                } else {
                    break;
                }

                j += 1;
            }
        } else {
            break;
        }

        i += 1;
    }

    Ok(())
}

fn process_document(document: Document) -> Result<(), Box<dyn Error>> {
    let exercises = document.find(Class("eplxSlide")).collect_vec();
    let exercise = get_only_element(exercises);

    let area = if exercise.is(Class("popupImpl")) {
        let areas = document.find(Class("popupContent")).collect_vec();
        let area = get_only_element(areas);
        Some(area)
    } else if exercise.is(Class("eplxLastSlide")) {
        None
    } else {
        let subheadings = exercise.find(Class("subheading")).collect_vec();
        if subheadings.is_empty() {
            println!("{}\n", exercise.html());
            panic!();
        }
        let _subheading = get_only_element(subheadings);

        let areas = document.find(Class("interactive-area")).collect_vec();
        let area = get_only_element(areas);

        // println!("Subheading:\n{}\n", subheading.inner_html());
        Some(area)
    };

    if let Some(area) = area {
        process_node(&area);
    }
    return Ok(());

    if let Some(area) = area {
        for child in area.children() {
            match child.name() {
                Some(name) => match name {
                    "p" => {
                        let new_children = child.children().collect_vec();
                        if let Some(new_child) = new_children.first() {
                            match new_child.name() {
                                Some(name) => match name {
                                    "a" | "b" | "i" | "span" | "script" => (),
                                    _ => panic!("New name in p: {}: {}", name, new_child.html()),
                                },
                                // text
                                None => (), // println!("{}\n", new_child.html()),
                            }
                        }
                    }
                    "table" => {
                        let t_children = child.children().collect_vec();
                        let mut t_bodies = vec![];

                        for child in t_children {
                            if child.is(Name("tbody")) {
                                t_bodies.push(child);
                            }
                        }

                        match t_bodies.len() {
                            // quiz
                            0 => println!("{}\n-> {}", exercise.html(), child.html()),
                            1 => {
                                let mut t_rows = vec![];
                                let t_body_children = t_bodies[0].children().collect_vec();

                                for child in t_body_children {
                                    if child.is(Name("tr")) {
                                        t_rows.push(child);
                                    }
                                }

                                match t_rows.len() {
                                    0 => println!("{}\n-> {}", exercise.html(), t_bodies[0].html()),
                                    1 => (),
                                    _ => println!("{}\n-> {}", exercise.html(), t_bodies[0].html()),
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    "ul" | "ol" | "div" | "h1" | "img" | "script" | "h2" | "iframe" | "h3"
                    | "a" => (),

                    _ => panic!("New name: {}: {}", name, child.html()),
                },
                None => {
                    let html = child.html();
                    let trimmed_html = html.trim();
                    if !trimmed_html.is_empty() {
                        panic!("{}", trimmed_html);
                    }
                }
            }
        }
    }

    Ok(())
}

fn process_node(node: &Node) {
    for child in node.children() {
        match child.name() {
            Some(name) => match name {
                "p" | "a" | "b" | "i" | "span" | "script" => (),
                "li" | "ul" | "ol" | "div" | "h1" | "img" | "caption" | "h2" | "iframe" | "h3" => {
                    ()
                }
                "table" | "tbody" | "td" | "tr" => (),
                "nobr" => (),
                "canvas" => (),
                "input" | "label" => (),
                _ => panic!("{}", name),
            },
            None => (),
        }
        for grandchild in child.children() {
            process_node(&grandchild);
        }
    }
}

/*
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
 */
