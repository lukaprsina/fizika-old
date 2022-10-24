use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use color_eyre::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{uppercase_first_letter, ChapterInfo};

#[derive(Debug, Serialize, Deserialize)]
pub struct Script {
    pub slides: Vec<Slide>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Slide {
    pub skip: bool,
    #[serde(rename = "type")]
    pub slide_type: String,
    pub namespace: String,
    pub id: String,
    pub name: String,

    // unused
    pub next: Option<String>,
    pub prev: Option<String>,
    pub navigation: Option<String>,
}

#[tracing::instrument]
pub fn extract_html2() -> Result<()> {
    let courses_dir = Path::new("courses");
    let output_dir = Path::new("courses_output");

    if output_dir.exists() {
        fs::remove_dir_all(&output_dir)?;
    }

    let mut chapter_info_json = {
        let chapter_info_path = Path::new("chapter_infos.json");
        let chapter_info_string = fs::read_to_string(chapter_info_path)?;
        serde_json::from_str::<Vec<ChapterInfo>>(&chapter_info_string)?
    };

    let mut i = 0;
    loop {
        let course_dir = courses_dir.join(i.to_string());
        let course_output_dir = output_dir.join(i.to_string());

        if course_dir.is_dir() {
            fs::create_dir_all(&course_output_dir)?;

            {
                let config_path = course_output_dir.join("config.json");

                let mut config = &mut chapter_info_json[i];
                let heading = &mut config.heading.clone();
                let (number, name) = heading.split_at(3);

                config.heading = name.to_string();

                for text_maybe in [&mut config.author, &mut config.goals] {
                    if let Some(text) = text_maybe {
                        let new = text.trim();
                        *text = uppercase_first_letter(new);
                    }
                }
                let new = config.heading.trim();
                config.heading = uppercase_first_letter(new);

                let num_char = number.chars().next().unwrap();
                config.year = Some(char::to_digit(num_char, 10).unwrap());

                let config_json = serde_json::to_string_pretty(&config)?;
                fs::write(&config_path, config_json.as_bytes())?;
            }

            // TODO
            let id_to_file: HashMap<Uuid, PathBuf> = HashMap::new();
            {
                let script = fs::read_to_string(course_dir.join("output.json"))?;
                let script_rs = serde_json::from_str::<Script>(&script)?;

                for slide in script_rs.slides {
                    println!("{}", slide.slide_type);
                }
            }
            /* let mut j = 0;
            let mut page_num = 0;
            loop {
                let page_path = course_dir.join(format!("pages/page_{}.html", j));
                let output_page_dir = course_output_dir.join(format!("pages/page_{}", page_num));
                fs::create_dir_all(&output_page_dir)?;

                if page_path.is_file() {
                    let mut popup;
                    parse_doc(&course_dir, &mut popup);

                    if !popup {
                        page_num += 1;
                    }
                } else {
                    break;
                }

                j += 1;
            } */
        } else {
            break;
        }

        i += 1;
    }

    Ok(())
}
