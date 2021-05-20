const AND: char = '&';
const OR: char = '|';
const XOR: char = '^';
const NOT: char = '!';

// when parsing pin the number comes first
// e.g. if NUM_FIRST == true `pin 1 = a;` else `pin a = 1;`
const NUM_FIRST: bool = true;

mod lexer;
mod syntax_analyser;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Number { value: u64 },
    BoolTable { table: Vec<bool> },
    Identifier { name: String },

    Pin,   // pin
    Table, // table
    Count, // count
    Fill,  // fill
    Dff,   //dff

    Comma,     // ,
    Semicolon, // ;
    Equals,    // =
    Dot,       // .

    And, // &
    Or,  // |
    Xor, // ^
    Not, // !

    CurlyOpen,   // {
    RoundOpen,   // (
    SquareOpen,  // [
    CurlyClose,  // }
    RoundClose,  // )
    SquareClose, // ]

    Arrow,                              // ->
    Unknown,                            // ?
    Ignore { comment: Option<String> }, // _
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    begin_char: usize,
    len_char: usize,
    begin_line: usize,
    len_line: usize,
    token_type: TokenType,
}

impl Token {
    pub fn new(
        begin_line: usize,
        begin_char: usize,
        len_char: usize,
        len_line: usize,
        token_type: TokenType,
    ) -> Self {
        Self {
            begin_line,
            begin_char,
            len_char,
            len_line,
            token_type,
        }
    }

    pub fn token_type(self) -> TokenType {
        self.token_type
    }

