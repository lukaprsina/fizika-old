use std::rc::Rc;

use color_eyre::eyre::Result;
use math_eval::{
    actions::{
        bind::Bind,
        is_same::{IsSame, IsSameNames},
    },
    ast::{app::App, context::Context, Element},
    initialize,
};
use tracing::info;

// TODO: vec remove unwrap
fn main() -> Result<()> {
    initialize()?;
    // info!("Started the logger crate");

    let app = App::new()?;

    let context = Context::new(Rc::clone(&app));

    let ctx_uuid = app.borrow_mut().add_context(context);

    // let a = "(2 + a)/cos(x)";
    // let a = "2/a";
    // let b = "(1/cos(x) + a/cos(x))";

    // let a = "1/7 * a * (2 - a) / 2 * (b + 4) * 4";
    // let a = "log(2+3+4)";

    // let a = "2log(a+c)+b";
    // let b = "b+2log(c+b)";
    let a = "sin(x) + 4";
    let b = "(sin(x)) + 2";

    let uuid1 = App::try_add_equation(Rc::clone(&app), ctx_uuid, a)?;
    let uuid2 = App::try_add_equation(Rc::clone(&app), ctx_uuid, b)?;

    {
        let mut borrowed_app = app.borrow_mut();
        // let ctx_uuid = borrowed_app.formulas;
        let ctx = borrowed_app.get_context_mut(ctx_uuid).unwrap();

        let eq1 = ctx.remove_equation(uuid1).unwrap();
        let eq2 = ctx.remove_equation(uuid2).unwrap();

        println!("{:#?}", eq1);
        // info!(%eq1);
        // info!(%eq2);

        let elem1 = eq1.equation_sides.first().unwrap();
        let elem2 = eq2.equation_sides.first().unwrap();

        elem1.bind(&elem2);

        let mut names = IsSameNames::new();
        let is_same = Element::is_same(&elem1, &elem2, &mut names);
        println!("{:#?}\n", names);
        info!("check: {}, is_same: {}", names.check(), is_same);
    }

    Ok(())
}
