use color_eyre::Result;
use fizika::html::extract_html;

pub fn main() -> Result<()> {
    extract_html()?;

    Ok(())
}
