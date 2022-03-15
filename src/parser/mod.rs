mod test;

use crate::TableData;
use hardware_sim::LookupTable;
use logos::Logos;
use std::collections::HashMap;
use tokenizer::{Error, Tokenizer, TypeEq};

pub fn parse(code: &str) -> Result<Vec<TableData>, Error> {
    let o_gal = OGal::parse(code)?;
    ogal2td(o_gal)
}
#[derive(PartialEq, Debug, Clone)]
pub struct OGal {
    pins: HashMap<String, usize>,
    lut: Vec<LookupTable>,
    dff: Vec<String>,
}

impl OGal {
    pub fn new(pins: Vec<(&str, usize)>, lut: Vec<LookupTable>, dff: Vec<&str>) -> Self {
        let mut pin_map = HashMap::new();
        for (name, pin) in pins {
            pin_map.insert(name.to_string(), pin);
        }

        Self {
            pins: pin_map,
            lut,
            dff: dff.iter().map(|&s| s.to_string()).collect(),
        }
    }

    pub fn parse(code: &str) -> Result<Self, Error> {
        let mut pins = HashMap::new();
        let mut lut = Vec::new();
        let mut dff = Vec::new();

        let mut tokenizer = Tokenizer::new(Token::lexer(code), vec![Token::Ignore((0, None))]);

        while let Some(token) = tokenizer.peek() {
            match token {
                Token::Pin => {
                    pin(&mut tokenizer, &mut pins)?;
                }
                Token::Table => table(&mut tokenizer, &mut lut)?,
                Token::Identifier(_) => {
                    let names = pin_name(&mut tokenizer)?;
                    if tokenizer.next_is(Token::Dot) {
                        get_dff(names, &mut tokenizer, &mut dff)?;
                    } else {
                        func(names, &mut tokenizer, &mut lut)?;
                    }
                }
                _ => {
                    tokenizer.next();
                    tokenizer.expect_multi(vec![
                        Token::Pin,
                        Token::Table,
                        Token::Identifier(String::new()),
                    ])?;
                    unreachable!();
                }
            }
        }

        Ok(Self { pins, lut, dff })
    }
}

fn pin(tokenizer: &mut Tokenizer<Token>, pins: &mut HashMap<String, usize>) -> Result<(), Error> {
    tokenizer.expect_next(Token::Pin)?;
    let nums = pin_num(tokenizer)?;
    tokenizer.expect_next(Token::Equals)?;
    let names = pin_name(tokenizer)?;

    if nums.len() != names.len() {
        return Err(tokenizer.error(&format!(
            "pin name len ({}) and pin number len ({}) doesn't match",
            names.len(),
            nums.len()
        )));
    }

    for i in 0..nums.len() {
        if let Some(num) = pins.insert(names[i].clone(), nums[i]) {
            return Err(tokenizer.error(&format!(
                "pin {} has been already assigned to {}",
                names[i].clone(),
                num
            )));
        }
    }

    tokenizer.expect_next(Token::Semicolon)?;

    Ok(())
}

fn func(
    names: Vec<String>,
    tokenizer: &mut Tokenizer<Token>,
    lut: &mut Vec<LookupTable>,
) -> Result<(), Error> {
    tokenizer.expect_next(Token::Equals)?;

    let mut func = Vec::new();
    while let Some(token) = tokenizer.next() {
        let bool_token = match token {
            Token::And => bool_algebra::Token::And,
            Token::Or => bool_algebra::Token::Or,
            Token::Xor => bool_algebra::Token::Xor,
            Token::Not => bool_algebra::Token::Not,
            Token::Identifier(name) => bool_algebra::Token::Var(name),
            Token::RoundClose => bool_algebra::Token::Close,
            Token::RoundOpen => bool_algebra::Token::Open,
            Token::Number(num) => {
                if num == "0" {
                    bool_algebra::Token::Zero
                } else if num == "1" {
                    bool_algebra::Token::One
                } else {
                    return Err(tokenizer
                        .error(&format!("unexpected char {} only '0' or '1' allowed", num)));
                }
            }
            Token::Semicolon => break,
            _ => {
                tokenizer.expect(Token::Semicolon)?;
                unreachable!();
            }
        };

        func.push(bool_token);
    }

    let out_table = match bool_algebra::parse(&func) {
        Ok(table) => table,
        Err(msg) => return Err(tokenizer.error(&msg)),
    };

    let mut table = Vec::new();
    for _ in 0..names.len() {
        table.push(out_table.clone());
    }
    let in_names = bool_algebra::get_names(&func);
    let in_names = in_names.iter().map(|s| s.as_ref()).collect();
    let out_names = names.iter().map(|s| s.as_ref()).collect();
    let lt = LookupTable::new(table, in_names, out_names, "").unwrap();

    lut.push(lt);

    Ok(())
}

