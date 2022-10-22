use color_eyre::Result;
use fizika::{html::extract_html, init};

pub fn main() -> Result<()> {
    init()?;
    extract_html()?;

    Ok(())
}
