use crate::translator::dnf::Expression;
use crate::CircuitConfig;

/// Fuses::BuildFromExpression generates a fuselist for a specific expression and outputs the result in a supplied
///	fuselist. It needs to know the term size and number of rows to correctly pad the fuselist with zeroes.
fn build_from_expression(
    expr: &Expression,
    num_rows: u32,
    row_len: u32,
    config: &CircuitConfig,
) -> Result<Vec<bool>, String> {
    if !is_valid(expr.out_pin, &config) {
        return Err("Expression has invalid output pin".to_string());
    } else if expr.rows.len() > maximum_terms(expr.out_pin, config)? as usize {
        return Err("Too many terms for given output pin".to_string());
    }

    let mut fuse_list = vec![false; (num_rows * row_len) as usize];

    for i in 0..row_len as usize {
        fuse_list[i] = true;
    }

    //	Start writing DNF terms.
    for term_index in 0..expr.rows.len() {
        for i in 0..row_len {
            let index = (row_len + term_index as u32 * row_len + i) as usize;
            fuse_list[index] = true;
        }

        for pin_index in 0..expr.rows[term_index].pins.len() {
            let pin = expr.rows[term_index].pins[pin_index].clone();

            let mode = if expr.enable_flip_flop {
                MacrocellMode::ModeRegisteredHigh
            } else {
                MacrocellMode::ModeCombinatorialHigh
            };
            let index = pin_to_index(pin.pin_num, pin.inverted, mode, config)?;
            let index = row_len as usize + term_index * row_len as usize + index as usize;
            fuse_list[index] = false;
        }
    }

    Ok(fuse_list)
}

