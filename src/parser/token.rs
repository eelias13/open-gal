use crate::constants::*;

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
    pub begin_char: usize,
    pub len_char: usize,
    pub begin_line: usize,
    pub len_line: usize,
    pub token_type: TokenType,
}

impl Token {
    pub fn begin_char(&self) -> usize {
        self.begin_char
    }

    pub fn len_char(&self) -> usize {
        self.len_char
    }

    pub fn begin_line(&self) -> usize {
        self.begin_line
    }

    pub fn len_line(&self) -> usize {
        self.len_line
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type.clone()
    }
}

#[cfg(test)]
impl Token {
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
