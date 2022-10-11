use std::{collections::HashMap, io::Write};

use itertools::Itertools;
use katex::OutputType;
use select::{
    node::Node,
    predicate::{And, Class, Comment},
};
use uuid::Uuid;
use xml::{writer::XmlEvent, EventWriter};

use crate::utils::{fix_formula, get_only_element};

pub fn recurse_node<W: Write>(
    node: Node,
    parents: &mut Vec<Option<String>>,
    popups: &mut HashMap<String, Uuid>,
    writer: &mut EventWriter<W>,
) {
    if node.is(Class("placeholder-for-subslides")) {
        return;
    }

    let mut default_tag = |name: &str| {
        let event: XmlEvent = XmlEvent::start_element(name).into();
        writer.write(event).unwrap();
        true
    };

    let ending_tag = match node.name() {
        Some(name) => {
            match name {
                "script" => {
                    if let Some(attr_type) = node.attr("type") {
                        let display_mode = match attr_type {
                            "math/tex" => false,
                            "math/tex; mode=display" => true,
                            _ => panic!("Script is not math"),
                        };

                        let script_children = node.children().collect_vec();
                        let script_child = get_only_element(script_children);

                        let mut formula = script_child.as_text().unwrap().to_string();
                        unsafe {
                            QUESTION_MARK_COUNTER += formula.matches('?').count() as i32;
                        }
                        fix_formula(&mut formula);

                        let opts = katex::Opts::builder()
                            .display_mode(display_mode)
                            .output_type(OutputType::Html)
                            .build()
                            .unwrap();
                        let mathml = katex::render_with_opts(&formula, opts).unwrap();
                        writer.write(XmlEvent::Characters(&mathml)).unwrap();
                    }
                    false
                }
                "div" => match node.attr("href") {
                    Some(href) => {
                        let event: XmlEvent = XmlEvent::start_element("video").into();
                        writer.write(event).unwrap();

                        let source: XmlEvent =
                            XmlEvent::start_element("source").attr("href", href).into();
                        writer.write(source).unwrap();
                        writer.write(XmlEvent::end_element()).unwrap();
                        true
                    }
                    None => default_tag("div"),
                },
                "img" => {
                    let src = node.attr("src").unwrap();
                    let mut start_event = XmlEvent::start_element("img").attr("src", src);

                    match node.attr("alt") {
                        Some(alt) => {
                            start_event = start_event.attr("alt", alt);
                        }
                        None => unsafe {
                            ALT_COUNTER += 1;
                        },
                    }

                    let event: XmlEvent = start_event.into();
                    writer.write(event).unwrap();
                    true
                }
                "a" => {
                    // TODO: skip non-explanetory ones like 7-1

                    if node.is(And(Class("goToSlide"), Class("explain"))) {
                        let mut href = node
                            .attr("href")
                            .expect("goToSlide must have an href")
                            .to_string();
                        href.remove(0);

                        let uuid = Uuid::new_v4();
                        let uuid_str = uuid.to_string();
                        popups.insert(href, uuid);
                        let event: XmlEvent = XmlEvent::start_element("button")
                            .attr("onclick", "() => course.openModal()")
                            .attr("data-id", &uuid_str)
                            .into();
                        writer.write(event).unwrap();
                        true
                    } else if node.is(Class("goToHidden")) {
                        false
                    } else {
                        false
                    }
                }
                name => default_tag(name),
            }
        }
        None => {
            if !node.is(Comment) {
                let html = node.html();
                let event: XmlEvent = XmlEvent::characters(&html).into();
                writer.write(event).unwrap();
                // if node.check_if_text() {}
            }

            false
        }
    };

    for child in node.children() {
        let mut new_parents = parents.clone();

        let maybe_name = match child.name() {
            Some(name) => Some(name.to_string()),
            None => None,
        };

        new_parents.push(maybe_name);

        recurse_node(child, &mut new_parents, popups, writer);
    }

    if ending_tag {
        writer.write(XmlEvent::end_element()).unwrap();
    }
}

pub static mut ALT_COUNTER: i32 = 0;
pub static mut QUESTION_MARK_COUNTER: i32 = 0;

/* trait WalkNode {
    fn check_if_text(&self) -> bool;
}

impl WalkNode for Node<'_> {
    fn check_if_text(&self) -> bool {
        if self.name().is_some() {
            return false;
        }

        for child in self.children() {
            let is_text = child.check_if_text();

            if is_text {
                return false;
            }
        }

        true
    }
}
 */
