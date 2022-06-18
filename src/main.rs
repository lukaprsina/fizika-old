use math_eval::{
    ast::Equation,
    tokenizer::parser::{ParseError, TokenizedString},
};

fn main() {
    let cases_string = include_str!("../examples.txt");
    let cases = cases_string
        .split('\n')
        .filter(|&case| !case.is_empty())
        .collect::<Vec<&str>>();

    for case in cases {
        let tokens = TokenizedString::try_new(case);

        match tokens {
            Ok(tokens) => {
                println!("Case: {}\n", case);
                // println!("{:#?}\n", &tokens);
                let mut ast = Equation::try_from(tokens).unwrap();
                println!("\n{:#?}\n", &ast);
                // ptree::print_tree(&ast).expect("Unable to print tree");
                println!("\nNot flattened:\n{}\n", ast);

                ast.flatten();
                // println!("{:#?}\n", &ast);
                // println!("Case:\n{}\n", case);

                println!("\nConverted back:\n{}\n", &ast);
                println!("{}\n", "-".repeat(80));
            }
            Err(e) => match e {
                ParseError::Empty => (),
                _ => println!("Error: {:?}", e),
            },
        }
    }
}
