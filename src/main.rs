use math_eval::{ast::Equation, tokenizer::parser::TokenizedString};

fn main() {
    let cases = vec![
        "1*2*3", "1+(2+3)", "1(2+3)",
        "(1*2)*3",
        // "1(2+3)",
        // "1(2*3)",
        // "1^(1+1)",
        // "(1+1)^1",
        // "(1+1)^(1+1)^(1+1)",
        /* "ceil(sin(60 + 1, 2) + 1)",
        "f(2x+1,y) + 1",
        "2x+1",
        "1/x + 2^(x * 6 * 2/(a + b)) - 3^2 + 1/(2 + x)", // ignores exponent bracket
        "(1\t)",
        "1*  m", // can be an unit
        "(2+x)^2",
        "a/b",
        "(1)a",
        "a",
        "1(\n b+c)", // ignores bracket
        "a*b",
        "674(374c-4)=40329464",
        "a+c//b",
        "1a",
        "a=b",
        "a<=b=c",
        "1/(2 + x)", */
    ];

    for case in cases {
        let tokens = TokenizedString::try_new(case);
        println!("Case: {}\n", case);

        if let Ok(tokens) = tokens {
            // println!("{:#?}\n", &tokens);
            let ast = Equation::try_from(tokens);
            println!("{:#?}\n", &ast);
            if let Ok(equation) = ast {
                println!("Converted back:\n{}\n", &equation);
            }
        } else {
            println!("Error: {:?}", tokens);
        }
        println!("{}", "-".repeat(80));
    }
}
