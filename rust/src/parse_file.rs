use color_eyre::Result;
use select::document::Document;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
    str::FromStr,
};
use uuid::Uuid;
use xml::EmitterConfig;

use crate::{
    process_html::{process_exercise, process_popup, ExerciseError},
    recurse_node,
};

pub fn parse_file(
    exercise_file: PathBuf,
    last_exercise_dir: &mut PathBuf,
    course_output_dir: PathBuf,
    output_exercise_dir: PathBuf,
    popup: bool,
    popup_count: &mut usize,
    popups: &mut HashMap<String, Uuid>,
) -> Result<()> {
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
    let mut config = EmitterConfig::new().perform_indent(true);
    config.perform_escaping = false;
    config.write_document_declaration = false;

    let (file_maybe, area_maybe) = if popup {
        let popup_dir = last_exercise_dir.join("popups");
        let (html_uuid, area) = process_popup(&document)?;
        create_dir_all(&popup_dir)?;

        let file_maybe = match popups.remove(&html_uuid) {
            Some(new_uuid) => {
                let mut name = new_uuid.to_string();
                name.push_str(".html");

                Some(File::create(popup_dir.join(&name))?)
            }
            None => {
                // TODO: hidden popup
                let lost_popups = course_output_dir.join("lost_popups");

                create_dir_all(&lost_popups)?;
                *popup_count += 1;
                // println!("{}", html_uuid);

                Some(File::create(
                    lost_popups.join(&format!("{}.html", html_uuid.as_str())),
                )?)
            }
        };

        (file_maybe, Some(area))
    } else {
        match process_exercise(&document) {
            Ok(result) => {
                if let Some((area, subheading)) = result {
                    let index_path = output_exercise_dir.join("index.html");
                    let index_file = File::create(&index_path)?;
                    *last_exercise_dir = output_exercise_dir.as_path().to_owned();

                    {
                        let config_path = output_exercise_dir.join("config.json");
                        let mut config_file = File::create(&config_path)?;
                        let config_json = serde_json::to_string_pretty(&PageConfig {
                            subheading: subheading.text(),
                        })?;
                        config_file.write_all(config_json.as_bytes())?;
                    }

                    (Some(index_file), Some(area))
                } else {
                    (None, None)
                }
            }
            Err(err) => {
                if err == ExerciseError::HiddenExercise {
                    // TODO: TODO: popup_count += 1;
                }
                (None, None)
            }
        }
    };

    if let (Some(file), Some(area)) = (file_maybe, area_maybe) {
        let mut writer = config.create_writer(file);
        let mut parents: Vec<Option<String>> = Vec::new();
        let mut new_popups: HashMap<String, Uuid> = HashMap::new();
        let mut question_mark_course = 0;

        recurse_node::recurse_node(
            area,
            &mut parents,
            &mut new_popups,
            &mut writer,
            &mut question_mark_course,
        );

        // println!("{}", question_mark_course);

        if !popup {
            *popup_count = new_popups.len();
            *popups = new_popups;
        }
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageConfig {
    pub subheading: String,
}
