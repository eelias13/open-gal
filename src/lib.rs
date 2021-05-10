#[allow(dead_code)]

const AND: char = '&';
const OR: char = '|';
const XOR: char = '^';
const NOT: char = '!';

#[derive(PartialEq, Debug, Clone)]
enum OperatorType {
    And,
    Or,
    Xor,
    Not,
}

impl std::fmt::Display for OperatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorType::And => write!(f, "{}", AND),
            OperatorType::Or => write!(f, "{}", OR),
            OperatorType::Xor => write!(f, "{}", XOR),
            OperatorType::Not => write!(f, "{}", NOT),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum KeywordType {
    Pin,   // pin
    Table, // table
    Count, // count
    Fill,  // fill
}

impl std::fmt::Display for KeywordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeywordType::Pin => write!(f, "pin"),
            KeywordType::Table => write!(f, "tabel"),
            KeywordType::Count => write!(f, "fill"),
            KeywordType::Fill => write!(f, "count"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum SymbolType {
    Comma,     // ,
    Semicolon, // ;
    Equals,    // =
    Arrow,     // =>
    Dot,       // .
}

impl std::fmt::Display for SymbolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolType::Comma => write!(f, ","),
            SymbolType::Semicolon => write!(f, ";"),
            SymbolType::Equals => write!(f, "="),
            SymbolType::Arrow => write!(f, "=>"),
            SymbolType::Dot => write!(f, "."),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum ParenthesesType {
    Curly,  // {}
    Round,  // ()
    Square, // []
}

#[derive(PartialEq, Debug, Clone)]
enum TokenType {
    Number { value: isize },
    Boolean { value: bool },
    Keyword { value: KeywordType },
    Symbol { value: SymbolType },
    Identifier { value: String },
    Operator { value: OperatorType },
    Parentheses { open: bool, value: ParenthesesType },
}

#[derive(PartialEq, Debug, Clone)]
struct Token {
    line: usize,
    begin_char: usize,
    len: usize,
    token_type: TokenType,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.token_type {
            TokenType::Boolean { value } => write!(f, "{}", value),
            TokenType::Identifier { value } => write!(f, "{}", value),
            TokenType::Keyword { value } => write!(f, "{}", value),
            TokenType::Number { value } => write!(f, "{}", value),
            TokenType::Operator { value } => write!(f, "{}", value),
            TokenType::Parentheses { open, value } => {
                if *open {
                    match value {
                        ParenthesesType::Curly => write!(f, "{}", "{"),
                        ParenthesesType::Round => write!(f, "{}", "("),
                        ParenthesesType::Square => write!(f, "{}", "["),
                    }
                } else {
                    match value {
                        ParenthesesType::Curly => write!(f, "{}", "}"),
                        ParenthesesType::Round => write!(f, "{}", ")"),
                        ParenthesesType::Square => write!(f, "{}", "]"),
                    }
                }
            }
            TokenType::Symbol { value } => write!(f, "{}", value),
        }
    }
}

fn lex(input: Vec<&str>) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();

    for (line_index, line) in input.iter().enumerate() {
        let mut skip_char = 0;
        for (char_index, c) in line.chars().enumerate() {
            if skip_char != 0 {
                skip_char -= 1;
                continue;
            }

            if let Some(token) = lex_operator(c, char_index, line_index) {
                output.push(token.clone());
                continue;
            }

            if let Some(token) = lex_parentheses(c, char_index, line_index) {
                output.push(token.clone());
                continue;
            }
        }
    }

    output
}

fn lex_parentheses(input: char, char_index: usize, line_index: usize) -> Option<Token> {
    match input {
        '(' => Some(Token {
            line: line_index,
            begin_char: char_index,
            len: 1,
            token_type: TokenType::Parentheses {
                value: ParenthesesType::Round,
                open: true,
            },
        }),
        ')' => Some(Token {
            line: line_index,
            begin_char: char_index,
            len: 1,
            token_type: TokenType::Parentheses {
                value: ParenthesesType::Round,
                open: false,
            },
        }),
        '{' => Some(Token {
            line: line_index,
            begin_char: char_index,
            len: 1,
            token_type: TokenType::Parentheses {
                value: ParenthesesType::Curly,
                open: true,
            },
        }),
        '}' => Some(Token {
            line: line_index,
            begin_char: char_index,
            len: 1,
            token_type: TokenType::Parentheses {
                value: ParenthesesType::Curly,
                open: false,
            },
        }),
        '[' => Some(Token {
            line: line_index,
            begin_char: char_index,
            len: 1,
            token_type: TokenType::Parentheses {
                value: ParenthesesType::Square,
                open: true,
            },
        }),
        ']' => Some(Token {
            line: line_index,
            begin_char: char_index,
            len: 1,
            token_type: TokenType::Parentheses {
                value: ParenthesesType::Square,
                open: false,
            },
        }),
        _ => None,
    }
}

fn lex_operator(input: char, char_index: usize, line_index: usize) -> Option<Token> {
    match input {
        AND => Some(Token {
            line: line_index,
            begin_char: char_index,
            len: 1,
            token_type: TokenType::Operator {
                value: OperatorType::And,
            },
        }),
        OR => Some(Token {
            line: line_index,
            begin_char: char_index,
            len: 1,
            token_type: TokenType::Operator {
                value: OperatorType::Or,
            },
        }),
        XOR => Some(Token {
            line: line_index,
            begin_char: char_index,
            len: 1,
            token_type: TokenType::Operator {
                value: OperatorType::Xor,
            },
        }),
        NOT => Some(Token {
            line: line_index,
            begin_char: char_index,
            len: 1,
            token_type: TokenType::Operator {
                value: OperatorType::Not,
            },
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_lex_operator() {
        let input = vec!["&|", "^^!&"];
        let output = vec![
            Token {
                line: 0,
                begin_char: 0,
                len: 1,
                token_type: TokenType::Operator {
                    value: OperatorType::And,
                },
            },
            Token {
                line: 0,
                begin_char: 1,
                len: 1,
                token_type: TokenType::Operator {
                    value: OperatorType::Or,
                },
            },
            Token {
                line: 1,
                begin_char: 0,
                len: 1,
                token_type: TokenType::Operator {
                    value: OperatorType::Xor,
                },
            },
            Token {
                line: 1,
                begin_char: 1,
                len: 1,
                token_type: TokenType::Operator {
                    value: OperatorType::Xor,
                },
            },
            Token {
                line: 1,
                begin_char: 2,
                len: 1,
                token_type: TokenType::Operator {
                    value: OperatorType::Not,
                },
            },
            Token {
                line: 1,
                begin_char: 3,
                len: 1,
                token_type: TokenType::Operator {
                    value: OperatorType::And,
                },
            },
        ];

        assert_eq!(lex(input), output);
    }
}

// one line comment

/*
    multi line comment
*/

/*

pin 13 = i0;
pin 11 = i1;
pin 17 = and;
pin 18 = or;
pin 19 = xor;

table(i0, i1 => and) {
    00 0
    01 0
    10 0
    11 1
}

table(i0, i1 => xor ).count {
    0
    1
    1
    0
}

table(i0, i1 => or).fill(1) {
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
table(i0, i1 => and).fill(0) {
    11 1
}

table(i0, i1 => or).fill(1) {
    00 0
}

table(i0, i1 => xor ).count {
    0
    1
    1
    0
}

table(i0 => not) {
    01
    10
}

*/
