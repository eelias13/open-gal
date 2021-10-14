use crate::parser::atom::Atom;
use crate::parser::token::*;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub struct ParsingError {
    begin_line: usize,
    begin_char: usize,
    len_char: usize,
    len_line: usize,
    msg: String,
    data: Vec<String>,
}

impl ParsingError {
    pub fn from_token(token: Token, msg: String, data: Vec<String>) -> Self {
        Self {
            begin_line: token.begin_line(),
            begin_char: token.begin_char(),
            len_char: token.len_char(),
            len_line: token.len_line(),
            msg,
            data,
        }
    }

    pub fn from_atom(atom: Atom, msg: String, data: Vec<String>) -> Self {
        Self {
            begin_line: atom.begin_line(),
            begin_char: atom.begin_char(),
            len_char: atom.len_char(),
            len_line: atom.len_line(),
            msg,
            data,
        }
    }

    pub fn expect_tokens(token: &Token, expect: Vec<TokenType>, data: Vec<String>) -> Option<Self> {
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
            begin_line: token.begin_line(),
            begin_char: token.begin_char(),
            len_char: token.len_char(),
            len_line: token.len_line(),
            msg,
            data,
        })
    }
}

/*
impl fmt::Debug for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
*/

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut line = String::new();
        for i in self.begin_line..(self.begin_line + self.len_line - 1) {
            println!("begin {} len {}", self.begin_line, self.len_line);
            line.push_str(self.data[i].as_str());
        }
        let mut under = String::new();
        for i in self.begin_char..(self.begin_char + self.len_char) {
            if let Some(c) = self.data[self.begin_line + self.len_line].chars().nth(i) {
                line.push(c);
                under.push('^');
            }
        }
        line.push('\n');
        line.push_str(under.as_str());

        write!(
            f,
            "{} \n{} at line <{}> at index <{}>",
            line,
            self.msg,
            self.begin_line + 1,
            self.begin_char
        )
    }
}
