use color_eyre::Result;
use fizika::{html::extract_html, javascript::parse_js, scrape::scrape_normal};

pub fn main() -> Result<()> {
    scrape_normal()?;
    parse_js()?;
    extract_html()?;

    Ok(())
}
