use crate::constants::NUM_FIRST;
use crate::parser::atom::{Atom, AtomType, TableType};
use crate::parser::error::ParsingError;
use crate::parser::token::{Token, TokenType};

pub struct Atomizer {
    data: Vec<String>,
    tokens: Vec<Token>,
    index: usize,
    atoms: Vec<Atom>,
    current_token: Token,
    is_eof: bool,
}

impl Atomizer {
    pub fn new(data: Vec<String>, tokens: Vec<Token>) -> Self {
        Atomizer {
            data,
            tokens: tokens.clone(),
            index: 0,
            atoms: Vec::<Atom>::new(),
            current_token: tokens[0].clone(),
            is_eof: false,
        }
    }

    pub fn atomize(&mut self) -> Result<Vec<Atom>, ParsingError> {
        while !self.is_eof {
            match self.current() {
                TokenType::Pin => self.atomize_pin()?,
                TokenType::Table => self.atomize_table()?,
                TokenType::Identifier { name: _ } => self.atomize_identifiers()?,
                _ => {
                    self.expect_multible(vec![
                        TokenType::Pin,
                        TokenType::Table,
                        TokenType::Identifier {
                            name: String::new(),
                        },
                    ])?;
                }
            }
        }

        Ok(self.atoms.clone())
    }

    fn next(&mut self) {
        if self.index == self.tokens.len() - 1 {
            self.is_eof = true;
            self.current_token = Token {
                begin_char: self.tokens[self.index].begin_char(),
                len_char: self.tokens[self.index].len_char(),
                begin_line: self.tokens[self.index].begin_line(),
                len_line: self.tokens[self.index].len_line(),
                token_type: TokenType::Unknown,
            };
        } else {
            self.index += 1;
            self.current_token = self.tokens[self.index].clone();
        }

        match self.current_token.token_type() {
            TokenType::Ignore { comment: _ } => {
                self.next();
            }
            _ => {}
        }
    }

    fn current(&self) -> TokenType {
        self.current_token.token_type()
    }

    fn parse_bool(&mut self) -> Result<bool, ParsingError> {
        match self.current() {
            TokenType::BoolTable { table } => {
                if table.len() != 1 {
                    // TODO make error
                    let err = ParsingError::from_token(
                        self.current_token.clone(),
                        format!("expected <1> boolean got <{}>", table.len()),
                        self.data.clone(),
                    );
                    return Err(err);
                } else {
                    Ok(table[0])
                }
            }
            _ => {
                self.expect(TokenType::BoolTable { table: Vec::new() })?;
                unreachable!();
            }
        }
    }

    fn expect_multible(&mut self, token_type: Vec<TokenType>) -> Result<(), ParsingError> {
        if let Some(err) =
            ParsingError::expect_tokens(&self.current_token, token_type, self.data.clone())
        {
            Err(err)
        } else {
            self.next();
            Ok(())
        }
    }

    fn expect(&mut self, token_type: TokenType) -> Result<(), ParsingError> {
        self.expect_multible(vec![token_type])?;
        Ok(())
    }

    fn parse_identifiers(&mut self) -> Result<Vec<String>, ParsingError> {
        let mut result = Vec::<String>::new();
        let first = self.get_identifier()?;

        if self.current() == TokenType::SquareOpen {
            let nums = self.parse_num()?;
            for i in nums {
                result.push(format!("{}{}", first, i));
            }
        } else {
            result.push(first);
            while self.current() == TokenType::Comma {
                self.expect(TokenType::Comma)?;
                result.push(self.get_identifier()?);
            }
        }

        Ok(result)
    }

