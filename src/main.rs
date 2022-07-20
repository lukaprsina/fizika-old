use std::rc::Rc;

use color_eyre::eyre::Result;
use math_eval::ast::context::Context;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    color_eyre::install()?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Started the logger crate");

    let context = Context::new();

    Context::try_add_equation(Rc::clone(&context), "4 + 4x + x^2 + 5").unwrap();

    Context::try_add_equation(Rc::clone(&context), "a^2 + 2a*b + b^2").unwrap();

    context.borrow().solve();
    Ok(())
}
