
New formula: (-1-2)-3
 INFO math_eval::ast::equation: Before flatten (- 1 - 2) - 3
Expression {
    products: [
        Product {
            numerator: [
                Element {
                    sign: Positive,
                    node_or_expression: Expression(
                        Expression {
                            products: [
                                Product {
                                    // loči tudi po side_len
                                    numerator: [
                                        Element {
                                            sign: Negative,
                                            node_or_expression: Node(
                                                Number(
                                                    Int(
                                                        1,
                                                    ),
                                                ),
                                            ),
                                            is_number: true,
                                        },
                                    ],
                                    denominator: [],
                                },
                                Product {
                                    numerator: [
                                        Element {
                                            sign: Negative,
                                            node_or_expression: Node(
                                                Number(
                                                    Int(
                                                        2,
                                                    ),
                                                ),
                                            ),
                                            is_number: true,
                                        },
                                    ],
                                    denominator: [],
                                },
                            ],
                        },
                    ),
                    is_number: false,
                },
            ],
            denominator: [],
        },
        Product {
            numerator: [
                Element {
                    sign: Negative,
                    node_or_expression: Node(
                        Number(
                            Int(
                                3,
                            ),
                        ),
                    ),
                    is_number: true,
                },
            ],
            denominator: [],
        },
    ],
}
 INFO flatten: math_eval::actions::flatten: Flatten: (- 1 - 2) - 3
 INFO flatten:flatten: math_eval::actions::flatten: Flatten: - 1 - 2
 INFO flatten: math_eval::actions::flatten: Flatten result: Polynomial -> - 1 - 2
 INFO flatten: math_eval::actions::flatten: num_elements_in_expr 1
 INFO math_eval::ast::equation: After flatten: - 1 * (- 2) - 3
Expression {
    products: [
        Product {
            numerator: [
                Element {
                    sign: Negative,
                    node_or_expression: Node(
                        Number(
                            Int(
                                1,
                            ),
                        ),
                    ),
                    is_number: true,
                },
                Element {
                    sign: Negative,
                    node_or_expression: Node(
                        Number(
                            Int(
                                2,
                            ),
                        ),
                    ),
                    is_number: true,
                },
            ],
            denominator: [],
        },
        Product {
            numerator: [
                Element {
                    sign: Negative,
                    node_or_expression: Node(
                        Number(
                            Int(
                                3,
                            ),
                        ),
                    ),
                    is_number: true,
                },
            ],
            denominator: [],
        },
    ],
}