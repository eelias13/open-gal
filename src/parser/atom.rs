use crate::parser::token::Token;

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
        func: Vec<bool_algebra::Token>,
    },
    Dff {
        names: Vec<String>,
    },
}

#[derive(PartialEq, Debug, Clone)]
pub enum TableType {
    Fill(bool),
    Full,
    Count,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Atom {
    pub begin_char: usize,
    pub len_char: usize,
    pub len_line: usize,
    pub begin_line: usize,
    pub begin_token: usize,
    pub len_token: usize,
    pub atom_type: AtomType,
}

impl Atom {
    pub fn begin_char(&self) -> usize {
        self.begin_char
    }

    pub fn len_char(&self) -> usize {
        self.len_char
    }

    pub fn len_line(&self) -> usize {
        self.len_line
    }

    pub fn begin_line(&self) -> usize {
        self.begin_line
    }

    pub fn atom_type(&self) -> AtomType {
        self.atom_type.clone()
    }
}
impl Atom {
    pub fn new(
        tokens: &Vec<Token>,
        begin_token: usize,
        len_token: usize,
        atom_type: AtomType,
    ) -> Self {
        Self {
            begin_char: tokens[0].begin_char(),
            len_char: tokens.into_iter().map(|t| t.len_char()).sum(),
            begin_line: tokens[0].len_line(),
            len_line: tokens.into_iter().map(|t| t.len_line()).sum(),
            begin_token,
            len_token,
            atom_type,
        }
    }
}
