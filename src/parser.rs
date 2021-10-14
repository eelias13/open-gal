use crate::atom::{AtomType, TableType};
use crate::atomizer::Atomizer;
use crate::error::ParsingError;
use crate::lexer::Lexer;
use crate::TableData;
use std::{collections::HashMap, u32};

pub fn parse(data: Vec<String>) -> Result<Vec<TableData>, ParsingError> {
    let mut result = Vec::new();

    let mut lexer = Lexer::new(data.clone());
    let tokens = lexer.lex()?;
    let mut atomizer = Atomizer::new(data.clone(), tokens);
    let atoms = atomizer.atomize()?;

    let mut pin_map: HashMap<String, u32> = HashMap::new();
    let mut used_pin = Vec::new();
    let mut is_dff = Vec::<u32>::new();

    for atom in atoms {
        match atom.atom_type() {
            AtomType::Pin { pins, names } => match set_pins(names, pins, &mut pin_map) {
                Ok(()) => (),
                Err(msg) => {
                    let err = ParsingError::from_atom(atom, msg, data);
                    return Err(err);
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
                    return Err(err);
                }
            },
            AtomType::BoolFunc { in_names, func } => {
                match parse_func(in_names, func, &mut pin_map, &mut used_pin) {
                    Ok(table_data) => table_data.iter().for_each(|td| result.push(td.clone())),
                    Err(msg) => {
                        let err = ParsingError::from_atom(atom, msg, data);
                        return Err(err);
                    }
                }
            }
            AtomType::Dff { names } => match get_pins(names, &mut pin_map, &mut used_pin) {
                Ok(pins) => pins.iter().for_each(|&p| is_dff.push(p)),
                Err(msg) => {
                    let err = ParsingError::from_atom(atom, msg, data);
                    return Err(err);
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

    Ok(result)
}

fn parse_func(
    in_names: Vec<String>,
    func: Vec<bool_func_parser::Token>,
    pin_map: &mut HashMap<String, u32>,
    used_pin: &mut Vec<u32>,
) -> Result<Vec<TableData>, String> {
    let mut result = Vec::new();
    let output_pins = match get_pins(in_names, pin_map, used_pin) {
        Ok(pins) => pins,
        Err(pin_name) => return Err(format!("pin <{}> is not definde", pin_name)),
    };

    for output_pin in output_pins {
        if let Some(table) = bool_func_parser::parse(&func) {
            let in_names = bool_func_parser::get_names(&func);
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
    let table_2d =
        match crate::table_parser::parse(in_names.len(), out_names.len(), table, table_type) {
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

// ---------------------------------------------------------------------- Tests ----------------------------------------------------------------------

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

        let input = parse(data.iter().map(|l| format!("{}\n", l)).collect()).unwrap();
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
        let input = parse(data.iter().map(|l| format!("{}\n", l)).collect()).unwrap();
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
