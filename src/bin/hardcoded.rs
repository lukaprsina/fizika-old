use std::rc::Rc;

use color_eyre::eyre::Result;
use itertools::Itertools;
use math_eval::ast::{app::App, context::Context};
use once_cell::sync::Lazy;
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

#[allow(dead_code, unused_variables)]
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

    let mut uuids: Vec<Uuid> = vec![];

    for equation in EQUATIONS.iter() {
        let uuid = App::try_add_equation(Rc::clone(&app), ctx_uuid, equation.as_str())?;
        uuids.push(uuid);
        let mut borrowed_app = app.borrow_mut();
        let ctx = borrowed_app.get_context_mut(ctx_uuid).unwrap();
    }

    {
        let mut borrowed_app = app.borrow_mut();
        let context = borrowed_app.get_context_mut(ctx_uuid).unwrap();

        context.solve();
    }

    for uuid in uuids {
        let mut borrowed_app = app.borrow_mut();
        let ctx = borrowed_app.get_context_mut(ctx_uuid).unwrap();
        let mut eq = ctx.remove_equation(uuid).unwrap();
        // do_stuff(eq);
        let expr = eq.equation_sides.first_mut().unwrap();
        expr.flatten();
        debug!("{expr:#?}");
        debug!("{expr}");
        println!("\n\n");

        /* let simplify = borrowed_app.strategies.get_mut("simplify").unwrap();
        let func = simplify.equation.as_deref_mut().unwrap();
        let mut cloned_eq = eq.clone();
        func(&mut cloned_eq, "x");
        debug!("{cloned_eq:#?}");
        debug!("{cloned_eq}"); */
    }

    Ok(())
}

static EQUATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    let strings = vec![
        // "a*(b+c)", // "f(g(h, x+2))",
        // "-2/(-a/-8)",
        "(-1-2)-3",
        "(-1*(-2))-3",
        "(-1-2)*3 - 3",
        "(-1*(-2))*3 - 3",
        "(1/2)/(3/4)",
    ];
    strings
        .into_iter()
        .map(|string| string.to_string())
        .collect_vec()
});
