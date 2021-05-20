use super::*;

pub fn test() {
    let data = vec!["pin 3 = a;\n".to_string()];
    let tokens = Token::vec(vec![vec![
        TokenType::Pin,
        TokenType::Ignore { comment: None },
        TokenType::Number { value: 3 },
        TokenType::Ignore { comment: None },
        TokenType::Identifier {
            name: "a".to_string(),
        },
        TokenType::Semicolon,
        TokenType::Ignore { comment: None },
    ]]);

    let mut syntax_analyser = SyntaxAnalyser::new(data, tokens.clone());
    let input = syntax_analyser.analys();
    assert_eq!(input.len(), 1);
    assert_eq!(
        input[0],
        Sentence::new(
            &tokens,
            0,
            6,
            SentenceType::Pin {
                pins: vec![3],
                names: vec!["a".to_string()]
            },
        )
    );
}

pub struct SyntaxAnalyser {
    data: Vec<String>,
    tokens: Vec<Token>,
    index: usize,
    sentences: Vec<Sentence>,
    current_token: Token,
    is_eof: bool,
}

impl SyntaxAnalyser {
    pub fn new(data: Vec<String>, tokens: Vec<Token>) -> Self {
        SyntaxAnalyser {
            data,
            tokens: tokens.clone(),
            index: 0,
            sentences: Vec::<Sentence>::new(),
            current_token: tokens[0].clone(),
            is_eof: false,
        }
    }

    pub fn analys(&mut self) -> Vec<Sentence> {
        while !self.is_eof {
            match self.current() {
                TokenType::Pin => self.analysis_pin(),
                TokenType::Table => self.analysis_table(),
                TokenType::Identifier { name: _ } => self.analysis_identifiers(),
                _ => {
                    self.expect_multible(vec![
                        TokenType::Pin,
                        TokenType::Table,
                        TokenType::Identifier {
                            name: String::new(),
                        },
                    ]);
                    unreachable!();
                }
            }
        }

        self.sentences.clone()
    }

    fn next(&mut self) {
        if self.index == self.tokens.len() - 1 {
            self.is_eof = true;
            self.current_token = Token {
                begin_char: self.tokens[self.index].begin_char,
                len_char: self.tokens[self.index].len_char,
                begin_line: self.tokens[self.index].begin_line,
                len_line: self.tokens[self.index].len_line,
                token_type: TokenType::Unknown,
            };
        } else {
            self.index += 1;
            self.current_token = self.tokens[self.index].clone();
        }

        match self.current_token.token_type {
            TokenType::Ignore { comment: _ } => {
                self.next();
            }
            _ => {}
        }
    }

    fn current(&self) -> TokenType {
        self.current_token.token_type.clone()
    }

    fn parse_bool(&mut self) -> bool {
        match self.current() {
            TokenType::BoolTable { table } => {
                if table.len() != 1 {
                    // TODO make error
                    ParsingError::from_token(
                        self.current_token.clone(),
                        format!("expected <1> boolean got <{}>", table.len()),
                        self.data.clone(),
                    );
                    unreachable!();
                } else {
                    self.next();
                    table[0]
                }
            }
            _ => {
                self.expect(TokenType::BoolTable { table: Vec::new() });
                unreachable!();
            }
        }
    }

    fn expect_multible(&mut self, token_type: Vec<TokenType>) {
        if let Some(err) =
            ParsingError::expect_tokens(&self.current_token, token_type, self.data.clone())
        {
            err.panic();
        }
        self.next();
    }

    fn expect(&mut self, token_type: TokenType) {
        self.expect_multible(vec![token_type]);
    }

    fn parse_identifiers(&mut self) -> Vec<String> {
        let mut result = Vec::<String>::new();
        let first = self.get_identifier();

        if self.current() == TokenType::SquareOpen {
            let nums = self.parse_num();
            for i in nums {
                result.push(format!("{}{}", first, i));
            }
        } else {
            result.push(first);
            while self.current() == TokenType::Comma {
                self.expect(TokenType::Comma);
                result.push(self.get_identifier());
            }
        }

        result
    }

