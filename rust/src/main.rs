use color_eyre::Result;
use fizika::{
    database::add_to_meilisearch, html::extract_html, init, javascript::parse_js,
    scrape::scrape_normal,
};

#[tokio::main]
pub async fn main() -> Result<()> {
    init()?;

    println!("scrape_normal");
    scrape_normal().await?;

    println!("parse_js");
    parse_js()?;

    println!("extract_html");
    extract_html()?;

    println!("add_to_meilisearch");
    add_to_meilisearch().await?;

    Ok(())
}
