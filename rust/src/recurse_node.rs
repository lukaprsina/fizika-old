use std::{collections::HashMap, io::Write};

use itertools::Itertools;
use katex::OutputType;
use select::{
    node::Node,
    predicate::{And, Class},
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
    let ending_tag: Option<()> = match node.name() {
        Some(name) => {
            match name {
                "p" => None,
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
                        fix_formula(&mut formula);

                        let opts = katex::Opts::builder()
                            .display_mode(display_mode)
                            .output_type(OutputType::Mathml)
                            .build()
                            .unwrap();
                        let _mathml = katex::render_with_opts(&formula, opts).unwrap();
                        /* println!(
                            "{}\n{}\n",
                            "-".repeat(60)
                        ); */
                    }
                    None
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
                        popups.insert(href, uuid);

                        /* elem.push_attribute((
                            "onclick",
                            format!("() => {{openModal(\'{}\')}}", uuid).as_str(),
                        )); */
                    } else if node.is(Class("goToHidden")) {
                    } else {
                    }

                    None
                }
                _name => None,
            }
        }
        None => None,
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

    if let Some(end) = ending_tag {
        writer.write(XmlEvent::end_element()).unwrap();
        // writer.write_event(Event::End(end)).unwrap();
    }
}