fn get_dff(
    names: Vec<String>,
    tokenizer: &mut Tokenizer<Token>,
    dff: &mut Vec<String>,
) -> Result<(), Error> {
    tokenizer.expect_next(Token::Dot)?;
    tokenizer.expect_next(Token::Dff)?;
    tokenizer.expect_next(Token::Semicolon)?;

    for name in names {
        dff.push(name);
    }

    Ok(())
}

fn table(tokenizer: &mut Tokenizer<Token>, lut: &mut Vec<LookupTable>) -> Result<(), Error> {
    tokenizer.expect_next(Token::Table)?;
    tokenizer.expect_next(Token::RoundOpen)?;
    let in_names = pin_name(tokenizer)?;
    tokenizer.expect_next(Token::Arrow)?;
    let out_names = pin_name(tokenizer)?;
    tokenizer.expect_next(Token::RoundClose)?;

    enum Fill {
        Fill(bool),
        None,
        Count,
    }

    let fill;

    if tokenizer.next_is(Token::Dot) {
        tokenizer.expect_next(Token::Dot)?;
        if tokenizer.next_is(Token::Count) {
            fill = Fill::Count;

            tokenizer.expect_next(Token::Count)?;
        } else {
            tokenizer.expect_next(Token::Fill)?;
            tokenizer.expect_next(Token::RoundOpen)?;

            let num =
                if let Token::Number(num) = tokenizer.expect_next(Token::Number(String::new()))? {
                    num
                } else {
                    unreachable!()
                };
            tokenizer.expect_next(Token::RoundClose)?;

            if num == "0" {
                fill = Fill::Fill(false);
            } else if num == "1" {
                fill = Fill::Fill(true);
            } else {
                return Err(
                    tokenizer.error(&format!("unexpected char {} only '0' or '1' allowed", num))
                );
            }
        }
    } else {
        fill = Fill::None;
    }

    tokenizer.expect_next(Token::CurlyOpen)?;

    let table = bool_table(tokenizer)?;

    let table = match fill {
        Fill::Count => bool_algebra::parse_count(
            in_names.len(),
            out_names.len(),
            table,
            crate::COUNT_VERTICAL,
        ),
        Fill::Fill(fill) => bool_algebra::parse_fill(in_names.len(), out_names.len(), table, fill),
        Fill::None => bool_algebra::parse_full(in_names.len(), out_names.len(), table),
    };

    let table = match table {
        Ok(table) => table,
        Err(msg) => return Err(tokenizer.error(&msg)),
    };

    let in_names = in_names.iter().map(|s| s.as_ref()).collect();
    let out_names = out_names.iter().map(|s| s.as_ref()).collect();
    let lt = LookupTable::new(table, in_names, out_names, "").unwrap();
    lut.push(lt);

    tokenizer.expect_next(Token::CurlyClose)?;

    Ok(())
}

fn pin_num(tokenizer: &mut Tokenizer<Token>) -> Result<Vec<usize>, Error> {
    if tokenizer.next_is(Token::SquareOpen) {
        tokenizer.expect_next(Token::SquareOpen)?;
        let start = get_num(tokenizer)?;
        tokenizer.expect_next(Token::Dot)?;
        tokenizer.expect_next(Token::Dot)?;
        let end = get_num(tokenizer)?;

        let mut result = Vec::new();
        for num in start..end + 1 {
            result.push(num);
        }

        tokenizer.expect_next(Token::SquareClose)?;
        Ok(result)
    } else {
        let mut result = Vec::new();

        let num = get_num(tokenizer)?;
        result.push(num);

        while tokenizer.next_is(Token::Comma) {
            tokenizer.expect_next(Token::Comma)?;
            let num = get_num(tokenizer)?;
            result.push(num);
        }

        Ok(result)
    }
}

