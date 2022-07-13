use math_eval::ast::context::Context;

fn main() {
    let mut context = Context::new();

    let _test = context.try_add_equation("4 + 4x + x^2 + 5").unwrap();

    let _instructions = context.try_add_equation("a^2 + 2a*b + b^2").unwrap();

    context.solve();

    println!("{:#?}", context);
    /* test.get_equation(&context).sides[0]
    .element
    .bind(&instructions.get_equation(&context).sides[0].element); */
}
