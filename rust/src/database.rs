use std::{fs, path::Path};

use color_eyre::Result;
use meilisearch_sdk::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    html: String,
}

#[tracing::instrument]
pub async fn add_to_meilisearch() -> Result<()> {
    let url = std::env::var("MEILI_HTTP_ADDR")?;
    let master_password = std::env::var("MEILI_MASTER_KEY")?;
    let client = Client::new(url, master_password);
    let courses_dir = Path::new("courses_output");

    if !client.is_healthy().await {
        panic!();
    }

    let _task_info = client.delete_index("courses").await.ok();
    let courses = client.index("courses");

    let mut i = 0;
    loop {
        let course_dir = courses_dir.join(i.to_string());
        if course_dir.is_dir() {
            let mut j = 0;

            loop {
                let exercise_file = course_dir.join(format!("pages/page_{}/index.html", j));
                println!("{:#?}", exercise_file);

                if exercise_file.is_file() {
                    let file = fs::read_to_string(&exercise_file)?;
                    courses
                        .add_documents(&[Page { html: file }], Some(&format!("{}_{}", i, j)))
                        .await?;
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
