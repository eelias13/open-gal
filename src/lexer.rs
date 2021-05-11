const AND: char = '&';
const OR: char = '|';
const XOR: char = '^';
const NOT: char = '!';

struct LexingError {
    token: Token,
    msg: String,
}

impl LexingError {
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
    fn new(token: Token, msg: String) -> Self {
        Self { token, msg }
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

#[derive(PartialEq, Debug, Clone)]
enum TokenType {
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

#[allow(dead_code)]
impl Token {
    /// this is to simplify testing this function creates a list of tokens
    // it automatically figures out the line number and length
    ///
    /// ## Arguments
    ///
    /// * `vec2d` - vec of vecs that contain token type new vec is a new line
    ///
    /// ## Examples
    ///
    /// ```rust
    /// let input = lex(vec!["pin in = 2;", "","&1010 // comment"]);
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
    fn vec(vec2d: Vec<Vec<TokenType>>) -> Vec<Self> {
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

#[allow(dead_code)]
/// the main lexer this function lexes everything
/// ## Arguments
///
/// * `input` - is a the sorce code (as `Vec<&str>`)
///
/// ## Examples
///
/// ```rust
/// let input = lex(vec!["pin in = 2;", "", "&1010 // comment"]);
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
fn lex(input: Vec<&str>) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();

    let mut skip_char = 0;
    for (line_index, line) in input.iter().enumerate() {
        for (char_index, c) in line.chars().enumerate() {
            if skip_char != 0 {
                skip_char -= 1;
                continue;
            }

            if let Some(token) = lex_char(c, char_index, line_index) {
                output.push(token.clone());
                continue;
            }

            if let Some(token) = lex_num(line, char_index, line_index) {
                output.push(token.clone());
                skip_char = token.len_char - 1;
                continue;
            }

            if let Some(token) = lex_arrow(line, char_index, line_index) {
                output.push(token.clone());
                skip_char = 1;
                continue;
            }

            if let Some(token) = lex_identifier(line, char_index, line_index) {
                output.push(token.clone());
                skip_char = token.len_char - 1;
                continue;
            }

            if let Some(token) = lex_comment(input.clone(), char_index, line_index) {
                output.push(token.clone());
                skip_char = token.len_char - 1;
                continue;
            }

            let err = LexingError::new(
                Token {
                    begin_line: line_index,
                    begin_char: char_index,
                    len_char: 1,
                    len_line: 1,
                    token_type: TokenType::Unknown,
                },
                format!("unexpected character <{}>", c),
            );
            err.panic();
        }
    }

    output
}

/// return None except by comments
///
/// !!! Note: not intendet to use outside of `lex(..)` function
///
/// ## Examples
///
///
/// ```rust
/// lex_comment(vec!["// this is a comment"], 0, 0)
/// ```
/// returns for this input
/// ```rust
/// Some(Token {
///     begin_line: 0,
///     begin_char: 0,
///     len_char: 20,
///     len_line: 1,
///     token_type: TokenType::Ignore {
///         comment: Some("// this is a comment".to_string())
///     },
/// })
/// ```
fn lex_comment(input: Vec<&str>, mut char_index: usize, mut line_index: usize) -> Option<Token> {
    if input[line_index].chars().nth(char_index).unwrap() != '/' {
        return None;
    }

    let mut is_multiline = false;
    let begin_line = line_index;
    let begin_char = char_index;

    if let Some(c) = input[line_index].chars().nth(char_index + 1) {
        match c {
            '*' => is_multiline = true,
            '/' => is_multiline = false,
            _ => {
                let err = LexingError::new(
                    Token {
                        begin_line: line_index,
                        begin_char: char_index,
                        len_char: 1,
                        len_line: 1,
                        token_type: TokenType::Unknown,
                    },
                    format!("unexpected character expected <*, /> got <{}>", c),
                );
                err.panic();
            }
        };
    } else {
        let err = LexingError::new(
            Token {
                begin_line: line_index,
                begin_char: char_index,
                len_char: 1,
                len_line: 1,
                token_type: TokenType::Unknown,
            },
            "unexpected line braek expected <*, /> got <new line>".to_string(),
        );
        err.panic();
    }

    let mut comment = String::new();
    comment.push(input[line_index].chars().nth(char_index).unwrap());
    comment.push(input[line_index].chars().nth(char_index + 1).unwrap());
    char_index += 2;

    if is_multiline {
        let mut last_star = false;
        loop {
            if let Some(c) = input[line_index].chars().nth(char_index) {
                if c == '/' && last_star {
                    comment.push(c);
                    break;
                }

                if c == '*' {
                    last_star = true;
                } else {
                    last_star = false;
                }
                comment.push(c);
                char_index += 1;
            } else {
                if line_index > input.len() {
                    break;
                }
                line_index += 1;
                char_index = 0;
                last_star = false;
            }
        }
    } else {
        while let Some(c) = input[line_index].chars().nth(char_index) {
            comment.push(c);
            char_index += 1;
        }
    }

    Some(Token {
        begin_line,
        begin_char,
        len_char: comment.len(),
        len_line: line_index - begin_line + 1,
        token_type: TokenType::Ignore {
            comment: Some(comment),
        },
    })
}

/// return None except by keyword(`pin`,`fill` etc) and idetifiers
///
/// !!! Note: not intendet to use outside of `lex(..)` function
///
/// ## Examples
///
///
/// ```rust
/// lex_identifier("i_0", 0, 0)
/// ```
/// returns for this input
/// ```rust
/// Some(Token {
///     begin_line: 0,
///     begin_char: 0,
///     len_char: 3,
///     len_line: 1,
///     token_type: TokenType::Identifier { name: "i_0".to_string() },
/// })
/// ```
fn lex_identifier(input: &str, mut char_index: usize, line: usize) -> Option<Token> {
    if !is_letter(input.chars().nth(char_index).unwrap()) {
        return None;
    }

    let mut name = String::new();
    let begin_char = char_index;

    while let Some(c) = input.chars().nth(char_index) {
        if is_letter(c) || is_digit(c) || c == '_' {
            name.push(c);
            char_index += 1;
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

    Some(Token {
        begin_char,
        begin_line: line,
        len_char: name.len(),
        len_line: 1,
        token_type,
    })
}

/// return None except by arrow (`->`)
///
/// !!! Note: not intendet to use outside of `lex(..)` function
///
/// ## Examples
///
///
/// ```rust
/// lex_arrow("->", 0, 0)
/// ```
/// returns for this input
/// ```rust
/// Some(Token {
///     begin_line: 0,
///     begin_char: 0,
///     len_char: 2,
///     len_line: 1,
///     token_type: TokenType::Arrow,
/// })
/// ```
fn lex_arrow(input: &str, char_index: usize, line: usize) -> Option<Token> {
    if input.chars().nth(char_index).unwrap() == '-' {
        if input.chars().nth(char_index + 1).unwrap() == '>' {
            Some(Token {
                begin_line: line,
                begin_char: char_index,
                len_char: 2,
                len_line: 1,
                token_type: TokenType::Arrow,
            })
        } else {
            let err = LexingError::new(
                Token {
                    begin_line: line,
                    begin_char: char_index,
                    len_char: 2,
                    len_line: 1,
                    token_type: TokenType::Unknown,
                },
                format!(
                    "unexpected char expected <{}> got <{}>",
                    '>',
                    input.chars().nth(char_index + 1).unwrap()
                ),
            );
            err.panic();

            None
        }
    } else {
        None
    }
}

/// return None except by boolean table and number
///
/// !!! Note: not intendet to use outside of `lex(..)` function
///
/// ## Examples
///
///
/// ```rust
/// lex_num("123", 0, 0)
/// ```
/// returns for this input
/// ```rust
/// Some(Token {
///     begin_line: 0,
///     begin_char: 0,
///     len_char: 3,
///     len_line: 1,
///     token_type: TokenType::Number { value: 123 },
/// })
/// ```
fn lex_num(input: &str, mut char_index: usize, line: usize) -> Option<Token> {
    let begin_char = char_index;
    let first_char = input.chars().nth(begin_char).unwrap();
    if !is_digit(first_char) {
        return None;
    }

    let mut num_chars: Vec<char> = Vec::new();
    let begin_0 = first_char == '0';
    let mut is_bool = first_char == '0' || first_char == '1';

    loop {
        if let Some(c) = input.chars().nth(char_index) {
            char_index += 1;
            if is_digit(c) {
                if !(c == '1' || c == '0') {
                    if begin_0 {
                        let err = LexingError::new(
                            Token {
                                begin_char,
                                begin_line: line,
                                len_char: num_chars.len(),
                                len_line: 1,
                                token_type: TokenType::BoolTable { table: Vec::new() },
                            },
                            format!("expectet <0, 1> got <{}>", c),
                        );
                        err.panic();
                    }

                    if is_bool {
                        is_bool = false;
                    }
                }
                num_chars.push(c)
            } else {
                break;
            }
        } else {
            break;
        }
    }

    if is_bool {
        Some(Token {
            begin_char,
            begin_line: line,
            len_char: num_chars.len(),
            len_line: 1,
            token_type: TokenType::BoolTable {
                table: num_chars.iter().map(|&c| c == '1').collect(),
            },
        })
    } else {
        let mut num_str = String::new();
        num_chars.iter().for_each(|&c| num_str.push(c));

        let result: Result<isize, _> = num_str.parse();

        if result.is_err() {
            let err = LexingError::new(
                Token {
                    begin_char,
                    begin_line: line,
                    len_char: num_chars.len(),
                    len_line: 1,
                    token_type: TokenType::Number { value: 0 },
                },
                format!(
                    "parsing error while parsing number expectet <[0-9]> got <{}>",
                    num_str
                ),
            );
            err.panic();
        }

        Some(Token {
            begin_char,
            begin_line: line,
            len_char: num_chars.len(),
            len_line: 1,
            token_type: TokenType::Number {
                value: num_str.parse().unwrap(),
            },
        })
    }
}

/// return None except by boolean table and number
///
/// !!! Note: not intendet to use outside of `lex(..)` function
///
/// ## Examples
///
///
/// ```rust
/// lex_num(".", 0, 0)
/// ```
/// returns for this input
/// ```rust
/// Some(Token {
///     begin_line: 0,
///     begin_char: 0,
///     len_char: 1,
///     len_line: 1,
///     token_type: TokenType::Dot,
/// })
/// ```
fn lex_char(input: char, begin_char: usize, line: usize) -> Option<Token> {
    let token_type_option = match input {
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
        Some(Token {
            begin_line: line,
            len_char: 1,
            len_line: 1,
            begin_char,
            token_type,
        })
    } else {
        None
    }
}

/// helper function returs true if c is a letter
///
/// ## Examples
///
///
/// `is_letter('a')` => `true`
///
/// `is_letter('?')` => `false`
///
/// `is_letter('4')` => `false`
fn is_letter(c: char) -> bool {
    for l in "AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz".chars() {
        if l == c {
            return true;
        }
    }
    return false;
}

/// helper function returs true if c is a digit
///
/// ## Examples
///
///
/// `is_letter('4')` => `true`
///
/// `is_letter('a')` => `false`
///
/// `is_letter('?')` => `false`

fn is_digit(c: char) -> bool {
    for l in "0123456789".chars() {
        if l == c {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chars() {
        let input = lex(vec!["&|", "^^!&", "([", "{ ; }])", ".,==.,"]);
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

        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i]);
        }
    }

    #[test]
    fn comments() {
        let input = lex(vec![
            "// one line comment",
            "",
            "/*",
            "multi line comment",
            "*/",
        ]);

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
    fn doc_example() {
        let input = lex(vec!["pin in = 2;", "", "&1010 // comment"]);

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
    fn other() {
        let input = lex(vec![
            "123 010",
            "102 2 349645",
            "ab ab3",
            "c_f_g ->",
            "1010",
            "pin table fill.count",
        ]);
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
