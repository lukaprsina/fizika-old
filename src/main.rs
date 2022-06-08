use math_eval::{ast::Equation, tokenizer::parser::TokenizedString};

fn main() {
    let cases = vec![
        // "1-(2-3)-4",
        "1*(2-3)-4",
        /* "1-(2*3)-4",
        "1*(2*3)-4",
        "1-(2-3)*4",
        "1*(2-3)*4",
        "1-(2*3)*4",
        "1*(2*3)*4", */
        /* "1^(1+1)",
        "(1+1)^1",
        "(1+1)^(1+1)^(1+1)",
        "ceil(sin(60 + 1, 2) + 1)",
        "f(2x+1,y) + 1",
        "2x+1",
        "1/x + 2^(x * 6 * 2/(a + b)) - 3^2 + 1/(2 + x)", // ignores exponent bracket
        "(1\t)",
        "1*  m", // can be an unit
        "(2+x)^2",
        "a/b",
        "(1)a",
        "a",
        "a*b",
        "674(374c-4)=40329464",
        "a/*\tabc */+c//b",
        "1a",
        "a=b",
        "a<=b=c", */
    ];

    for case in cases {
        let tokens = TokenizedString::try_new(case);
        println!("\nCase: {}\n", case);

        if let Ok(tokens) = tokens {
            // println!("{:#?}\n", &tokens);
            let mut ast = Equation::try_from(tokens).unwrap();
            println!("{:#?}\n", &ast);
            println!("Not flattened:\n{}\n", ast);

            ast.flatten();
            println!("{:#?}\n", &ast);
            // println!("Case:\n{}\n", case);

            println!("\nConverted back:\n{}\n", &ast);
        } else {
            println!("Error: {:?}", tokens);
        }
        println!("{}", "-".repeat(80));
    }
}
