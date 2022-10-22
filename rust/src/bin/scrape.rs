use color_eyre::Result;
use fizika::scrape::scrape_normal;

pub fn main() -> Result<()> {
    scrape_normal()?;

    Ok(())
}