pub fn build(exprs: &Vec<Expression>, config: &CircuitConfig) -> Result<Vec<bool>, String> {
    //	Get row length for one DNF term.
    let row_len = get_row_length(config);

    //	Adjust fuselist size to the fuse list size of the integrated circuit.
    //	Set AR Fuses to zero (we don't need them as of yet)
    let mut fuse_out = vec![false; config.num_fuses as usize];

    //	Start writing expressions to FuseList.
    for expr in exprs.clone() {
        let expr_start = get_first_fuse_index(expr.out_pin, config)? as usize;
        let num_rows = maximum_terms(expr.out_pin, config)?;

        let expr_buf = build_from_expression(&expr, num_rows + 1, row_len, config)?;

        //	Copy ExpressionBuffer into the correct target destination in the fuse matrix.
        for i in 0..expr_buf.len() {
            fuse_out[i + expr_start] = expr_buf[i];
        }
    }

    //	Set SP fuses to zero because we also don't need them as of yet.
    let last_fuse_idx = get_last_fuse_index(config.outputs.first().unwrap().0, config)?;
    for i in 0..row_len {
        fuse_out[(i + last_fuse_idx) as usize] = false;
    }

    //	Set S0 & S1 fuses.
    for expr in exprs {
        let mode_fuses = mode_fuse_indices(expr.out_pin, config)?;

        if expr.enable_flip_flop {
            fuse_out[(mode_fuses.0) as usize] = true;
            fuse_out[(mode_fuses.1) as usize] = false;
        } else {
            fuse_out[(mode_fuses.0) as usize] = true;
            fuse_out[(mode_fuses.1) as usize] = true;
        }
    }

    Ok(fuse_out)
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum MacrocellMode {
    ModeNone,
    ModeRegisteredHigh,
    // ModeRegisteredLow,
    ModeCombinatorialHigh,
    // ModeCombinatorialLow,
}

/// Fuses::Output::GetLastFuseIndex returns the last fuse of an OLMC output.
fn get_last_fuse_index(pin_num: u32, config: &CircuitConfig) -> Result<u32, String> {
    if !is_valid(pin_num, config) {
        return Err("Invalid output pin".to_string());
    }

    Ok(get_first_fuse_index(pin_num, config)?
        + (maximum_terms(pin_num, config)? + 1) * get_row_length(config))
}

/// Fuses::Output::IsValid checks if a given pin is an output pin:
fn is_valid(pin_num: u32, config: &CircuitConfig) -> bool {
    for (out_pin, _) in config.outputs.clone() {
        if pin_num == out_pin {
            return true;
        }
    }
    false
}

/// Fuses::Output::ModeFuseIndices returns the mode control fuses for a given output pin.
/// The return value is a boolean which indicates if the fuse pair was written to the given
/// std::pair reference. The function will only return false if the given pin number is
/// an input pin who has no OLMC connected and therefore no control mode pin.
fn mode_fuse_indices(pin_num: u32, config: &CircuitConfig) -> Result<(u32, u32), String> {
    if !is_valid(pin_num, config) {
        return Err(String::new());
    }

    //	Get last fuse.
    let fuse_index = get_last_fuse_index(config.outputs[0].0, config)? + get_row_length(config);

    //	Get OLMC number.
    for i in 0..config.outputs.len() {
        if config.outputs[i].0 == pin_num {
            let fuses_start = fuse_index + ((config.outputs.len() - 1 - i) as u32) * 2;
            return Ok((fuses_start, fuses_start + 1));
        }
    }

    Err(format!("output pin number {} not found in config", pin_num))
}

/// Fuses::Output::GetFirstFuseIndex returns the first fuse of an OLMC output.
fn get_first_fuse_index(pin_num: u32, config: &CircuitConfig) -> Result<u32, String> {
    if !is_valid(pin_num, config) {
        return Err("Invalid output pin".to_string());
    }

    let mut fuse_index = get_row_length(config);

    let mut olmc = config.outputs.last().unwrap().0;
    while olmc > pin_num {
        fuse_index += (maximum_terms(olmc, config)? + 1) * get_row_length(config);
        olmc -= 1;
    }
    Ok(fuse_index)
}

/// Fuses::GetRowLength returns the length of one DNF term row.
fn get_row_length(config: &CircuitConfig) -> u32 {
    ((config.inputs.len() + config.special_pins.len()) * 2) as u32
}

/// Fuses::Output::MaximumTerms returns the maximum amount of terms an output OLMC can handle.
/// if the function return value is -1 it means that the given pin number is not an valid output pin
/// thus the function can't return a valid term number.
fn maximum_terms(pin_num: u32, config: &CircuitConfig) -> Result<u32, String> {
    for out_pin in config.outputs.clone() {
        if out_pin.0 == pin_num {
            return Ok(out_pin.1);
        }
    }
    Err(format!("output pin number {} not found in config", pin_num))
}

/// Fuses::PinToIndex converts a PIN to a fuselist row index. It takes in a PIN number and a boolean
/// which indicates if the PIN is supposed to be inverted. It requires an extra parameter if the given
/// PIN number correlates to an output pin. This parameter is called "Mode" and is an enum. The parameter
/// is needed because the output from the OLMC is inverted or not inverted depending on the mode it is
/// operating in (the mode is set through S0 and S1 fuses) so we need to know the mode to correctly pick
/// the inverted or non inverted output from the OLMC output.
///
/// Note: The "Mode" parameter is ignored if the PIN number correlates to an input pin. If the PIN is an
/// output pin and the parameter is not set the function will return -1 which is not a valid index.
///
/// Note: If the return value is -1 that means that the function couldn't find a valid index for the given
/// parameters.
fn pin_to_index(
    pin_num: u32,
    inverted: bool,
    mode: MacrocellMode,
    config: &CircuitConfig,
) -> Result<u32, String> {
    // Handles special pins.
    for special_pin in config.special_pins.clone() {
        if special_pin.0 == pin_num {
            if inverted {
                return Ok(special_pin.1 + 1);
            } else {
                return Ok(special_pin.1);
            }
        }
    }

    // Handles output pins.
    let fuse_index;

    for i in 0..config.outputs.len() {
        if config.outputs[i].0 == pin_num {
            fuse_index = (2 + (config.outputs.len() - 1 - i) * 4) as u32;

            if mode == MacrocellMode::ModeCombinatorialHigh {
                if inverted {
                    return Ok(fuse_index + 1);
                } else {
                    return Ok(fuse_index);
                }
            } else if mode != MacrocellMode::ModeNone {
                if inverted {
                    return Ok(fuse_index);
                } else {
                    return Ok(fuse_index + 1);
                }
            } else {
                return Err(format!("unexpected mode {:?}", mode));
            }
        }
    }

    // Handles input pins.
    fuse_index = (pin_num - 1) * 4;
    if inverted {
        return Ok(fuse_index + 1);
    } else {
        return Ok(fuse_index);
    }
}

#[cfg(test)]
mod tests {

    use crate::translator::dnf::Pin;
    use crate::translator::dnf::Row;
    use crate::translator::utils::bool_to_byte;


    #[test]
    fn fuses_as_bytes_test() {
        let fuses = vec![true; 9];
        assert_eq!(fuses_as_bytes(fuses), vec![255, 128])
    }

    #[test]
    fn maximum_terms() {
        let config = super::CircuitConfig::new(
            5892,
            24,
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            ],
            vec![
                (14, 8),
                (15, 10),
                (16, 12),
                (17, 14),
                (18, 16),
                (19, 16),
                (20, 14),
                (21, 12),
                (22, 10),
                (23, 8),
            ],
            vec![(13, 42)],
        );
        assert_eq!(super::maximum_terms(23, &config), Ok(8));
    }

    #[test]
    fn get_row_length() {
        let config = super::CircuitConfig::new(
            5892,
            24,
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            ],
            vec![
                (14, 8),
                (15, 10),
                (16, 12),
                (17, 14),
                (18, 16),
                (19, 16),
                (20, 14),
                (21, 12),
                (22, 10),
                (23, 8),
            ],
            vec![(13, 42)],
        );
        assert_eq!(super::get_row_length(&config), 44);
    }

    #[test]
    fn expression() {
        let expression = super::Expression {
            out_pin: 23,
            enable_flip_flop: true,
            rows: vec![Row {
                pins: vec![Pin::new(false, 11), Pin::new(true, 10)],
            }],
        };

        let config = super::CircuitConfig::new(
            5892,
            24,
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            ],
            vec![
                (14, 8),
                (15, 10),
                (16, 12),
                (17, 14),
                (18, 16),
                (19, 16),
                (20, 14),
                (21, 12),
                (22, 10),
                (23, 8),
            ],
            vec![(13, 42)],
        );

        let row_length = super::get_row_length(&config);
        let num_rows = super::maximum_terms(expression.out_pin, &config).unwrap();

        let result =
            super::build_from_expression(&expression, num_rows + 1, row_length, &config).unwrap();

        assert_eq!(result.len(), 396);
        let bytes = fuses_as_bytes(result);
        assert_eq!(
            bytes,
            vec![
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xB7, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ]
        );
    }

    #[test]
    fn expressions() {
        use super::Expression;

        let config = super::CircuitConfig::new(
            5892,
            24,
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            ],
            vec![
                (14, 8),
                (15, 10),
                (16, 12),
                (17, 14),
                (18, 16),
                (19, 16),
                (20, 14),
                (21, 12),
                (22, 10),
                (23, 8),
            ],
            vec![(13, 42)],
        );

        let expressions = vec![
            Expression {
                out_pin: 23,
                enable_flip_flop: true,
                rows: vec![Row {
                    pins: vec![Pin::new(false, 11), Pin::new(true, 10)],
                }],
            },
            Expression {
                out_pin: 17,
                enable_flip_flop: false,
                rows: vec![Row {
                    pins: vec![Pin::new(false, 11), Pin::new(false, 10)],
                }],
            },
            Expression {
                out_pin: 19,
                enable_flip_flop: false,
                rows: vec![
                    Row {
                        pins: vec![Pin::new(true, 11), Pin::new(false, 10)],
                    },
                    Row {
                        pins: vec![Pin::new(false, 11), Pin::new(true, 10)],
                    },
                ],
            },
            Expression {
                out_pin: 18,
                enable_flip_flop: false,
                rows: vec![
                    Row {
                        pins: vec![Pin::new(true, 11), Pin::new(false, 10)],
                    },
                    Row {
                        pins: vec![Pin::new(false, 11), Pin::new(true, 10)],
                    },
                    Row {
                        pins: vec![Pin::new(false, 11), Pin::new(false, 10)],
                    },
                ],
            },
            Expression {
                out_pin: 23,
                enable_flip_flop: true,
                rows: vec![
                    Row {
                        pins: vec![Pin::new(true, 2), Pin::new(true, 3)],
                    },
                    Row {
                        pins: vec![Pin::new(true, 2), Pin::new(false, 3)],
                    },
                    Row {
                        pins: vec![Pin::new(false, 2), Pin::new(false, 3)],
                    },
                ],
            },
            Expression {
                out_pin: 23,
                enable_flip_flop: true,
                rows: vec![
                    Row {
                        pins: vec![Pin::new(true, 2), Pin::new(false, 3)],
                    },
                    Row {
                        pins: vec![Pin::new(false, 2), Pin::new(true, 3)],
                    },
                ],
            },
        ];

        let result = super::build(&expressions, &config).unwrap();

        assert_eq!(result.len(), 5892);
        let bytes = fuses_as_bytes(result);
        assert_eq!(
            bytes,
            vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x0F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFB, 0x7F, 0xFF,
                0xFF, 0xFF, 0xFF, 0x7B, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x0F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF7,
                0xBF, 0xFF, 0xFF, 0xFF, 0xFF, 0xB7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7B, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFB, 0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0x77, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xF7, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0xFC,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ]
        );
    }

    pub fn fuses_as_bytes(fuses: Vec<bool>) -> Vec<u8> {
        let mut byte = vec![false; 8];
        let mut result = Vec::new();

        for i in 0..((fuses.len() as f64 / 8.).ceil() as usize) {
            for j in 0..8 {
                byte[j] = if let Some(&b) = fuses.get(i * 8 + j) {
                    b
                } else {
                    false
                };
            }
            result.push(bool_to_byte(&byte));
        }
        result
    }
}