    fn get_identifier(&mut self) -> String {
        let result = match self.current() {
            TokenType::Pin => "pin".to_string(),
            TokenType::Table => "table".to_string(),
            TokenType::Dff => "dff".to_string(),
            TokenType::Count => "count".to_string(),
            TokenType::Fill => "fill".to_string(),
            TokenType::Identifier { name } => name,
            _ => {
                self.expect(TokenType::Identifier {
                    name: String::new(),
                });
                unreachable!();
            }
        };
        self.next();
        result
    }

    fn get_num(&mut self) -> u64 {
        let result = match self.current() {
            TokenType::Number { value } => value,
            TokenType::BoolTable { table } => {
                // convert bool vec to u64
                let mut result = 0;

                for (i, &val) in table.iter().enumerate() {
                    if val {
                        result += u64::pow(10, (table.len() - i - 1) as u32);
                    }
                }

                result
            }
            _ => {
                self.expect(TokenType::Number { value: 0 });
                unreachable!();
            }
        };
        self.next();
        result
    }

    fn parse_num(&mut self) -> Vec<u64> {
        let mut result = Vec::<u64>::new();
        if self.current() == TokenType::SquareOpen {
            self.expect(TokenType::SquareOpen);
            let start = self.get_num();
            self.expect(TokenType::Dot);
            self.expect(TokenType::Dot);
            let end = self.get_num();
            self.expect(TokenType::SquareClose);
            if start == end {
                // TODO make error
                unreachable!();
            }

            for i in start..(end + 1) {
                result.push(i);
            }
        } else {
            let first = self.get_num();

            result.push(first);
            while self.current() == TokenType::Comma {
                self.expect(TokenType::Comma);
                result.push(self.get_num());
            }
        }
        result
    }

    fn pars_table(&mut self) -> Vec<bool> {
        let mut result = Vec::<bool>::new();

        match self.current() {
            TokenType::BoolTable { table: _ } => (),
            _ => {
                self.expect(TokenType::BoolTable { table: Vec::new() });
                unreachable!();
            }
        };

        while let Some(table) = match self.current() {
            TokenType::BoolTable { table } => Some(table),
            _ => None,
        } {
            table.iter().for_each(|v| result.push(v.clone()));
            self.expect(TokenType::BoolTable { table: Vec::new() });
        }

        result
    }

    fn analysis_pin(&mut self) {
        let begin_token = self.index;

        self.expect(TokenType::Pin);

        let pins;
        let names;
        if NUM_FIRST {
            pins = self.parse_num();
            println!("{:?}", self.current());
            self.expect(TokenType::Equals);

            names = self.parse_identifiers();
        } else {
            names = self.parse_identifiers();
            self.expect(TokenType::Equals);
            pins = self.parse_num();
        }

        self.expect(TokenType::Semicolon);

        let len_token = begin_token - self.index;
        let tokens = &self.tokens;
        let sentence_type = SentenceType::Pin { names, pins };

        self.sentences
            .push(Sentence::new(tokens, begin_token, len_token, sentence_type));
    }

    fn analysis_table(&mut self) {
        let begin_token = self.index;

        self.expect(TokenType::Table);
        self.expect(TokenType::RoundOpen);
        let in_names = self.parse_identifiers();
        self.expect(TokenType::Arrow);
        let out_names = self.parse_identifiers();
        self.expect(TokenType::RoundClose);

        let table_type;
        if self.current() == TokenType::Dot {
            self.expect(TokenType::Dot);

            table_type = match self.current() {
                TokenType::Count => {
                    self.expect(TokenType::Count);
                    TableType::Count
                }
                TokenType::Fill => {
                    self.expect(TokenType::Fill);
                    self.expect(TokenType::RoundOpen);
                    let value = self.parse_bool();
                    self.expect(TokenType::RoundClose);

                    TableType::Fill { value }
                }
                _ => {
                    self.expect_multible(vec![TokenType::Count, TokenType::Fill]);
                    unreachable!();
                }
            };
        } else {
            table_type = TableType::Full;
        }

        self.expect(TokenType::CurlyOpen);
        let table = self.pars_table();
        self.expect(TokenType::CurlyClose);

        let len_token = begin_token - self.index;
        let tokens = &self.tokens;
        let sentence_type = SentenceType::Table {
            in_names,
            out_names,
            table,
            table_type,
        };

        self.sentences
            .push(Sentence::new(tokens, begin_token, len_token, sentence_type));
    }

