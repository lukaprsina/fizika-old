use color_eyre::Result;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub mod actions;
pub mod ast;
pub mod output;
pub mod tokenizer;

pub fn initialize() -> Result<()> {
    color_eyre::install()?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .without_time()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    Ok(())
}
