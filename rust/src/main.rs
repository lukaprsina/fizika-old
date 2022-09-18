use build_html::{Html, HtmlPage};
use color_eyre::Result;
use fizika::utils::get_only_element;
use itertools::Itertools;
use katex::OutputType;
use select::{
    document::Document,
    node::Node,
    predicate::{And, Class},
};
use std::{
    collections::HashMap,
    fs::{create_dir_all, remove_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};
use thiserror::Error;
use uuid::Uuid;

fn main() -> Result<()> {
    color_eyre::install()?;

    let courses_dir = Path::new("courses");
    let output_dir = Path::new("output");

    if output_dir.exists() {
        remove_dir_all(&output_dir)?;
    }

    let mut i = 0;
    loop {
        let course_dir = courses_dir.join(i.to_string());
        let course_output_dir = output_dir.join(i.to_string());

        if course_dir.is_dir() {
            let mut j = 0;
            let mut popup_count = 0;

            let mut last_exercise_dir = PathBuf::new();
            let mut popups: HashMap<String, Uuid> = HashMap::new();
            loop {
                let popup = if popup_count == 0 {
                    false
                } else {
                    popup_count -= 1;
                    true
                };

                if i == 24 && j == 2
                /* 32 */
                {
                    println!("a");
                }

                if i == 21 {
                    if j > 34 && j < 38 {
                        j = 38;
                        popups.clear();
                        popup_count = 0;
                        continue;
                    }
                }

                let exercise_file = course_dir.join(format!("page_{}.html", j));
                let output_exercise_dir = course_output_dir.join(format!("page_{}", j));
                create_dir_all(&output_exercise_dir)?;

                if exercise_file.is_file() {
                    let file = exercise_file.as_path();
                    let file = file.canonicalize()?;
                    let name = file.to_str().unwrap();

                    println!(
                        "{}\n{} -> popup: {}, popup count: {}\n",
                        "-".repeat(50),
                        name,
                        popup,
                        popup_count
                    );
                    let content = std::fs::read_to_string(name)?;
                    let document = Document::from(tendril::StrTendril::from_str(&content).unwrap());
                    let mut output_page = build_html::HtmlPage::new();

                    if popup {
                        let html_uuid = process_popup(document, &mut output_page)?;
                        let popup_dir = last_exercise_dir.join("popups");
                        create_dir_all(&popup_dir)?;

                        match popups.remove(&html_uuid) {
                            Some(new_uuid) => {
                                let mut name = new_uuid.to_string();
                                name.push_str(".html");

                                let mut file = File::create(popup_dir.join(&name))?;
                                file.write_all(output_page.to_html_string().as_bytes())?;
                            }
                            None => {
                                // TODO: hidden popup
                                let lost_popups = course_output_dir.join("lost_popups");

                                create_dir_all(&lost_popups)?;
                                let mut file = File::create(
                                    lost_popups.join(&format!("{}.html", html_uuid.as_str())),
                                )?;
                                file.write_all(output_page.to_html_string().as_bytes())?;

                                println!("{}", html_uuid);
                                popup_count += 1;
                            }
                        }
                    } else {
                        let result = process_exercise(document, &mut output_page);
                        let index_file = output_exercise_dir.join("index.html");
                        let mut file = File::create(&index_file)?;
                        file.write_all(output_page.to_html_string().as_bytes())?;
                        last_exercise_dir = output_exercise_dir.as_path().to_owned();

                        match result {
                            Ok(new_popups) => {
                                println!("{:#?}", &new_popups);
                                popup_count = new_popups.len();
                                popups = new_popups;
                            }
                            Err(err) => {
                                if err == ExerciseError::HiddenExercise {
                                    // TODO: popup_count += 1;
                                }
                            }
                        }
                    }
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

fn process_popup(document: Document, output_page: &mut HtmlPage) -> Result<String> {
    let areas = document.find(Class("popupContent")).collect_vec();
    let area = get_only_element(areas);

    let exercises = document
        .find(And(Class("eplxSlide"), Class("popupImpl")))
        .collect_vec();
    let exercise = get_only_element(exercises);
    let uuid = exercise.attr("id").unwrap();

    let mut parent_vec = vec![];
    let _map: HashMap<usize, Vec<Option<String>>> = HashMap::new();

    let mut popups: HashMap<String, Uuid> = HashMap::new();
    node_hot(area, &mut parent_vec, &mut popups);

    // TODO: pri kvizu so za naprej
    // assert_eq!(popups.len(), 0);

    Ok(uuid.to_string())
}

#[derive(Error, Debug, PartialEq, PartialOrd)]
enum ExerciseError {
    #[error("This is a hidden exercise")]
    HiddenExercise,
}

fn process_exercise(
    document: Document,
    output_page: &mut HtmlPage,
) -> Result<HashMap<String, Uuid>, ExerciseError> {
    let exercises = document.find(Class("eplxSlide")).collect_vec();
    let exercise = get_only_element(exercises);

    let area = if exercise.is(Class("popupImpl")) {
        return Err(ExerciseError::HiddenExercise);
    } else if exercise.is(Class("eplxLastSlide")) {
        None
    } else {
        let subheadings = exercise.find(Class("subheading")).collect_vec();
        if subheadings.is_empty() {
            unreachable!("{}\n", exercise.html());
        }
        let _subheading = get_only_element(subheadings);

        let areas = document.find(Class("interactive-area")).collect_vec();
        let area = get_only_element(areas);

        Some(area)
    };

    if let Some(area) = area {
        let mut parent_vec = vec![];
        let _map: HashMap<usize, Vec<Option<String>>> = HashMap::new();

        let mut popups: HashMap<String, Uuid> = HashMap::new();
        node_hot(area, &mut parent_vec, &mut popups);
        return Ok(popups);
    }

    Ok(HashMap::new())
}

fn node_hot(node: Node, parents: &mut Vec<Option<String>>, popups: &mut HashMap<String, Uuid>) {
    match node.name() {
        Some(name) => match name {
            "script" => {
                if let Some(attr_type) = node.attr("type") {
                    let display_mode = match attr_type {
                        "math/tex" => false,
                        "math/tex; mode=display" => true,
                        _ => panic!("Script is not math"),
                    };

                    let script_children = node.children().collect_vec();
                    let script_child = get_only_element(script_children);

                    let formula = script_child.as_text().unwrap();
                    let mut formula = formula
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
                        formula = formula.replace(long.0, long.1)
                    }

                    let opts = katex::Opts::builder()
                        .display_mode(display_mode)
                        .output_type(OutputType::Mathml)
                        .build()
                        .unwrap();
                    /* println!(
                        "{}\n{}\n",
                        katex::render_with_opts(&formula, opts).unwrap(),
                        "-".repeat(60)
                    ); */
                }
            }
            "a" => {
                // TODO: skip non-explanetory ones like 7-1
                if node.is(And(Class("goToSlide"), Class("explain"))) {
                    let mut href = node
                        .attr("href")
                        .expect("goToSlide must have an href")
                        .to_string();
                    href.remove(0);

                    popups.insert(href, Uuid::new_v4());
                } else if node.is(Class("goToHidden")) {
                } else {
                }
            }
            _ => (),
        },
        None => (),
    }
    for child in node.children() {
        let mut new_parents = parents.clone();

        let maybe_name = match child.name() {
            Some(name) => Some(name.to_string()),
            None => None,
        };

        new_parents.push(maybe_name);

        node_hot(child, &mut new_parents, popups);
    }
}

/* fn process_node<'a>(
    node: Node<'a>,
    parents: &'a mut Vec<Option<String>>,
    counts: &mut HashMap<usize, Vec<Option<String>>>,
) -> Node<'a> {
    let maybe_count = counts.remove(&parents.len());
    let new_count = match maybe_count {
        Some(_) => parents.clone(),
        None => vec![],
    };

    counts.insert(parents.len(), new_count);

    for child in node.children() {
        let maybe_name = child.name();

        match maybe_name {
            Some(name) => match name {
                "span" | "p" | "a" | "b" | "i" => (),
                "li" | "ul" | "ol" | "div" | "h1" | "caption" | "h2" | "iframe" | "h3" => (),
                "img" => {
                    println!("{:#?}", parents);
                }
                "table" | "tbody" | "td" | "tr" => (),
                "nobr" => (),
                "canvas" => (),
                "input" | "label" => (),
                "script" => {
                    if let Some(attr_type) = child.attr("type") {
                        if attr_type == "math/tex" {
                            // println!("{}", child.inner_html());
                        } else if attr_type == "math/tex; mode=display" {
                        } else {
                            println!("{}: {}", attr_type, child.html());
                            panic!("Script is not math");
                        }
                    }
                }
                _ => panic!("{}", name),
            },
            None => (),
        }

        let maybe_name = match maybe_name {
            Some(name) => Some(name.to_string()),
            None => None,
        };

        /* println!(
            "{:#?}\n\n{:#?}\n\n{}\n\n",
            node.html(),
            maybe_name,
            "-".repeat(60)
        ); */
        for grandchild in child.children() {
            let mut new_parents = parents.clone();
            new_parents.push(maybe_name.clone());
            println!("{:?}", grandchild.name());
            process_node(grandchild, &mut new_parents, counts);
        }
    }

    node
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
 */
