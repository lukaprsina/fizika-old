use std::rc::Rc;

use color_eyre::eyre::Result;
use math_eval::ast::{app::App, context::Context};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    color_eyre::install()?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .without_time()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Started the logger crate");

    let app = App::new()?;

    let context = Context::new(Rc::clone(&app));

    let ctx_uuid = app.borrow_mut().add_context(context);

    // let a = "4x + 4 + x^2 + 5";

    // let a = "(2 + a)/cos(x)";
    // let a = "2/a";
    // let b = "(1/cos(x) + a/cos(x))";

    let a = "2+3";
    let b = "4-5";

    let e1 = App::try_add_equation(Rc::clone(&app), ctx_uuid, a)?;
    let e2 = App::try_add_equation(Rc::clone(&app), ctx_uuid, b)?;

    let uuid1 = e1.uuids.first().unwrap();
    let uuid2 = e2.uuids.first().unwrap();

    {
        let borrowed_app = app.borrow_mut();
        let ctx = borrowed_app.get_context(ctx_uuid).unwrap();

        let expr1 = ctx.get_element(*uuid1).unwrap();
        let expr2 = ctx.get_element(*uuid2).unwrap();

        println!("EXPR1:\n{expr1}\n");
        println!("EXPR2:\n{expr2}\n");
        println!("{}", expr1.element.clone() * expr2.element.clone());
    }

    Ok(())
}
