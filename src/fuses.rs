use crate::{CircuitConfig, Expression};

/*
*		Fuses::BuildFromExpression generates a fuselist for a specific expression and outputs the result in a supplied
*		fuselist. It needs to know the term size and number of rows to correctly pad the fuselist with zeroes.
*/

fn build_from_expression(
    expression: &Expression,
    num_rows: u32,
    row_length: u32,
    config: &CircuitConfig,
) -> Result<Vec<bool>, String> {
    if !is_valid(expression.out_pin, &config) {
        return Err("Expression has invalid output pin".to_string());
    } else if expression.rows.len() > maximum_terms(expression.out_pin, config)? as usize {
        return Err("Too many terms for given output pin".to_string());
    }

    let mut fuse_list = vec![true; (num_rows * row_length) as usize];

    //	Start writing DNF terms.
    for term_index in 0..expression.rows.len() {
        for pin_index in 0..expression.rows[term_index].pins.len() {
            let pin = expression.rows[term_index].pins[pin_index].clone();

            let mode = if expression.enable_flip_flop {
                MacrocellMode::ModeRegisteredHigh
            } else {
                MacrocellMode::ModeCombinatorialHigh
            };
            let index = pin_to_index(pin.pin_num, pin.inverted, mode, config)?;
            let index = row_length as usize + term_index * row_length as usize + index as usize;
            fuse_list[index] = false;
        }
    }

    Ok(fuse_list)
}

/*

bool Fuses::Build(vector<DNF::Expression> Expressions, vector<bool>& FuseListOut, Configs::CircuitConfig* pConfig)
{
    if (!Expressions.size())
    {
        ERROR("%s", "No expressions were given");
        return false;
    }

    //	Get row length for one DNF term.

    uint32_t iRowLength = Fuses::GetRowLength(pConfig);

    if (FuseListOut.size())
        FuseListOut.clear();

    //	Adjust fuselist size to the fuse list size of the integrated circuit.

    FuseListOut.resize(pConfig->m_iNumFuses);

    //	Set AR Fuses to zero (we don't need them as of yet)

    std:fill(FuseListOut.begin(), FuseListOut.begin() + iRowLength, false);

    //	Start writing expressions to FuseList.

    for(uint32_t Index = 0; Index < Expressions.size(); Index++)
    {
        uint32_t ExpIndexStart = Fuses::Output::GetFirstFuseIndex(Expressions[Index].m_OutputPin, pConfig);

        if(ExpIndexStart == -1)
        {
            ERROR("%s", "Couldn't get fuse index start");
            return false;
        }

        vector<bool> ExpressionBuffer;

        if (!Fuses::BuildFromExpression(Expressions[Index], Fuses::Output::MaximumTerms(Expressions[Index].m_OutputPin, pConfig) + 1, iRowLength, ExpressionBuffer, pConfig))
        {
            ERROR("%s", "Couldn't build all expression fuses");
            return false;
        }

        //	Copy ExpressionBuffer into the correct target destination in the fuse matrix.

        std::copy(ExpressionBuffer.begin(), ExpressionBuffer.end(), FuseListOut.begin() + ExpIndexStart);
    }

    //	Set SP fuses to zero because we also don't need them as of yet.

    uint32_t iLastFuseIDX = Fuses::Output::GetLastFuseIndex(pConfig->m_Outputs.front().first, pConfig);
    std::fill(FuseListOut.begin() + iLastFuseIDX, FuseListOut.begin() + iLastFuseIDX + iRowLength, false);

    //	Set S0 & S1 fuses.

    for(DNF::Expression Expression : Expressions)
    {
        pair<uint32_t, uint32_t> ModeFuses;

        if(!Fuses::Output::ModeFuseIndices(Expression.m_OutputPin, ModeFuses, pConfig))
        {
            ERROR("%s", "Invalid PIN");
            return false;
        }

        if(Expression.m_EnableFlipFlop)
        {
            FuseListOut[ModeFuses.first] = 1;
            FuseListOut[ModeFuses.second] = 0;
        }
        else
        {
            FuseListOut[ModeFuses.first] = 1;
            FuseListOut[ModeFuses.second] = 1;
        }
    }

    return true;
}
*/

#[derive(PartialEq, Debug, Clone, Copy)]
enum MacrocellMode {
    ModeNone,
    ModeRegisteredHigh,
    ModeRegisteredLow,
    ModeCombinatorialHigh,
    ModeCombinatorialLow,
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
    for (output_pin, _) in config.outputs.clone() {
        if pin_num == output_pin {
            return true;
        }
    }
    false
}

/// Fuses::Output::ModeFuseIndices returns the mode control fuses for a given output pin.
/// The return value is a boolean which indicates if the fuse pair was written to the given
/// std::pair reference. The function will only return false if the given pin number is
/// an input pin who has no OLMC connected and therefore no control mode pin.
fn mode_fuse_indices(
    pin_num: u32,
    fuses_out: &mut (u32, u32),
    config: &CircuitConfig,
) -> Result<(u32, u32), String> {
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

    Err(String::new())
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
    for output_pin in config.outputs.clone() {
        if output_pin.0 == pin_num {
            return Ok(output_pin.1);
        }
    }
    Err(String::new())
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
                return Err(String::new());
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
