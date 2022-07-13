use math_eval::ast::context::Context;

fn main() {
    let cases_string = include_str!("../examples.txt");
    let cases = cases_string
        .split('\n')
        .filter(|&case| !case.is_empty())
        .collect::<Vec<&str>>();

    let mut context = Context::new();
    for case in cases {
        context.try_add_equation(case).unwrap();
    }

    println!("{:#?}", context);
}
