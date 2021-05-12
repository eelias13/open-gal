use super::*;

struct Lexer {
    data: Vec<String>,
    line_index: usize,
    char_index: usize,
    current_line: String,
    current_char: char,
    tokens: Vec<Token>,
    eof: bool, // end of file
    eol: bool, // end of line by last char
}

impl Lexer {
    pub fn new(vec: Vec<&str>) -> Self {
        // TODO: check if vec is empty
        let data: Vec<String> = vec.iter().map(|&s| format!("{}\n", s)).collect();
        Lexer {
            data: data.clone(),
            line_index: 0,
            char_index: 0,
            current_line: data[0].clone(),
            current_char: data[0].chars().nth(0).unwrap(),
            tokens: Vec::<Token>::new(),
            eof: false,
            eol: false,
        }
    }

    fn next(&mut self) {
        if self.eof {
            // TODO: make lexing error
            panic!("lexer has reached end of file on next char available");
        }

        if self.eol {
            if self.data.len() - 1 == self.line_index {
                self.eof = true;
                self.eol = true;
            } else {
                self.line_index += 1;
                self.char_index = 0;
                self.current_line = self.data[self.line_index].clone();
            }
            return;
        }

        if self.current_line.len() - 2 >= self.char_index {
            self.eol = true;
        }

        self.char_index += 1;
        self.current_char = self.current_line.chars().nth(self.char_index).unwrap();
    }

    /// the main lexer this function lexes everything
    /// ## Arguments
    ///
    /// * `input` - is a the sorce code (as `Vec<&str>`)
    ///
    /// ## Examples
    ///
    /// ```rust
    /// let mut lexer = Lexer::new(vec!["pin in = 2;", "", "&1010 // comment"]);
    /// let input = lexer.lex();
    ///
    /// let output = Token::vec(vec![
    ///     vec![
    ///         TokenType::Pin,
    ///         TokenType::Ignore { comment: None },
    ///         TokenType::Identifier {
    ///             name: "in".to_string(),
    ///         },
    ///         TokenType::Ignore { comment: None },
    ///         TokenType::Equals,
    ///         TokenType::Ignore { comment: None },
    ///         TokenType::Number { value: 2 },
    ///         TokenType::Semicolon,
    ///     ],
    ///     vec![],
    ///     vec![
    ///         TokenType::And,
    ///         TokenType::BoolTable {
    ///             table: vec![true, false, true, false],
    ///         },
    ///         TokenType::Ignore { comment: None },
    ///         TokenType::Ignore {
    ///             comment: Some("// comment".to_string()),
    ///         },
    ///     ],
    /// ]);
    ///
    /// assert_eq!(input.len(), output.len());
    /// for i in 0..input.len() {
    ///     assert_eq!(input[i], output[i], "token <{}>", i);
    /// }
    /// ```
    pub fn lex(&mut self) -> Vec<Token> {
        while !self.eof {
            if self.lex_char() {
                continue;
            }
            if self.lex_num() {
                continue;
            }
            if self.lex_arrow() {
                continue;
            }
            if self.lex_identifier() {
                continue;
            }
            if self.lex_comment() {
                continue;
            }

            let err = ParsingError::new(
                Token {
                    begin_line: self.line_index,
                    begin_char: self.char_index,
                    len_char: 1,
                    len_line: 1,
                    token_type: TokenType::Unknown,
                },
                format!("unexpected character <{}>", self.current_char),
                self.data.clone(),
            );
            err.panic();
        }

        self.tokens.clone()
    }

