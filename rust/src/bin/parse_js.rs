use color_eyre::Result;
use fizika::javascript::parse_js;

pub fn main() -> Result<()> {
    parse_js()?;

    Ok(())
}
