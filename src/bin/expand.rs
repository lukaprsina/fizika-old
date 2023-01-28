use std::rc::Rc;

use color_eyre::Result;
use math_eval::{
    ast::{app::App, context::Context, Expression, NodeOrExpression},
    initialize,
};
use tracing::debug;

fn main() -> Result<()> {
    initialize()?;
    let app = App::new()?;
    let context = Context::new(Rc::clone(&app));
    let ctx_uuid = app.borrow_mut().add_context(context);

    let a = "4(2+a)";

    let uuid1 = App::try_add_equation(Rc::clone(&app), ctx_uuid, a)?;

    {
        let mut borrowed_app = app.borrow_mut();
        let ctx = borrowed_app.get_context_mut(ctx_uuid).unwrap();
        let mut eq1 = ctx.remove_equation(uuid1).unwrap();
        let elem1 = eq1.eq_sides.first_mut().unwrap();

        if let NodeOrExpression::Expression(expr1) = &mut elem1.node_or_expression {
            debug!("{}", expr1);
            expr1.expand();
            debug!("{}", expr1);
        }
    }

    Ok(())
}
