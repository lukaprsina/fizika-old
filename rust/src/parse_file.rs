use color_eyre::Result;
use select::document::Document;
use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    path::PathBuf,
    str::FromStr,
};
use uuid::Uuid;
use xml::{writer::XmlEvent, EmitterConfig};

use crate::process_html::{process_exercise, process_popup, ExerciseError};

pub fn parse_file(
    exercise_file: PathBuf,
    last_exercise_dir: &mut PathBuf,
    course_output_dir: PathBuf,
    output_exercise_dir: PathBuf,
    popup: bool,
    popup_count: &mut usize,
    popups: &mut HashMap<String, Uuid>,
) -> Result<bool> {
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

    let file_maybe = if popup {
        let popup_dir = last_exercise_dir.join("popups");
        let (html_uuid, area) = process_popup(&document)?;
        create_dir_all(&popup_dir)?;

        match popups.remove(&html_uuid) {
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
        }
    } else {
        match process_exercise(&document) {
            Ok((new_popups, area)) => {
                let index_file = output_exercise_dir.join("index.html");
                let file = File::create(&index_file)?;
                *last_exercise_dir = output_exercise_dir.as_path().to_owned();
                // println!("{:#?}", &new_popups);
                *popup_count = new_popups.len();
                *popups = new_popups;
                Some(file)
            }
            Err(err) => {
                if err == ExerciseError::HiddenExercise {
                    // TODO: TODO: popup_count += 1;
                }
                None
            }
        }
    };

    if let Some(file) = file_maybe {
        let writer = config.create_writer(file);
        let event: XmlEvent = xml::writer::XmlEvent::start_element("a")
            .attr("test", "id")
            .into();
    }

    Ok(popup)
}
