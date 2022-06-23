use math_eval::{actions::bind::Bind, ast::context::Context};

fn main() {
    let mut context = Context::new();

    let test = context.add_equation("1 + 2").unwrap();

    let instructions = context.add_equation("3 + 1").unwrap();

    test.get_equation(&context).sides[0]
        .element
        .bind(&instructions.get_equation(&context).sides[0].element);
}
