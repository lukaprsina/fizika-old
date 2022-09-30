use color_eyre::Result;
use fizika::parse_file;

use std::{
    collections::HashMap,
    fs::{create_dir_all, remove_dir_all},
    path::{Path, PathBuf},
};
use uuid::Uuid;

fn main() -> Result<()> {
    color_eyre::install()?;

    let courses_dir = Path::new("courses");
    let output_dir = Path::new("output");

    if output_dir.exists() {
        remove_dir_all(&output_dir)?;
    }

    let mut i = 0;
    while i < 1 {
        let course_dir = courses_dir.join(i.to_string());
        let course_output_dir = output_dir.join(i.to_string());
        let mut page_num = 0;

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

                if i == 21 {
                    if j > 34 && j < 38 {
                        j = 38;
                        popups.clear();
                        popup_count = 0;
                        continue;
                    }
                }

                let exercise_file = course_dir.join(format!("page_{}.html", j));
                let output_exercise_dir = course_output_dir.join(format!("page_{}", page_num));
                create_dir_all(&output_exercise_dir)?;

                if exercise_file.is_file() {
                    let popup2 = parse_file::parse_file(
                        exercise_file,
                        &mut last_exercise_dir,
                        course_output_dir.clone(),
                        output_exercise_dir,
                        popup,
                        &mut popup_count,
                        &mut popups,
                    )?;

                    assert_eq!(popup, popup2);

                    if !popup2 {
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

    Ok(())
}
