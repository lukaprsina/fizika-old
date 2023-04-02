use std::rc::Rc;

use color_eyre::eyre::Result;
use itertools::Itertools;
use math_eval::ast::{app::App, context::Context};
use once_cell::sync::Lazy;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

// TODO: vec remove unwrap
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
        /* let eq = ctx.remove_equation(uuid1).unwrap();
        do_stuff(eq); */
    }

    {
        let mut borrowed_app = app.borrow_mut();
        let context = borrowed_app.get_context_mut(ctx_uuid).unwrap();

        context.solve();
    }

    for uuid in uuids {
        let mut borrowed_app = app.borrow_mut();
        let ctx = borrowed_app.get_context_mut(ctx_uuid).unwrap();
        let eq = ctx.remove_equation(uuid).unwrap();
        do_stuff(eq);
    }

    Ok(())
}

#[allow(dead_code, unused_variables)]
fn do_stuff(eq: math_eval::ast::Equation) {
    // eq.to_string();
    println!("{eq:#?}");
    // println!("{eq}");
    /* if let NodeOrExpression::Expression(expr) = eq.eq_sides[0].node_or_expression.clone() {
        expr.expand();
    }; */
}

static EQUATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    // TODO: expression is not cached to the top
    let strings = vec![
        "a*(b+c)", // "f(g(h, x+2))",
                  // "f(a, b^2)=3x",
    ];
    strings
        .into_iter()
        .map(|string| string.to_string())
        .collect_vec()
});
