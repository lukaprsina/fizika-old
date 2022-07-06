use math_eval::ast::context::{Context, EquationReference};

fn main() {
    let mut context = Context::new();

    let test: EquationReference = context.try_add_equation("4 + 4x + x^2 + 5").unwrap();

    let instructions = context.try_add_equation("a^2 + 2ab + b^2").unwrap();

    context.solve();
    /* test.get_equation(&context).sides[0]
    .element
    .bind(&instructions.get_equation(&context).sides[0].element); */
}
