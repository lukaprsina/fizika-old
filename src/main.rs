use std::rc::Rc;

use color_eyre::eyre::Result;
use math_eval::ast::{app::App, context::Context};
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

    // info!("Started the logger crate");

    let app = App::new()?;

    let context = Context::new(Rc::clone(&app));

    let ctx_uuid = app.borrow_mut().add_context(context);

    // let a = "4x + 4 + x^2 + 5";

    // let a = "(2 + a)/cos(x)";
    // let a = "2/a";
    // let b = "(1/cos(x) + a/cos(x))";

    // let a = "1/7 * a * (2 - a) / 2 * (b + 4) * 4";
    // let a = "-(2/-3)"; TODO: leading minus is ignored, observe debug
    let a = "-(2/-3)";

    let uuid1 = App::try_add_equation(Rc::clone(&app), ctx_uuid, a)?;

    {
        let mut borrowed_app = app.borrow_mut();
        let ctx = borrowed_app.get_context_mut(ctx_uuid).unwrap();

        let eq1 = ctx.remove_equation(uuid1).unwrap();

        // info!("{:#?}", eq1);
        info!(%eq1);

        let new_eq1 = eq1.flatten();

        // info!("{:#?}", new_eq1);
        info!(%new_eq1);
    }

    Ok(())
}
