use math_eval::{
    expression::{ast::NodeOrExpressionOrEquation, token_to_rpn::ReversePolishNotation},
    tokenizer::parser::TokenizedString,
};

fn main() {
    loop {
        // read a line into string case
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        println!("Case: {}\n", input);

        let tokens = TokenizedString::try_new(input.as_str());

        if let Ok(tokens) = tokens {
            println!("Tokens:\n{:#?}\n", &tokens);
            if let Ok(rpn) = ReversePolishNotation::try_from(tokens.clone()) {
                println!("Reverse Polish notation:\n{:#?}\n", &rpn);
                let expr = NodeOrExpressionOrEquation::from(rpn);
                println!("Expression:\n{:#?}\n", &expr);
                println!("Converted back:\n{}", expr);
            } else {
                println!("Reverse Polish notation failed");
            }
        } else {
            println!("Error: {:?}", tokens);
        }
        println!("{}", "-".repeat(80));
    }
}
