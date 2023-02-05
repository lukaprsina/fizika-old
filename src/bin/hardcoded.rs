use std::rc::Rc;

use color_eyre::eyre::Result;
use itertools::Itertools;
use math_eval::ast::{app::App, context::Context};
use once_cell::sync::Lazy;
use tracing::Level;
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

    for equation in EQUATIONS.iter() {
        let uuid1 = App::try_add_equation(Rc::clone(&app), ctx_uuid, equation.as_str())?;
        let mut borrowed_app = app.borrow_mut();
        let ctx = borrowed_app.get_context_mut(ctx_uuid).unwrap();

        let eq = ctx.remove_equation(uuid1).unwrap();
        do_stuff(eq)
    }

    Ok(())
}

fn do_stuff(eq: math_eval::ast::Equation) {
    // eq.to_string();
    println!("{eq:#?}");
    println!("{eq}");
    /* if let NodeOrExpression::Expression(expr) = eq.eq_sides[0].node_or_expression.clone() {
        expr.expand();
    }; */
}

static EQUATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    let strings = vec!["1-(-2-3)", "1-(-2-3)/(-4-5)"];
    strings
        .into_iter()
        .map(|string| string.to_string())
        .collect_vec()
});
