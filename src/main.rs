const AND: char = '&';
const OR: char = '|';
const XOR: char = '?';
const NOT: char = '!';

// when parsing pin the number comes first
// e.g. if NUM_FIRST == true `pin 1 = a;` else `pin a = 1;`
const NUM_FIRST: bool = true;
const COUNT_VERTICAL: bool = false;

mod atomizer;
mod function_parser;
mod lexer;
mod table_parser;

use std::usize;
use std::{collections::HashMap, u32};

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

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub enum BoolFunc {
    And,
    Or,
    Xor,
    Not,
    Var { name: String },
    One,
    Zero,
    Open,
    Close,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AtomType {
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
        func: Vec<BoolFunc>,
    },
    Dff {
        names: Vec<String>,
    },
}

#[derive(PartialEq, Debug, Clone)]
pub struct Atom {
    begin_char: usize,
    len_char: usize,
    len_line: usize,
    begin_line: usize,
    begin_token: usize,
    len_token: usize,
    atom_type: AtomType,
}

impl Atom {
    fn new(tokens: &Vec<Token>, begin_token: usize, len_token: usize, atom_type: AtomType) -> Self {
        Self {
            begin_char: tokens[0].begin_char,
            len_char: tokens.into_iter().map(|t| t.len_char).sum(),
            begin_line: tokens[0].len_line,
            len_line: tokens.into_iter().map(|t| t.len_line).sum(),
            begin_token,
            len_token,
            atom_type,
        }
    }
}

/*
*   This data structure contains following data from processed expressions.
*
*   - "m_InputPins" stores all the input pins which are used in the expression
*   - "m_OutputPin" stores the output pin
*   - "m_Table" contains the truth table for the expression and is used to generate a dnf expression later on
*   - "m_EnableDFlipFlop" holds a boolean which decides if the output pin should have its flip flop turned on.
*/

#[repr(C)]
#[derive(PartialEq, Debug, Clone)]
pub struct TableData {
    input_pins: Vec<u32>,
    output_pin: u32,
    table: Vec<bool>,
    enable_flip_flop: bool,
}

// --------------------------------------------------- Error ---------------------------------------------------
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

    fn from_atom(atom: Atom, msg: String, data: Vec<String>) -> Self {
        Self {
            begin_line: atom.begin_line,
            begin_char: atom.begin_char,
            len_char: atom.len_char,
            len_line: atom.len_line,
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

        panic!(
            "{} \n{} at line <{}> at index <{}>",
            line,
            self.msg,
            self.begin_line + 1,
            self.begin_char
        );
    }
}

// --------------------------------------------------- Parser ---------------------------------------------------