fn get_num(tokenizer: &mut Tokenizer<Token>) -> Result<usize, Error> {
    if let Token::Number(num) = tokenizer.expect_next(Token::Number(String::new()))? {
        match num.parse() {
            Ok(num) => Ok(num),
            Err(err) => Err(tokenizer.error(&format!("ParseIntError: {}", err))),
        }
    } else {
        unreachable!();
    }
}

fn pin_name(tokenizer: &mut Tokenizer<Token>) -> Result<Vec<String>, Error> {
    let name = get_name(tokenizer)?;
    if tokenizer.next_is(Token::SquareOpen) {
        let mut result = Vec::new();
        let nums = pin_num(tokenizer)?;
        for num in nums {
            result.push(format!("{}{}", name, num));
        }

        Ok(result)
    } else {
        let mut result = Vec::new();
        result.push(name);

        while tokenizer.next_is(Token::Comma) {
            tokenizer.expect_next(Token::Comma)?;
            let name = get_name(tokenizer)?;
            result.push(name);
        }

        Ok(result)
    }
}

fn get_name(tokenizer: &mut Tokenizer<Token>) -> Result<String, Error> {
    if let Token::Identifier(name) = tokenizer.expect_next(Token::Identifier(String::new()))? {
        Ok(name)
    } else {
        unreachable!();
    }
}

fn bool_table(tokenizer: &mut Tokenizer<Token>) -> Result<Vec<bool>, Error> {
    let mut result = Vec::new();

    while tokenizer.next_is(Token::Number(String::new())) {
        if let Token::Number(num) = tokenizer.expect_next(Token::Number(String::new()))? {
            let temp = match get_bool(num) {
                Ok(temp) => temp,
                Err(msg) => return Err(tokenizer.error(&msg)),
            };

            for b in temp {
                result.push(b);
            }
        } else {
            unreachable!();
        }
    }

    Ok(result)
}

fn get_bool(num: String) -> Result<Vec<bool>, String> {
    let mut result = Vec::new();
    for c in num.chars() {
        if c == '0' {
            result.push(false);
        } else if c == '1' {
            result.push(true);
        } else {
            return Err(format!("unexpected char {} only '0' or '1' allowed", c));
        }
    }

    Ok(result)
}

pub fn ogal2td(o_gal: OGal) -> Result<Vec<TableData>, Error> {
    use hardware_sim::Component;
    let mut td_vec = Vec::with_capacity(o_gal.lut.len());

    for lut in o_gal.lut {
        let out_pins = lookup_pins(lut.out_names(), &o_gal.pins)?;
        let dffs = lookup_pins(o_gal.dff.clone(), &o_gal.pins)?;

        for (i, &out_pin) in out_pins.iter().enumerate() {
            let td = TableData::new(
                lookup_pins(lut.in_names(), &o_gal.pins)?,
                out_pin,
                lut.get_table()[i].clone(),
                dffs.contains(&out_pin),
            );
            td_vec.push(td);
        }
    }

    Ok(td_vec)
}

fn lookup_pins(pin_name: Vec<String>, pin_map: &HashMap<String, usize>) -> Result<Vec<u32>, Error> {
    let mut pin_num = Vec::with_capacity(pin_name.len());
    for name in pin_name {
        if let Some(&num) = pin_map.get(&name) {
            pin_num.push(num as u32);
        } else {
            return Err(Error::msg(&format!("pin {} not defined", name)));
        }
    }
    Ok(pin_num)
}

#[derive(PartialEq, Debug, Clone, logos::Logos)]
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

    #[regex(r"[a-zA-Z_][a-zA-Z_0-9]+", |lex| lex.slice().parse())]
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

fn ignore(lexer: &mut logos::Lexer<Token>) -> Option<(usize, Option<String>)> {
    let slice = lexer.slice();
    match slice {
        " " => Some((0, None)),
        "\n" => Some((1, Some("newline".to_string()))),
        "\r\n" => Some((1, Some("newline".to_string()))),
        "\t" => Some((0, None)),
        _ => Some((slice.matches("\n").count(), Some(slice.to_string()))),
    }
}
