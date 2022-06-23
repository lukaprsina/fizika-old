use math_eval::{actions::bind::Bind, ast::context::Context};

fn main() {
    let mut context = Context::new();

    let test = context.add_equation("1 + 3").unwrap();

    let instructions = context.add_equation("1/cos(x) + 3/cos(x)").unwrap();

    for side in test.get_equation(&context).sides.iter() {
        side.element
            .bind(&instructions.get_equation(&context).sides[0].element);
    }
}