use atomizer::Atomizer;
use lexer::Lexer;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_file(file: &str) -> Vec<String> {
    let mut result = Vec::new();

    for (i, line) in read_lines(file)
        .expect(format!("couldn't open file {}", file).as_str())
        .enumerate()
    {
        result.push(format!(
            "{}\n",
            line.expect(format!("couldn't open line {}", i).as_str())
        ));
    }
    result
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_func(
    in_names: Vec<String>,
    func: Vec<BoolFunc>,
    pin_map: &mut HashMap<String, u32>,
    used_pin: &mut Vec<u32>,
) -> Result<Vec<TableData>, String> {
    let mut result = Vec::new();
    let output_pins = match get_pins(in_names, pin_map, used_pin) {
        Ok(pins) => pins,
        Err(pin_name) => return Err(format!("pin <{}> is not definde", pin_name)),
    };

    for output_pin in output_pins {
        if let Some(table) = function_parser::parse(func.clone()) {
            let in_names = function_parser::get_names(func.clone());
            let input_pins = match get_pins(in_names, pin_map, used_pin) {
                Ok(pins) => pins,
                Err(pin_name) => return Err(format!("pin <{}> is not definde", pin_name)),
            };

            result.push(TableData {
                input_pins,
                output_pin,
                table,
                enable_flip_flop: false,
            });
        } else {
            return Err("can't evaluete boolean expression".to_string());
        }
    }
    Ok(result)
}

fn parse_table(
    in_names: Vec<String>,
    out_names: Vec<String>,
    table: Vec<bool>,
    table_type: TableType,
    pin_map: &mut HashMap<String, u32>,
    used_pin: &mut Vec<u32>,
) -> Result<Vec<TableData>, String> {
    let mut result = Vec::new();
    let table_2d = match table_parser::parse(in_names.len(), out_names.len(), table, table_type) {
        Ok(t) => t,
        Err(msg) => return Err(msg),
    };
    let output_pins = match get_pins(out_names.clone(), pin_map, used_pin) {
        Ok(pins) => pins,
        Err(pin_name) => return Err(format!("pin <{}> is not definde", pin_name)),
    };

    if output_pins.len() != table_2d.len() {
        return Err(format!(
            "table len <{}> dose not match output atguments <{}>",
            table_2d.len(),
            output_pins.len()
        ));
    }

    for i in 0..table_2d.len() {
        let input_pins = match get_pins(in_names.clone(), pin_map, used_pin) {
            Ok(pins) => pins,
            Err(pin_name) => return Err(format!("pin <{}> is not definde", pin_name)),
        };
        result.push(TableData {
            input_pins,
            output_pin: output_pins[i],
            table: table_2d[i].clone(),
            enable_flip_flop: false,
        });
    }
    Ok(result)
}

fn get_pins(
    names: Vec<String>,
    pin_map: &mut HashMap<String, u32>,
    used_pin: &mut Vec<u32>,
) -> Result<Vec<u32>, String> {
    let mut pins = Vec::new();
    for name in names {
        if let Some(&pin) = pin_map.get(&name) {
            pins.push(pin);
            let mut is_used = false;
            for p in used_pin.clone() {
                if p == pin {
                    is_used = true;
                    break;
                }
            }

            if !is_used {
                used_pin.push(pin);
            }
        } else {
            return Err(name);
        }
    }
    Ok(pins)
}

fn set_pins(
    names: Vec<String>,
    pins: Vec<u64>,
    pin_map: &mut HashMap<String, u32>,
) -> Result<(), String> {
    if pins.len() != names.len() {
        return Err(format!(
            "pin len <{}> dose not match name atguments <{}>",
            pins.len(),
            names.len()
        ));
    }
    for i in 0..pins.len() {
        // TODO validate pins
        let mut def_pin = None;
        for (_, pin) in pin_map.clone() {
            if pin == pins[i] as u32 {
                def_pin = Some(pin);
                break;
            }
        }
        if let Some(pin) = def_pin {
            return Err(format!(
                "pin <{}> has been defined previously (with name <{}>)",
                pin,
                names[i].clone()
            ));
        }

        if let Some(name) = pin_map.insert(names[i].clone(), pins[i] as u32) {
            return Err(format!("name <{}> has been defined previously", name));
        }
    }

    Ok(())
}

pub fn parse(data: Vec<String>) -> Vec<TableData> {
    let mut result = Vec::new();

    let mut lexer = Lexer::new(data.clone());
    let tokens = lexer.lex();
    let mut atomizer = Atomizer::new(data.clone(), tokens);
    let atoms = atomizer.atomize();

    let mut pin_map: HashMap<String, u32> = HashMap::new();
    let mut used_pin = Vec::new();
    let mut is_dff = Vec::<u32>::new();

    for atom in atoms {
        match atom.clone().atom_type {
            AtomType::Pin { pins, names } => match set_pins(names, pins, &mut pin_map) {
                Ok(()) => (),
                Err(msg) => {
                    let err = ParsingError::from_atom(atom, msg, data);
                    err.panic();
                    unreachable!()
                }
            },
            AtomType::Table {
                in_names,
                out_names,
                table,
                table_type,
            } => match parse_table(
                in_names,
                out_names,
                table,
                table_type,
                &mut pin_map,
                &mut used_pin,
            ) {
                Ok(table_data) => table_data.iter().for_each(|td| result.push(td.clone())),
                Err(msg) => {
                    let err = ParsingError::from_atom(atom, msg, data);
                    err.panic();
                    unreachable!()
                }
            },
            AtomType::BoolFunc { in_names, func } => {
                match parse_func(in_names, func, &mut pin_map, &mut used_pin) {
                    Ok(table_data) => table_data.iter().for_each(|td| result.push(td.clone())),
                    Err(msg) => {
                        let err = ParsingError::from_atom(atom, msg, data);
                        err.panic();
                        unreachable!()
                    }
                }
            }
            AtomType::Dff { names } => match get_pins(names, &mut pin_map, &mut used_pin) {
                Ok(pins) => pins.iter().for_each(|&p| is_dff.push(p)),
                Err(msg) => {
                    let err = ParsingError::from_atom(atom, msg, data);
                    err.panic();
                    unreachable!()
                }
            },
        };
    }

    for dff in is_dff {
        let mut dff_def = false;
        for tb in &mut result {
            if tb.output_pin == dff {
                dff_def = true;
                tb.enable_flip_flop = true;
                break;
            }
        }
        if !dff_def {
            panic!("dff pin <{}> has alrady been definde", dff);
        }
    }

    result
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
        let data = vec![
            "pin 13 = i0;",
            "pin 11 = i1;",
            "pin 17 = and;",
            "pin 18 = or;",
            "pin 19 = xor;",
            "",
            "table(i0, i1 -> and) {",
            "    00 0",
            "    01 0",
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

        let input = parse(data.iter().map(|l| format!("{}\n", l)).collect());
        let output = vec![
            TableData {
                input_pins: vec![13, 11],
                output_pin: 17,
                table: vec![false, false, false, true],
                enable_flip_flop: false,
            },
            TableData {
                input_pins: vec![13, 11],
                output_pin: 19,
                table: vec![false, true, true, false],
                enable_flip_flop: false,
            },
            TableData {
                input_pins: vec![13, 11],
                output_pin: 18,
                table: vec![false, true, true, true],
                enable_flip_flop: false,
            },
            TableData {
                input_pins: vec![3, 2],
                output_pin: 23,
                table: vec![true, true, false, true],
                enable_flip_flop: true,
            },
        ];

        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i], "at {}", i);
        }
    }

    #[test]
    fn open_gal() {
        let data = vec![
            "pin 1, 2 = i[0..1];",
            "pin [13..16] = and, or, xor, not;",
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
        let input = parse(data.iter().map(|l| format!("{}\n", l)).collect());
        let output = vec![
            TableData {
                input_pins: vec![1, 2],
                output_pin: 13,
                table: vec![false, false, false, true],
                enable_flip_flop: false,
            },
            TableData {
                input_pins: vec![1, 2],
                output_pin: 14,
                table: vec![false, true, true, true],
                enable_flip_flop: false,
            },
            TableData {
                input_pins: vec![1, 2],
                output_pin: 15,
                table: vec![false, true, true, false],
                enable_flip_flop: false,
            },
            TableData {
                input_pins: vec![1],
                output_pin: 16,
                table: vec![true, false],
                enable_flip_flop: false,
            },
        ];

        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i], "at {}", i);
        }
    }
}

fn main() {
    let data = read_file("code.txt");
    println!("{:?}", parse(data));
}
