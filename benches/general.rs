use criterion::{criterion_group, criterion_main, Criterion};

use math_eval::{
    ast::Equation,
    tokenizer::parser::{ParseError, TokenizedString},
};

fn cases() {
    let cases_string = include_str!("../examples.txt");
    let cases = cases_string
        .split('\n')
        .filter(|&case| !case.is_empty())
        .collect::<Vec<&str>>();

    for case in cases {
        let tokens = TokenizedString::try_new(case);

        match tokens {
            Ok(tokens) => {
                let mut ast = Equation::try_from(tokens).unwrap();
                ast.flatten();
            }
            Err(e) => match e {
                ParseError::Empty => (),
                _ => panic!("Error: {:?}", e),
            },
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cases", |b| b.iter(|| cases()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