    fn get_identifier(&mut self) -> Result<String, ParsingError> {
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
                })?;
                unreachable!();
            }
        };
        self.next();
        Ok(result)
    }

    fn get_num(&mut self) -> Result<u64, ParsingError> {
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
                self.expect(TokenType::Number { value: 0 })?;
                unreachable!();
            }
        };
        self.next();
        Ok(result)
    }

    fn parse_num(&mut self) -> Result<Vec<u64>, ParsingError> {
        let mut result = Vec::<u64>::new();
        if self.current() == TokenType::SquareOpen {
            self.expect(TokenType::SquareOpen)?;
            let start = self.get_num()?;
            self.expect(TokenType::Dot)?;
            self.expect(TokenType::Dot)?;
            let end = self.get_num()?;
            self.expect(TokenType::SquareClose)?;
            if start == end {
                // TODO make error
                todo!();
            }

            for i in start..(end + 1) {
                result.push(i);
            }
        } else {
            let first = self.get_num()?;

            result.push(first);
            while self.current() == TokenType::Comma {
                self.expect(TokenType::Comma)?;
                let num = self.get_num()?;
                result.push(num);
            }
        }
        Ok(result)
    }

    fn pars_table(&mut self) -> Result<Vec<bool>, ParsingError> {
        let mut result = Vec::new();

        match self.current() {
            TokenType::BoolTable { table: _ } => (),
            _ => {
                self.expect(TokenType::BoolTable { table: Vec::new() })?;
            }
        };

        while let Some(table) = match self.current() {
            TokenType::BoolTable { table } => Some(table),
            _ => None,
        } {
            table.iter().for_each(|v| result.push(v.clone()));
            self.expect(TokenType::BoolTable { table: Vec::new() })?;
        }

        Ok(result)
    }

    fn atomize_pin(&mut self) -> Result<(), ParsingError> {
        let begin_token = self.index;

        self.expect(TokenType::Pin)?;

        let pins;
        let names;
        if NUM_FIRST {
            pins = self.parse_num()?;
            self.expect(TokenType::Equals)?;

            names = self.parse_identifiers()?;
        } else {
            names = self.parse_identifiers()?;
            self.expect(TokenType::Equals)?;
            pins = self.parse_num()?;
        }

        self.expect(TokenType::Semicolon)?;

        let len_token = self.index - begin_token;
        let tokens = &self.tokens;
        let atom_type = AtomType::Pin { names, pins };

        self.atoms
            .push(Atom::new(tokens, begin_token, len_token, atom_type));

        Ok(())
    }

    fn atomize_table(&mut self) -> Result<(), ParsingError> {
        let begin_token = self.index;

        self.expect(TokenType::Table)?;
        self.expect(TokenType::RoundOpen)?;
        let in_names = self.parse_identifiers()?;
        self.expect(TokenType::Arrow)?;
        let out_names = self.parse_identifiers()?;
        self.expect(TokenType::RoundClose)?;

        let table_type;
        if self.current() == TokenType::Dot {
            self.expect(TokenType::Dot)?;

            table_type = match self.current() {
                TokenType::Count => {
                    self.expect(TokenType::Count)?;
                    TableType::Count
                }
                TokenType::Fill => {
                    self.expect(TokenType::Fill)?;
                    self.expect(TokenType::RoundOpen)?;
                    let value = self.parse_bool()?;
                    self.next();
                    self.expect(TokenType::RoundClose)?;

                    TableType::Fill { value }
                }
                _ => {
                    self.expect_multible(vec![TokenType::Count, TokenType::Fill])?;
                    unreachable!();
                }
            };
        } else {
            table_type = TableType::Full;
        }

        self.expect(TokenType::CurlyOpen)?;
        let table = self.pars_table()?;
        self.expect(TokenType::CurlyClose)?;

        let len_token = self.index - begin_token;
        let tokens = &self.tokens;
        let atom_type = AtomType::Table {
            in_names,
            out_names,
            table,
            table_type,
        };

        self.atoms
            .push(Atom::new(tokens, begin_token, len_token, atom_type));

        Ok(())
    }

    fn atomize_identifiers(&mut self) -> Result<(), ParsingError> {
        let begin_token = self.index;
        let names = self.parse_identifiers()?;

        // parse dff
        if self.current() == TokenType::Dot {
            self.expect(TokenType::Dot)?;
            self.expect(TokenType::Dff)?;

            self.expect(TokenType::Semicolon)?;
            let len_token = self.index - begin_token;
            let tokens = &self.tokens;

            self.atoms.push(Atom::new(
                tokens,
                begin_token,
                len_token,
                AtomType::Dff { names },
            ));
        } else {
            // parse bool function

            self.expect(TokenType::Equals)?;
            let func = self.parse_func()?;

            self.expect(TokenType::Semicolon)?;
            let len_token = self.index - begin_token;
            let tokens = &self.tokens;

            self.atoms.push(Atom::new(
                tokens,
                begin_token,
                len_token,
                AtomType::BoolFunc {
                    in_names: names,
                    func,
                },
            ));
        }

        Ok(())
    }

    fn parse_func(&mut self) -> Result<Vec<bool_func_parser::Token>, ParsingError> {
        let mut result = Vec::new();

        // increments on '(' and decrements on ')' should never be -1. Exampel: (a) & b ) is invalid
        let mut count_parentheses = 0;
        // counts all binary operator (and, or, xor) and cheks if ther are enough identifiers. Exampel: !a & & b is invalid
        let mut count_binary = 0;
        // afer an identifier ther must be an operator. Exampel: a a & b is invalid
        let mut last_identifier = false;

        let mut count_identifier = 0;

        while self.current() != TokenType::Semicolon {
            match self.current() {
                TokenType::Or => {
                    result.push(bool_func_parser::Token::Or);
                    count_binary += 1;
                    last_identifier = false;
                }
                TokenType::Xor => {
                    result.push(bool_func_parser::Token::Xor);
                    count_binary += 1;
                    last_identifier = false;
                }
                TokenType::And => {
                    result.push(bool_func_parser::Token::And);
                    count_binary += 1;
                    last_identifier = false;
                }
                TokenType::Not => {
                    result.push(bool_func_parser::Token::Not);
                }
                TokenType::RoundOpen => {
                    result.push(bool_func_parser::Token::Open);
                    count_parentheses += 1;
                }
                TokenType::RoundClose => {
                    result.push(bool_func_parser::Token::Close);
                    if count_parentheses == 0 {
                        // TODO make error
                        unreachable!();
                    }
                    count_parentheses -= 1;
                }
                TokenType::Identifier { name } => {
                    result.push(bool_func_parser::Token::Var { name });
                    if last_identifier {
                        // TODO make error
                        unreachable!();
                    }
                    last_identifier = true;
                    count_identifier += 1;
                }

                TokenType::BoolTable { table: _ } => {
                    if self.parse_bool()? {
                        result.push(bool_func_parser::Token::One)
                    } else {
                        result.push(bool_func_parser::Token::Zero)
                    }
                    if last_identifier {
                        // TODO make error
                        unreachable!();
                    }
                    last_identifier = true;
                    count_identifier += 1;
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
                    ])?;
                }
            }
            self.next();
        }

        if count_binary != count_identifier - 1 {
            // TODO make error syntaxError(expression.at(expression.size() - 1), Token::Type::identifier);
            unreachable!();
        }
        if count_parentheses != 0 {
            // TODO make error
            unreachable!();
        }
        Ok(result)
    }
}

