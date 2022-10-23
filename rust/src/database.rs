use std::{fmt::Display, fs, path::Path, str::FromStr, time::Instant};

use color_eyre::Result;
use meilisearch_sdk::Client;
use select::document::Document;
use serde::{Deserialize, Serialize};
use tracing::log::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    id: String,
    html: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageDisplay {
    html: String,
    text: String,
}

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Text: {}", &self.text[..20])
    }
}

impl Display for PageDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Text: {}", &self.text[..20])
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MeilisearchError {
    #[error("Can't create an URL")]
    URL,
    #[error("Can't set database settings")]
    Settings,
}

#[tracing::instrument]
pub async fn add_to_meilisearch() -> Result<()> {
    let master_password = std::env::var("MEILI_MASTER_KEY")?;
    let url = "http://localhost:7700";

    info!("{}, {}", url, master_password);
    let client = Client::new(url, master_password);

    // client.delete_index("courses").await.unwrap();
    let courses = client.index("courses");

    {
        let displayed_attributes = ["text", "html"];
        let ranking_rules = ["words", "typo", "attribute", "exactness", "cost:asc"];
        let searchable_attributes = ["text"];

        let settings = meilisearch_sdk::settings::Settings::new()
            .with_ranking_rules(ranking_rules)
            .with_searchable_attributes(searchable_attributes)
            .with_displayed_attributes(displayed_attributes)
            .with_distinct_attribute("id");

        courses
            .set_settings(&settings)
            .await
            .map_err(|_| MeilisearchError::Settings)?;
    }

    let courses_dir = Path::new("courses_output");

    let mut i = 0;
    // while i < 1 {
    loop {
        let course_dir = courses_dir.join(i.to_string());
        if course_dir.is_dir() {
            let mut j = 0;

            loop {
                let exercise_file = course_dir.join(format!("pages/page_{}/index.html", j));
                println!("{:#?}", exercise_file);

                if exercise_file.is_file() {
                    let file = fs::read_to_string(&exercise_file)?;

                    let document = Document::from(tendril::StrTendril::from_str(&file).unwrap());
                    let text = document
                        .nth(0)
                        .expect("Can't get div for text")
                        .text()
                        .trim()
                        .to_string();

                    courses
                        .add_or_update(
                            &[Page {
                                html: file,
                                text,
                                id: format!("{}_{}", i, j),
                            }],
                            Some("id"),
                        )
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

    /* loop {
        let message = inquire::Text::new("Search or type document id").prompt()?;

        if message.starts_with("id ") {
            let a = message.split_once("id ").unwrap();
            let start = Instant::now();
            let page = courses.get_document::<Page>(a.1).await?;
            let duration = start.elapsed();

            println!("{} {}", page, duration.as_millis());
        } else {
            let result = courses
                .search()
                .with_query(&message)
                .execute::<PageDisplay>()
                .await?;

            println!("{}", result.processing_time_ms);
            for hit in result.hits {
                println!("{}", hit.result);
            }
        }
    } */
    Ok(())
}