    pub fn vec(vec2d: Vec<Vec<TokenType>>) -> Vec<Self> {
        let mut result = Vec::new();

        for (begin_line, vec) in vec2d.iter().enumerate() {
            let mut begin_char = 0;
            for token_type in vec {
                let len_char = match token_type {
                    TokenType::Arrow => 2,

                    TokenType::Pin => 3,
                    TokenType::Fill => 4,
                    TokenType::Count => 5,
                    TokenType::Table => 5,
                    TokenType::Dff => 3,

                    TokenType::Number { value } => value.clone().to_string().len(),
                    TokenType::BoolTable { table } => table.len(),
                    TokenType::Identifier { name } => name.len(),

                    TokenType::Ignore { comment } => {
                        if let Some(c) = comment {
                            c.len()
                        } else {
                            1
                        }
                    }

                    _ => 1,
                };

                result.push(Self {
                    begin_line,
                    begin_char,
                    len_char,
                    len_line: 1,
                    token_type: token_type.clone(),
                });

                begin_char += len_char;
            }
        }
        result
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.token_type {
            TokenType::BoolTable { table } => write!(f, "{:?}", table),
            TokenType::Identifier { name } => write!(f, "{}", name),
            TokenType::Number { value } => write!(f, "{}", value),

            TokenType::Pin => write!(f, "pin"),
            TokenType::Table => write!(f, "tabel"),
            TokenType::Count => write!(f, "count"),
            TokenType::Fill => write!(f, "fill"),
            TokenType::Dff => write!(f, "dff"),

            TokenType::And => write!(f, "{}", AND),
            TokenType::Or => write!(f, "{}", OR),
            TokenType::Xor => write!(f, "{}", XOR),
            TokenType::Not => write!(f, "{}", NOT),

            TokenType::CurlyOpen => write!(f, "{}", "{"),
            TokenType::RoundOpen => write!(f, "{}", "("),
            TokenType::SquareOpen => write!(f, "{}", "["),

            TokenType::CurlyClose => write!(f, "{}", "}"),
            TokenType::RoundClose => write!(f, "{}", ")"),
            TokenType::SquareClose => write!(f, "{}", "]"),

            TokenType::Comma => write!(f, ","),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Equals => write!(f, "="),
            TokenType::Dot => write!(f, "."),

            TokenType::Arrow => write!(f, "->"),
            TokenType::Unknown => write!(f, "?"),
            TokenType::Ignore { comment } => {
                if let Some(c) = comment {
                    write!(f, "{}", c)
                } else {
                    write!(f, "_")
                }
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TableType {
    Fill { value: bool },
    Full,
    Count,
}

#[derive(PartialEq, Debug, Clone)]
pub enum BoolFunc {
    And,
    Or,
    Xor,
    Not,
    Var { name: String },
    One,
    Zero,
}

#[derive(PartialEq, Debug, Clone)]
pub enum SentenceType {
    Pin {
        pins: Vec<u64>,
        names: Vec<String>,
    },
    Table {
        in_names: Vec<String>,
        out_names: Vec<String>,
        table: Vec<bool>,
        table_type: TableType,
    },
    BoolFunc {
        in_names: Vec<String>,
        rpn_func: Vec<BoolFunc>, // function is in reverse polish notation
    },
    Dff {
        names: Vec<String>,
    },
}

#[derive(PartialEq, Debug, Clone)]
pub struct Sentence {
    begin_char: usize,
    len_char: usize,
    len_line: usize,
    begin_line: usize,
    begin_token: usize,
    len_token: usize,
    sentence_type: SentenceType,
}

impl Sentence {
    fn new(
        tokens: &Vec<Token>,
        begin_token: usize,
        len_token: usize,
        sentence_type: SentenceType,
    ) -> Self {
        Self {
            begin_char: tokens[0].begin_char,
            len_char: tokens.into_iter().map(|t| t.len_char).sum(),
            begin_line: tokens[0].len_line,
            len_line: tokens.into_iter().map(|t| t.len_line).sum(),
            begin_token,
            len_token,
            sentence_type,
        }
    }
}

struct ParsingError {
    begin_line: usize,
    begin_char: usize,
    len_char: usize,
    len_line: usize,
    msg: String,
    data: Vec<String>,
}

impl ParsingError {
    fn from_token(token: Token, msg: String, data: Vec<String>) -> Self {
        Self {
            begin_line: token.begin_line,
            begin_char: token.begin_char,
            len_char: token.len_char,
            len_line: token.len_line,
            msg,
            data,
        }
    }

    fn expect_tokens(token: &Token, expect: Vec<TokenType>, data: Vec<String>) -> Option<Self> {
        let got_type = token.clone().token_type();
        for tt in expect.clone() {
            // ther has to be an easyer way can not use equal because of value feald
            match tt {
                TokenType::Identifier { name: _ } => match got_type {
                    TokenType::Identifier { name: _ } => {
                        return None;
                    }
                    _ => {}
                },
                TokenType::BoolTable { table: _ } => match got_type {
                    TokenType::BoolTable { table: _ } => {
                        return None;
                    }
                    _ => {}
                },
                TokenType::Number { value: _ } => match got_type {
                    TokenType::Number { value: _ } => {
                        return None;
                    }
                    _ => {}
                },
                _ => {
                    if tt == got_type {
                        return None;
                    }
                }
            }
        }

        let msg = format!("expected token type {:?}, but got {:?}", expect, got_type);
        Some(Self {
            begin_line: token.begin_line,
            begin_char: token.begin_char,
            len_char: token.len_char,
            len_line: token.len_line,
            msg,
            data,
        })
    }

    fn panic(self) {
        panic!(
            "{} at line <{}> at index <{}>",
            self.msg,
            self.begin_line + 1,
            self.begin_char
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, 3);
    }

    #[test]
    fn easy_gal() {
        let input = vec![
            "pin 13 = i0;",
            "pin 11 = i1;",
            "pin 17 = and;",
            "pin 18 = or;",
            "pin 19 = xor;",
            "",
            "table(i0, i1 -> and) {",
            "    00 0",
            "    01 ",
            "    10 0",
            "    11 1",
            "}",
            "",
            "table(i0, i1 -> xor).count {",
            "    0",
            "    1",
            "    1",
            "    0",
            "}",
            "",
            "table(i0, i1 -> or).fill(1) {",
            "    00 0",
            "    01 1",
            "    10 1",
            "}",
            "",
            "pin 23 = a;",
            "pin 3 = b;",
            "pin 2 = c;",
            "",
            "a = (!b | (c));",
            "a.dff;",
        ];
    }

    #[test]
    fn open_gal() {
        let input = vec![
            "pin 1, 2 = i[0..1];",
            "pin [13..18] = and, or, xor, not;",
            "table(i0, i1 -> and).fill(0) {",
            "    11 1",
            "}",
            "",
            "table(i0, i1 -> or).fill(1) {",
            "    00 0",
            "}",
            "",
            "table(i0, i1 -> xor ).count {",
            "    0",
            "    1",
            "    1",
            "    0",
            "}",
            "",
            "table(i0 -> not) {",
            "    01",
            "    10",
            "}",
        ];
    }
}

fn main() {
    syntax_analyser::test();
    /*
    let data = vec!["pin 1 = a;"];
    let mut lexer = lexer::Lexer::new(data.clone());
    let tokens = lexer.lex();

    let mut syntax_analyser = syntax_analyser::SyntaxAnalyser::new(
        data.clone()
            .iter()
            .map(|&line| format!("{}\n", line))
            .collect(),
        tokens,
    );
    let sentences = syntax_analyser.analys();

    print!("{:?}", sentences);
    */
}
