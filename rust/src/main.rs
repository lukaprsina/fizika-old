use color_eyre::Result;
use fizika::{
    parse_file::parse_file,
    recurse_node::{ALT_COUNTER, QUESTION_MARK_COUNTER},
    utils::ChapterInfo,
};

use std::{
    collections::HashMap,
    fs::{self, create_dir_all, remove_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};
use uuid::Uuid;

fn main() -> Result<()> {
    color_eyre::install()?;

    let courses_dir = Path::new("courses");
    let output_dir = Path::new("../react/courses");

    if output_dir.exists() {
        remove_dir_all(&output_dir)?;
    }

    let mut chapter_info_json = {
        let chapter_info_path = Path::new("chapter_infos.json");
        let chapter_info_string = fs::read_to_string(chapter_info_path)?;
        serde_json::from_str::<Vec<ChapterInfo>>(&chapter_info_string)?
    };

    let mut i = 0;
    // while i < 1 {
    loop {
        let course_dir = courses_dir.join(i.to_string());
        let course_output_dir = output_dir.join(i.to_string());
        let mut page_num = 0;

        if course_dir.is_dir() {
            create_dir_all(&course_output_dir)?;

            {
                let config_path = course_output_dir.join("config.json");
                let mut config_file = File::create(&config_path)?;

                let mut config = &mut chapter_info_json[i];
                let heading = &mut config.heading.clone();
                let (number, name) = heading.split_at(3);

                config.heading = name.to_string();
                let num_char = number.chars().next().unwrap();
                config.year = Some(char::to_digit(num_char, 10).unwrap());

                let config_json = serde_json::to_string_pretty(&config)?;
                config_file.write_all(config_json.as_bytes())?;
            }

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

                if i == 21 {
                    if j > 34 && j < 38 {
                        j = 38;
                        popups.clear();
                        popup_count = 0;
                        continue;
                    }
                }

                let exercise_file = course_dir.join(format!("page_{}.html", j));
                let output_exercise_dir =
                    course_output_dir.join(format!("pages/page_{}", page_num));
                create_dir_all(&output_exercise_dir)?;

                if exercise_file.is_file() {
                    parse_file(
                        exercise_file,
                        &mut last_exercise_dir,
                        course_output_dir.clone(),
                        output_exercise_dir,
                        popup,
                        &mut popup_count,
                        &mut popups,
                    )?;

                    if !popup {
                        page_num += 1;
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

    unsafe {
        println!("Missing alt attributes: {}", ALT_COUNTER);
        println!("Questions marks: {}", QUESTION_MARK_COUNTER);
    }

    fs::write(
        "../react/chapter_infos.json",
        serde_json::to_string_pretty(&chapter_info_json)?.as_bytes(),
    )?;

    Ok(())
}