// ---------------------------------------------------------------------- Tests ----------------------------------------------------------------------

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

        let mut atomizer = Atomizer::new(data, tokens);

        let input = atomizer.parse_func().unwrap();
        // (a|b&d|(c^!1))
        let output: Vec<bool_func_parser::Token> = vec![
            bool_func_parser::Token::Open,
            bool_func_parser::Token::Var {
                name: "a".to_string(),
            },
            bool_func_parser::Token::Or,
            bool_func_parser::Token::Var {
                name: "b".to_string(),
            },
            bool_func_parser::Token::And,
            bool_func_parser::Token::Var {
                name: "d".to_string(),
            },
            bool_func_parser::Token::Or,
            bool_func_parser::Token::Open,
            bool_func_parser::Token::Var {
                name: "c".to_string(),
            },
            bool_func_parser::Token::Xor,
            bool_func_parser::Token::Not,
            bool_func_parser::Token::One,
            bool_func_parser::Token::Close,
            bool_func_parser::Token::Close,
        ];

        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i], "at {}", i);
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
        let mut atomizer = Atomizer::new(data, tokens.clone());

        let input = atomizer.atomize().unwrap();
        let output = vec![
            Atom::new(
                &tokens,
                0,
                10,
                AtomType::BoolFunc {
                    in_names: vec!["a".to_string()],
                    func: vec![
                        bool_func_parser::Token::Var {
                            name: "b".to_string(),
                        },
                        bool_func_parser::Token::And,
                        bool_func_parser::Token::Var {
                            name: "c".to_string(),
                        },
                    ],
                },
            ),
            Atom::new(
                &tokens,
                10,
                4,
                AtomType::Dff {
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

        let mut atomizer = Atomizer::new(data, tokens);

        let mut input = atomizer.parse_identifiers().unwrap();
        let mut output = vec!["a", "b", "c"];
        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i]);
        }

        input = atomizer.parse_identifiers().unwrap();
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

        let mut atomizer = Atomizer::new(data, tokens);

        let mut input = atomizer.parse_num().unwrap();
        let mut output = vec![1, 2, 3, 10];

        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i]);
        }

        input = atomizer.parse_num().unwrap();
        output = vec![0, 1, 2, 3];
        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i]);
        }
    }

    #[test]
    fn test_pin() {
        let data = vec!["pin 3 = a;\n".to_string()];
        let tokens = Token::vec(vec![vec![
            TokenType::Pin,
            TokenType::Ignore { comment: None },
            TokenType::Number { value: 3 },
            TokenType::Ignore { comment: None },
            TokenType::Equals,
            TokenType::Ignore { comment: None },
            TokenType::Identifier {
                name: "a".to_string(),
            },
            TokenType::Semicolon,
            TokenType::Ignore { comment: None },
        ]]);

        let mut atomizer = Atomizer::new(data, tokens.clone());
        let input = atomizer.atomize().unwrap();
        assert_eq!(input.len(), 1);
        assert_eq!(
            input[0],
            Atom::new(
                &tokens,
                0,
                8,
                AtomType::Pin {
                    pins: vec![3],
                    names: vec!["a".to_string()]
                },
            )
        );
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
                    name: "i1".to_string(),
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

        let mut atomizer = Atomizer::new(data, tokens.clone());
        let input = atomizer.atomize().unwrap();
        assert_eq!(input.len(), 1);
        assert_eq!(
            input[0],
            Atom::new(
                &tokens,
                0,
                tokens.len() - 1,
                AtomType::Table {
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
}
