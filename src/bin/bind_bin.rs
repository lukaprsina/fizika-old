use std::rc::Rc;

use math_eval::ast::context::Context;

fn main() {
    let context = Context::new();

    Context::try_add_equation(Rc::clone(&context), "4 + 4x + x^2 + 5").unwrap();

    Context::try_add_equation(Rc::clone(&context), "a^2 + 2a*b + b^2").unwrap();

    context.borrow().solve();
}
