use color_eyre::Result;

pub mod database;
pub mod html;
pub mod javascript;
pub mod parse_file;
pub mod process_html;
pub mod recurse_node;
pub mod scrape;
pub mod scrape_utils;
pub mod utils;

pub static mut MATH_NOT_RENDERED_COUNTER: i32 = 0;

pub fn init() -> Result<()> {
    use tracing_subscriber::prelude::*;

    color_eyre::install()?;
    dotenv::dotenv()?;

    let fmt_layer = tracing_subscriber::fmt::layer().with_target(false);
    tracing_subscriber::registry()
        .with(tracing_error::ErrorLayer::default())
        .with(fmt_layer)
        .init();

    Ok(())
}
