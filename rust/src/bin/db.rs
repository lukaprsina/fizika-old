use color_eyre::Result;
use fizika::{database::add_to_meilisearch, init};

#[tokio::main]
pub async fn main() -> Result<()> {
    init()?;
    add_to_meilisearch().await?;

    Ok(())
}
