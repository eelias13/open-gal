use std::{collections::HashMap, result};

use crate::TableData;
use hardware_sim::LookupTable;
use logos::{Lexer, Logos};
use tokenizer::{Error, Tokenizer, TypeEq};

pub struct OGal {
    pins: HashMap<String, usize>,
    lut: Vec<LookupTable>,
    dff: Vec<String>,
}

impl OGal {
    pub fn new(pins: HashMap<String, usize>, lut: Vec<LookupTable>, dff: Vec<String>) -> Self {
        Self { pins, lut, dff }
    }

    pub fn parse(code: &str) -> Result<Self, Error> {
        let mut pins = HashMap::new();
        let mut lut = Vec::new();
        let mut dff = Vec::new();

        Ok(Self::new(pins, lut, dff))
    }
}

pub fn parse(code: &str) -> Result<Vec<TableData>, Error> {
    let o_gal = OGal::parse(code)?;
    let mut table_data = Vec::new();
    

    Ok(table_data)
}

#[derive(PartialEq, Debug, Clone, Logos)]
pub enum Token {
    #[token("pin")]
    Pin, // pin
    #[token("table")]
    Table, // table
    #[token("count")]
    Count, // count
    #[token("fill")]
    Fill, // fill
    #[token("dff")]
    Dff, //dff

    #[token(",")]
    Comma, // ,
    #[token(";")]
    Semicolon, // ;
    #[token("=")]
    Equals, // =
    #[token(".")]
    Dot, // .

    #[token("&")]
    And, // &
    #[token("|")]
    Or, // |
    #[token("^")]
    Xor, // ^
    #[token("!")]
    Not, // !

    #[token("{")]
    CurlyOpen, // {
    #[token("(")]
    RoundOpen, // (
    #[token("[")]
    SquareOpen, // [
    #[token("}")]
    CurlyClose, // }
    #[token(")")]
    RoundClose, // )
    #[token("]")]
    SquareClose, // ]

    #[token("->")]
    Arrow, // ->

    #[regex(r"[a-zA-Z_$][a-zA-Z_$0-9]+", |lex| lex.slice().parse())]
    #[regex(r"[a-zA-Z]", |lex| lex.slice().parse())]
    Identifier(String),

    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Number(String),

    #[token("\t", ignore)]
    #[token(" ", ignore)]
    #[token("\n", ignore)]
    #[token("\r\n", ignore)]
    #[regex(r"(/\*([^*]|\*[^/])*\*/)|(//[^\r\n]*(\r\n|\n)?)", ignore)]
    Ignore((usize, Option<String>)),

    #[error]
    Unknown,
}

impl TypeEq for Token {
    fn type_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Ignore(_), Token::Ignore(_)) => true,
            (Token::Number(_), Token::Number(_)) => true,
            (Token::Identifier(_), Token::Identifier(_)) => true,
            _ => self == other,
        }
    }
}

fn ignore(lexer: &mut Lexer<Token>) -> Option<(usize, Option<String>)> {
    let slice = lexer.slice();
    match slice {
        " " => Some((0, None)),
        "\n" => Some((1, Some("newline".to_string()))),
        "\r\n" => Some((1, Some("newline".to_string()))),
        "\t" => Some((0, None)),
        _ => Some((slice.matches("\n").count(), Some(slice.to_string()))),
    }
}