    fn lex_comment(&mut self) -> bool {
        if self.current_char != '/' {
            return false;
        }

        let mut is_multiline = false;
        let begin_line = self.line_index;
        let begin_char = self.char_index;
        let mut comment = String::new();

        comment.push(self.current_char);
        self.next();

        if !self.eol {
            match self.current_char {
                '*' => is_multiline = true,
                '/' => is_multiline = false,
                _ => {
                    let err = ParsingError::new(
                        Token {
                            begin_line: self.line_index,
                            begin_char: self.char_index,
                            len_char: 1,
                            len_line: 1,
                            token_type: TokenType::Unknown,
                        },
                        format!(
                            "unexpected character expected <*, /> got <{}>",
                            self.current_char
                        ),
                        self.data.clone(),
                    );
                    err.panic();
                    unreachable!();
                }
            };
        } else {
            let err = ParsingError::new(
                Token {
                    begin_line: self.line_index,
                    begin_char: self.char_index,
                    len_char: 1,
                    len_line: 1,
                    token_type: TokenType::Unknown,
                },
                "unexpected line braek expected <*, /> got <new line>".to_string(),
                self.data.clone(),
            );
            err.panic();
            unreachable!();
        }
        comment.push(self.current_char);

        if is_multiline {
            let mut last_star = false;
            loop {
                if !self.eof {
                    if self.current_char == '/' && last_star {
                        comment.push(self.current_char);
                        break;
                    }

                    last_star = self.current_char == '*' && !self.eol;
                    comment.push(self.current_char);
                    self.next();
                } else {
                    let err = ParsingError::new(
                        Token {
                            begin_line: self.line_index,
                            begin_char: self.char_index,
                            len_char: 1,
                            len_line: 1,
                            token_type: TokenType::Unknown,
                        },
                        format!(
                            "unexpected character expected <*, /> got <{}>",
                            self.current_char
                        ),
                        self.data.clone(),
                    );
                    err.panic();
                    unreachable!();
                }
            }
        } else {
            while !self.eol {
                comment.push(self.current_char);
                self.next();
            }
        }

        self.tokens.push(Token {
            begin_line,
            begin_char,
            len_char: comment.len(),
            len_line: self.line_index - begin_line + 1,
            token_type: TokenType::Ignore {
                comment: Some(comment),
            },
        });
        true
    }

    fn lex_identifier(&mut self) -> bool {
        if !Self::is_letter(self.current_char) {
            return false;
        }

        let mut name = String::new();
        let begin_char = self.char_index;

        while !self.eol {
            if Self::is_letter(self.current_char)
                || Self::is_digit(self.current_char)
                || self.current_char == '_'
            {
                name.push(self.current_char);
                self.next()
            } else {
                break;
            }
        }

        let token_type = match name.as_ref() {
            "pin" => TokenType::Pin,
            "table" => TokenType::Table,
            "count" => TokenType::Count,
            "fill" => TokenType::Fill,
            _ => TokenType::Identifier { name: name.clone() },
        };

        self.tokens.push(Token {
            begin_char,
            begin_line: self.line_index,
            len_char: name.len(),
            len_line: 1,
            token_type,
        });

        true
    }

    fn lex_arrow(&mut self) -> bool {
        if self.current_char == '-' {
            self.next();
            if self.current_char == '>' {
                self.tokens.push(Token {
                    begin_line: self.line_index,
                    begin_char: self.char_index - 1,
                    len_char: 2,
                    len_line: 1,
                    token_type: TokenType::Arrow,
                });
                self.next();
                true
            } else {
                let err = ParsingError::new(
                    Token {
                        begin_line: self.line_index,
                        begin_char: self.char_index,
                        len_char: 2,
                        len_line: 1,
                        token_type: TokenType::Unknown,
                    },
                    format!(
                        "unexpected char expected <{}> got <{}>",
                        '>', self.current_char
                    ),
                    self.data.clone(),
                );
                err.panic();
                unreachable!();
            }
        } else {
            true
        }
    }

