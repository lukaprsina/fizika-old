use color_eyre::Result;
use fizika::{html2::extract_html2, init};

pub fn main() -> Result<()> {
    init()?;
    extract_html2()?;

    Ok(())
}