    fn analysis_identifiers(&mut self) {
        let begin_token = self.index;
        let names = self.parse_identifiers();

        // parse dff
        if self.current() == TokenType::Dot {
            self.expect(TokenType::Dot);
            self.expect(TokenType::Dff);

            self.expect(TokenType::Semicolon);
            let len_token = self.index - begin_token;
            let tokens = &self.tokens;

            self.sentences.push(Sentence::new(
                tokens,
                begin_token,
                len_token,
                SentenceType::Dff { names },
            ));
        } else {
            // parse bool function

            self.expect(TokenType::Equals);
            let func = self.parse_func();

            self.expect(TokenType::Semicolon);
            let len_token = self.index - begin_token;
            let tokens = &self.tokens;

            self.sentences.push(Sentence::new(
                tokens,
                begin_token,
                len_token,
                SentenceType::BoolFunc {
                    in_names: names,
                    rpn_func: func,
                },
            ));
        }
    }

    fn parse_func(&mut self) -> Vec<BoolFunc> {
        let mut operators = Vec::<TokenType>::new();
        let mut rpn = Vec::<TokenType>::new();

        while self.current() != TokenType::Semicolon {
            match self.current() {
                TokenType::Or => {
                    while match operators.last() {
                        Some(TokenType::Xor) => true,
                        Some(TokenType::And) => true,
                        Some(TokenType::Not) => true,
                        _ => false,
                    } {
                        if let Some(operator) = operators.pop() {
                            rpn.push(operator);
                        }
                    }
                    operators.push(self.current());
                    self.next();
                }
                TokenType::Xor => {
                    while match operators.last() {
                        Some(TokenType::And) => true,
                        Some(TokenType::Not) => true,
                        _ => false,
                    } {
                        if let Some(operator) = operators.pop() {
                            rpn.push(operator);
                        }
                    }
                    operators.push(self.current());
                    self.next();
                }

                TokenType::And => {
                    while match operators.last() {
                        Some(TokenType::Not) => true,
                        _ => false,
                    } {
                        if let Some(operator) = operators.pop() {
                            rpn.push(operator);
                        }
                    }
                    operators.push(self.current());
                    self.next();
                }
                TokenType::Not => {
                    operators.push(self.current());
                    self.next();
                }
                TokenType::RoundOpen => {
                    operators.push(self.current());
                    self.next();
                }
                TokenType::RoundClose => {
                    while operators.last() != Some(&TokenType::RoundOpen) {
                        if let Some(operator) = operators.pop() {
                            rpn.push(operator);
                        } else {
                            // TODO add error
                            unreachable!();
                        }
                    }
                    operators.pop();
                    self.next();
                }
                TokenType::Identifier { name: _ } => {
                    rpn.push(self.current());
                    self.next();
                }
                TokenType::BoolTable { table: _ } => {
                    rpn.push(self.current());
                    let _ = self.parse_bool();
                }
                _ => {
                    self.expect_multible(vec![
                        TokenType::Or,
                        TokenType::Xor,
                        TokenType::And,
                        TokenType::Not,
                        TokenType::Identifier {
                            name: String::new(),
                        },
                        TokenType::BoolTable { table: Vec::new() },
                    ]);
                    unreachable!();
                }
            }
        }
        while let Some(o) = operators.pop() {
            rpn.push(o);
        }

        let mut result = Vec::<BoolFunc>::new();
        for t in rpn {
            result.push(match t {
                TokenType::Or => BoolFunc::Or,
                TokenType::Xor => BoolFunc::Xor,
                TokenType::And => BoolFunc::And,
                TokenType::Not => BoolFunc::Not,
                TokenType::Identifier { name } => BoolFunc::Var { name },
                TokenType::BoolTable { table } => {
                    // has only one value this is assured by `let _ = self.parse_bool();` line above
                    if table[0] {
                        BoolFunc::One
                    } else {
                        BoolFunc::Zero
                    }
                }
                _ => {
                    self.expect_multible(vec![
                        TokenType::Or,
                        TokenType::Xor,
                        TokenType::And,
                        TokenType::Not,
                        TokenType::Identifier {
                            name: String::new(),
                        },
                        TokenType::BoolTable { table: Vec::new() },
                    ]);
                    unreachable!();
                }
            });
        }
        result
    }
}