    fn lex_num(&mut self) -> bool {
        let begin_char = self.char_index;
        let first_char = self.current_char;
        if !Self::is_digit(first_char) {
            return false;
        }

        let mut num_chars = String::new();
        let begin_0 = first_char == '0';
        let mut is_bool = first_char == '0' || first_char == '1';
        self.next();

        loop {
            if Self::is_digit(self.current_char) {
                if !(self.current_char == '1' || self.current_char == '0') {
                    if begin_0 {
                        let err = ParsingError::new(
                            Token {
                                begin_char,
                                begin_line: self.line_index,
                                len_char: num_chars.len(),
                                len_line: 1,
                                token_type: TokenType::BoolTable { table: Vec::new() },
                            },
                            format!("expectet <0, 1> got <{}>", self.current_char),
                            self.data.clone(),
                        );
                        err.panic();
                        unreachable!();
                    }

                    if is_bool {
                        is_bool = false;
                    }
                }
                num_chars.push(self.current_char);
                self.next();
            } else {
                break;
            }
        }

        if is_bool {
            self.tokens.push(Token {
                begin_char,
                begin_line: self.line_index,
                len_char: num_chars.len(),
                len_line: 1,
                token_type: TokenType::BoolTable {
                    table: num_chars.chars().map(|c| c == '1').collect(),
                },
            });
            true
        } else {
            let mut num_str = String::new();
            num_chars.chars().for_each(|c| num_str.push(c));

            let result: Result<isize, _> = num_str.parse();

            if result.is_err() {
                let err = ParsingError::new(
                    Token {
                        begin_char,
                        begin_line: self.line_index,
                        len_char: num_chars.len(),
                        len_line: 1,
                        token_type: TokenType::Number { value: 0 },
                    },
                    format!(
                        "parsing error while parsing number expectet <[0-9]> got <{}>",
                        num_str
                    ),
                    self.data.clone(),
                );
                err.panic();
                unreachable!();
            }

            self.tokens.push(Token {
                begin_char,
                begin_line: self.line_index,
                len_char: num_chars.len(),
                len_line: 1,
                token_type: TokenType::Number {
                    value: num_str.parse().unwrap(),
                },
            });
            true
        }
    }

    fn lex_char(&mut self) -> bool {
        let token_type_option = match self.current_char {
            AND => Some(TokenType::And),
            OR => Some(TokenType::Or),
            XOR => Some(TokenType::Xor),
            NOT => Some(TokenType::Not),

            '(' => Some(TokenType::RoundOpen),
            ')' => Some(TokenType::RoundClose),
            '{' => Some(TokenType::CurlyOpen),
            '}' => Some(TokenType::CurlyClose),
            '[' => Some(TokenType::SquareOpen),
            ']' => Some(TokenType::SquareClose),

            ',' => Some(TokenType::Comma),
            ';' => Some(TokenType::Semicolon),
            '=' => Some(TokenType::Equals),
            '.' => Some(TokenType::Dot),

            ' ' => Some(TokenType::Ignore { comment: None }),
            '\t' => Some(TokenType::Ignore { comment: None }),
            '\n' => Some(TokenType::Ignore { comment: None }),

            _ => None,
        };

        if let Some(token_type) = token_type_option {
            self.tokens.push(Token {
                begin_line: self.line_index,
                len_char: 1,
                len_line: 1,
                begin_char: self.char_index,
                token_type,
            });
            self.next();
            true
        } else {
            false
        }
    }

    fn is_letter(c: char) -> bool {
        for l in "AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz".chars() {
            if l == c {
                return true;
            }
        }
        return false;
    }

    fn is_digit(c: char) -> bool {
        for l in "0123456789".chars() {
            if l == c {
                return true;
            }
        }
        return false;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn chars() {
        let mut lexer = Lexer::new(vec!["&|", "^^!&", "([", "{ ; }])", ".,==.,"]);
        let input = lexer.lex();
        let output = Token::vec(vec![
            vec![TokenType::And, TokenType::Or],
            vec![
                TokenType::Xor,
                TokenType::Xor,
                TokenType::Not,
                TokenType::And,
            ],
            vec![TokenType::RoundOpen, TokenType::SquareOpen],
            vec![
                TokenType::CurlyOpen,
                TokenType::Ignore { comment: None },
                TokenType::Semicolon,
                TokenType::Ignore { comment: None },
                TokenType::CurlyClose,
                TokenType::SquareClose,
                TokenType::RoundClose,
            ],
            vec![
                TokenType::Dot,
                TokenType::Comma,
                TokenType::Equals,
                TokenType::Equals,
                TokenType::Dot,
                TokenType::Comma,
            ],
        ]);

        //assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i], "at token <{}>", i);
        }
    }

