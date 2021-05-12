const AND: char = '&';
const OR: char = '|';
const XOR: char = '^';
const NOT: char = '!';

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Number { value: u64 },
    BoolTable { table: Vec<bool> },
    Identifier { name: String },

    Pin,   // pin
    Table, // table
    Count, // count
    Fill,  // fill

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
struct Token {
    begin_line: usize,
    begin_char: usize,
    len_char: usize,
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

    /// this is to simplify testing this function creates a list of tokens
    /// it automatically figures out the line number and length
    ///
    /// ## Arguments
    ///
    /// * `vec2d` - vec of vecs that contain token type new vec is a new line
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

struct ParsingError {
    token: Token,
    msg: String,
    data: Vec<String>,
}

impl ParsingError {
    /// creats an error
    /// in order to execut the error call `err.panic()`
    ///
    /// ## Examples
    ///
    /// ```rust
    /// let err = LexingError::new(
    ///     Token {
    ///         begin_line: 0,
    ///         begin_char: 0,
    ///         len_char: 1,
    ///         len_line: 1,
    ///         token_type: TokenType::Unknown,
    ///     },
    ///     format!("unexpected character <{}>", c),
    /// );
    /// ```
    fn new(token: Token, msg: String, data: Vec<String>) -> Self {
        Self { token, msg, data }
    }

    /// panics with the supplied message and gives the line on which the error occurred
    ///
    /// ## Examples
    ///
    /// `err.panic()` => e.g. `unexpected character <$> at line <1>`
    fn panic(self) {
        panic!("{} at line <{}>", self.msg, self.token.begin_line + 1);
    }
}

// mod lexer;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, 3);
    }
}
