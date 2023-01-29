use std::{fs, rc::Rc};

use color_eyre::eyre::Result;
use itertools::Itertools;
use math_eval::{
    ast::{
        app::App,
        context::{Context, CreateEquationError},
        NodeOrExpression,
    },
    tokenizer::parser::ParseError,
};
use once_cell::sync::Lazy;
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

    let file = fs::read_to_string("formulas.txt").expect("File formulas.txt not found");
    let lines = file.lines().collect_vec();

    for equation in lines {
        match App::try_add_equation(Rc::clone(&app), ctx_uuid, equation) {
            Ok(uuid) => {
                // print!("=\"{equation}\",");
                println!("{equation}");
                let mut borrowed_app = app.borrow_mut();
                let ctx = borrowed_app.get_context_mut(ctx_uuid).unwrap();

                let eq = ctx.remove_equation(uuid).unwrap();

                multiply(eq);
                // println!("\n");
            }
            Err(err) => match err {
                CreateEquationError::ParseError(ParseError::Empty) => {}
                _ => return Err(err.into()),
            },
        }
    }

    Ok(())
}

fn multiply(eq: math_eval::ast::Equation) {
    println!("{eq}\n");
    /* if let NodeOrExpression::Expression(expr) = eq.eq_sides[0].node_or_expression.clone() {
        // expr.expand();
        println!("{expr:#?}");
        println!("{expr}");
    }; */
}