    #[test]
    #[ignore]
    fn comments() {
        let mut lexer = Lexer::new(vec![
            "// one line comment",
            "",
            "/*",
            "multi line comment",
            "*/",
        ]);
        let input = lexer.lex();

        let mut output = Vec::<Token>::new();

        output.push(Token {
            begin_line: 0,
            begin_char: 0,
            len_char: "// one line comment".len(),
            len_line: 1,
            token_type: TokenType::Ignore {
                comment: Some("// one line comment".to_string()),
            },
        });

        output.push(Token {
            begin_line: 2,
            begin_char: 0,
            len_char: "/*multi line comment*/".len(),
            len_line: 3,
            token_type: TokenType::Ignore {
                comment: Some("/*multi line comment*/".to_string()),
            },
        });

        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i]);
        }
    }

    #[test]
    #[ignore]
    fn doc_example() {
        let mut lexer = Lexer::new(vec!["pin in = 2;", "", "&1010 // comment"]);
        let input = lexer.lex();

        let output = Token::vec(vec![
            vec![
                TokenType::Pin,
                TokenType::Ignore { comment: None },
                TokenType::Identifier {
                    name: "in".to_string(),
                },
                TokenType::Ignore { comment: None },
                TokenType::Equals,
                TokenType::Ignore { comment: None },
                TokenType::Number { value: 2 },
                TokenType::Semicolon,
            ],
            vec![],
            vec![
                TokenType::And,
                TokenType::BoolTable {
                    table: vec![true, false, true, false],
                },
                TokenType::Ignore { comment: None },
                TokenType::Ignore {
                    comment: Some("// comment".to_string()),
                },
            ],
        ]);

        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i], "at token <{}>", i);
        }
    }

    #[test]
    #[ignore]
    fn other() {
        let mut lexer = Lexer::new(vec![
            "123 010",
            "102 2 349645",
            "ab ab3",
            "c_f_g ->",
            "1010",
            "pin table fill.count",
        ]);
        let input = lexer.lex();

        let output = Token::vec(vec![
            vec![
                TokenType::Number { value: 123 },
                TokenType::Ignore { comment: None },
                TokenType::BoolTable {
                    table: vec![false, true, false],
                },
            ],
            vec![
                TokenType::Number { value: 102 },
                TokenType::Ignore { comment: None },
                TokenType::Number { value: 2 },
                TokenType::Ignore { comment: None },
                TokenType::Number { value: 349645 },
            ],
            vec![
                TokenType::Identifier {
                    name: "ab".to_string(),
                },
                TokenType::Ignore { comment: None },
                TokenType::Identifier {
                    name: "ab3".to_string(),
                },
            ],
            vec![
                TokenType::Identifier {
                    name: "c_f_g".to_string(),
                },
                TokenType::Ignore { comment: None },
                TokenType::Arrow,
            ],
            vec![TokenType::BoolTable {
                table: vec![true, false, true, false],
            }],
            vec![
                TokenType::Pin,
                TokenType::Ignore { comment: None },
                TokenType::Table,
                TokenType::Ignore { comment: None },
                TokenType::Fill,
                TokenType::Dot,
                TokenType::Count,
            ],
        ]);

        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i], "at token <{}>", i);
        }
    }
}

/*
pin 13 = i0;
pin 11 = i1;
pin 17 = and;
pin 18 = or;
pin 19 = xor;

table(i0, i1 -> and) {
    00 0
    01 0
    10 0
    11 1
}

table(i0, i1 -> xor ).count {
    0
    1
    1
    0
}

table(i0, i1 -> or).fill(1) {
    00 0
    01 1
    10 1
}

pin 23 = a;
pin 3 = b;
pin 2 = c;

a = (!b | (c));
a.dff;

*/

/*
pin 1, 2 = i[0..1];
pin [13..18] = and, or, xor, not;
table(i0, i1 -> and).fill(0) {
    11 1
}

table(i0, i1 -> or).fill(1) {
    00 0
}

table(i0, i1 -> xor ).count {
    0
    1
    1
    0
}

table(i0 -> not) {
    01
    10
}
*/
