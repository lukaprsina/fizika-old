// Case: 1 + 2a + 3

Equation {
    expressions: [
        (
            Expression(
                Expression {
                    products: [
                        Product {
                            sign: Positive,
                            numerator: [
                                Expression(
                                    Expression {
                                        products: [
                                            Product {
                                                sign: Positive,
                                                numerator: [
                                                    Node(
                                                        Number(
                                                            Int(
                                                                1,
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                                denominator: [],
                                            },
                                            Product {
                                                sign: Positive,
                                                numerator: [
                                                    Expression(
                                                        Expression {
                                                            products: [
                                                                Product {
                                                                    sign: Positive,
                                                                    numerator: [
                                                                        Node(
                                                                            Number(
                                                                                Int(
                                                                                    2,
                                                                                ),
                                                                            ),
                                                                        ),
                                                                        Node(
                                                                            Unit(
                                                                                "a",
                                                                            ),
                                                                        ),
                                                                    ],
                                                                    denominator: [],
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                ],
                                                denominator: [],
                                            },
                                        ],
                                    },
                                ),
                            ],
                            denominator: [],
                        },
                        Product {
                            sign: Positive,
                            numerator: [
                                Node(
                                    Number(
                                        Int(
                                            3,
                                        ),
                                    ),
                                ),
                            ],
                            denominator: [],
                        },
                    ],
                },
            ),
            None,
        ),
    ],
},

/* Converted back:
(1 + (2a)) + 3 */