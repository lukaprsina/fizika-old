use std::rc::Rc;

use color_eyre::eyre::Result;
use itertools::Itertools;
use math_eval::ast::{app::App, context::Context};
use once_cell::sync::Lazy;
use rustyline::{error::ReadlineError, Editor};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

// TODO: vec remove unwrap
fn main() -> Result<()> {
    color_eyre::install()?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .without_time()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app = App::new()?;

    let context = Context::new(Rc::clone(&app));

    let ctx_uuid = app.borrow_mut().add_context(context);

    let mut rl = Editor::<()>::new()?;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let uuid = App::try_add_equation(Rc::clone(&app), ctx_uuid, line.as_str())?;
                let mut borrowed_app = app.borrow_mut();
                let ctx = borrowed_app.get_context_mut(ctx_uuid).unwrap();

                let eq = ctx.remove_equation(uuid).unwrap();

                println!("{}", eq);
                // println!("{:#?}", eq);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
