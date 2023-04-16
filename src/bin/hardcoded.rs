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

    for (pos, uuid) in uuids.into_iter().enumerate() {
        let mut borrowed_app = app.borrow_mut();
        let context = borrowed_app.get_context_mut(ctx_uuid).unwrap();
        context.solve();

        borrowed_app.apply_strategy("flatten", uuid, ctx_uuid);

        let context = borrowed_app.get_context_mut(ctx_uuid).unwrap();
        context.solve();

        borrowed_app.apply_strategy("simplify", uuid, ctx_uuid);

        let context = borrowed_app.get_context_mut(ctx_uuid).unwrap();
        let eq = context.get_equation(uuid).unwrap();
        debug!("{eq:#?}");

        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;
    }

    Ok(())
}

static EQUATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    let strings = vec![
        "sin(x^3+1)=0",
        // "sin(x+1)=(x^2+1+(f(x)/2))/2",
        // "a*(b+c)",
        // "-2/(-a/-8)",
        // "f(g(h, x+2))",
        // "(-1-2)-3",
        // "(-1*(-2))-3",
        "1-((-2-3)*(-4-5))/((-6-7)*(-8-9))",
        "(-1-2)*3",
        "(-1-2)*3 - 3",
        "(-1*(-2))*3 - 3",
        "(1/2)/(3/4)",
    ];
    strings
        .into_iter()
        .map(|string| string.to_string())
        .collect_vec()
});