// ------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------- test ---------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_func() {
        // data is only for error, it does not influence the test
        let data = vec!["(a|b&d|(c^!1));".to_string()];
        let tokens = Token::vec(vec![vec![
            TokenType::RoundOpen,
            TokenType::Identifier {
                name: "a".to_string(),
            },
            TokenType::Or,
            TokenType::Identifier {
                name: "b".to_string(),
            },
            TokenType::And,
            TokenType::Identifier {
                name: "d".to_string(),
            },
            TokenType::Or,
            TokenType::RoundOpen,
            TokenType::Identifier {
                name: "c".to_string(),
            },
            TokenType::Xor,
            TokenType::Not,
            TokenType::BoolTable { table: vec![true] },
            TokenType::RoundClose,
            TokenType::RoundClose,
            TokenType::Semicolon,
        ]]);

        let mut syntax_analyser = SyntaxAnalyser::new(data, tokens);

        let input = syntax_analyser.parse_func();
        // otuput in reverse polish notation `abcd&c1!^||`
        let output = vec![
            BoolFunc::Var {
                name: "a".to_string(),
            },
            BoolFunc::Var {
                name: "b".to_string(),
            },
            BoolFunc::Var {
                name: "d".to_string(),
            },
            BoolFunc::And,
            BoolFunc::Var {
                name: "c".to_string(),
            },
            BoolFunc::One,
            BoolFunc::Not,
            BoolFunc::Xor,
            BoolFunc::Or,
            BoolFunc::Or,
        ];

        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i]);
        }
    }

    #[test]
    fn test_identifier() {
        // data is only for error, it does not influence the test
        let data = vec!["a = b & c;\n".to_string(), "a.dff;\n".to_string()];
        let tokens = Token::vec(vec![
            vec![
                TokenType::Identifier {
                    name: "a".to_string(),
                },
                TokenType::Ignore { comment: None },
                TokenType::Equals,
                TokenType::Ignore { comment: None },
                TokenType::Identifier {
                    name: "b".to_string(),
                },
                TokenType::Ignore { comment: None },
                TokenType::And,
                TokenType::Identifier {
                    name: "c".to_string(),
                },
                TokenType::Semicolon,
                TokenType::Ignore { comment: None },
            ],
            vec![
                TokenType::Identifier {
                    name: "a".to_string(),
                },
                TokenType::Dot,
                TokenType::Dff,
                TokenType::Semicolon,
                TokenType::Ignore { comment: None },
            ],
        ]);
        let mut syntax_analyser = SyntaxAnalyser::new(data, tokens.clone());

        let input = syntax_analyser.analys();
        let output = vec![
            Sentence::new(
                &tokens,
                0,
                10,
                SentenceType::BoolFunc {
                    in_names: vec!["a".to_string()],
                    rpn_func: vec![
                        BoolFunc::Var {
                            name: "b".to_string(),
                        },
                        BoolFunc::Var {
                            name: "c".to_string(),
                        },
                        BoolFunc::And,
                    ],
                },
            ),
            Sentence::new(
                &tokens,
                10,
                4,
                SentenceType::Dff {
                    names: vec!["a".to_string()],
                },
            ),
        ];

        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i]);
        }
    }

    #[test]
    fn test_parse_identifier() {
        // data is only for error, it does not influence the test
        let data = vec!["a, b, c".to_string(), "pin[0..3]".to_string()];
        let tokens = Token::vec(vec![
            vec![
                TokenType::Identifier {
                    name: "a".to_string(),
                },
                TokenType::Comma,
                TokenType::Ignore { comment: None },
                TokenType::Identifier {
                    name: "b".to_string(),
                },
                TokenType::Comma,
                TokenType::Ignore { comment: None },
                TokenType::Identifier {
                    name: "c".to_string(),
                },
            ],
            vec![
                TokenType::Pin,
                TokenType::SquareOpen,
                TokenType::BoolTable { table: vec![false] },
                TokenType::Dot,
                TokenType::Dot,
                TokenType::Number { value: 3 },
                TokenType::SquareClose,
            ],
        ]);

        let mut syntax_analyser = SyntaxAnalyser::new(data, tokens);

        let mut input = syntax_analyser.parse_identifiers();
        let mut output = vec!["a", "b", "c"];
        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i]);
        }

        input = syntax_analyser.parse_identifiers();
        output = vec!["pin0", "pin1", "pin2", "pin3"];
        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i]);
        }
    }

    #[test]
    fn test_parse_num() {
        // data is only for error, it does not influence the test
        let data = vec!["1, 2, 3, 10".to_string(), "[0..3]".to_string()];
        let tokens = Token::vec(vec![
            vec![
                TokenType::BoolTable { table: vec![true] },
                TokenType::Comma,
                TokenType::Ignore { comment: None },
                TokenType::Number { value: 2 },
                TokenType::Comma,
                TokenType::Ignore { comment: None },
                TokenType::Number { value: 3 },
                TokenType::Comma,
                TokenType::Ignore { comment: None },
                TokenType::BoolTable {
                    table: vec![true, false],
                },
            ],
            vec![
                TokenType::SquareOpen,
                TokenType::BoolTable { table: vec![false] },
                TokenType::Dot,
                TokenType::Dot,
                TokenType::Number { value: 3 },
                TokenType::SquareClose,
            ],
        ]);

        let mut syntax_analyser = SyntaxAnalyser::new(data, tokens);

        let mut input = syntax_analyser.parse_num();
        let mut output = vec![1, 2, 3, 10];

        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i]);
        }

        input = syntax_analyser.parse_num();
        output = vec![0, 1, 2, 3];
        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i]);
        }
    }

    #[test]
    fn test_table() {
        let data = vec![
            "table(i0, i1 -> and) {",
            "    00 0",
            "    01 0",
            "    10 0",
            "    11 1",
            "}",
        ]
        .iter()
        .map(|s| format!("{}\n", s))
        .collect();

        let tokens = Token::vec(vec![
            vec![
                TokenType::Table,
                TokenType::RoundOpen,
                TokenType::Identifier {
                    name: "i0".to_string(),
                },
                TokenType::Comma,
                TokenType::Ignore { comment: None },
                TokenType::Identifier {
                    name: "b".to_string(),
                },
                TokenType::Ignore { comment: None },
                TokenType::Arrow,
                TokenType::Ignore { comment: None },
                TokenType::Identifier {
                    name: "and".to_string(),
                },
                TokenType::RoundClose,
                TokenType::Ignore { comment: None },
                TokenType::CurlyOpen,
                TokenType::Ignore { comment: None },
            ],
            vec![
                TokenType::Ignore { comment: None },
                TokenType::BoolTable {
                    table: vec![false, false],
                },
                TokenType::Ignore { comment: None },
                TokenType::BoolTable { table: vec![false] },
                TokenType::Ignore { comment: None },
            ],
            vec![
                TokenType::Ignore { comment: None },
                TokenType::BoolTable {
                    table: vec![false, true],
                },
                TokenType::Ignore { comment: None },
                TokenType::BoolTable { table: vec![false] },
                TokenType::Ignore { comment: None },
            ],
            vec![
                TokenType::Ignore { comment: None },
                TokenType::BoolTable {
                    table: vec![true, false],
                },
                TokenType::Ignore { comment: None },
                TokenType::BoolTable { table: vec![false] },
                TokenType::Ignore { comment: None },
            ],
            vec![
                TokenType::Ignore { comment: None },
                TokenType::BoolTable {
                    table: vec![true, true],
                },
                TokenType::Ignore { comment: None },
                TokenType::BoolTable { table: vec![true] },
                TokenType::Ignore { comment: None },
            ],
            vec![
                TokenType::Ignore { comment: None },
                TokenType::CurlyClose,
                TokenType::Ignore { comment: None },
            ],
        ]);

        let mut syntax_analyser = SyntaxAnalyser::new(data, tokens.clone());
        let input = syntax_analyser.analys();
        assert_eq!(input.len(), 1);
        assert_eq!(
            input[0],
            Sentence::new(
                &tokens,
                0,
                tokens.len() - 1,
                SentenceType::Table {
                    in_names: vec!["i0".to_string(), "i1".to_string()],
                    out_names: vec!["and".to_string()],
                    table: vec![
                        false, false, false, false, true, false, true, false, false, true, true,
                        true
                    ],
                    table_type: TableType::Full,
                },
            )
        );
    }

    #[test]
    fn test_pin() {}
}
