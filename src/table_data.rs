/*
*   This data structure contains following data from processed expressions.
*
*   - "input_pins" stores all the input pins which are used in the expression
*   - "output_pin" stores the output pin
*   - "table" contains the truth table for the expression and is used to generate a dnf expression later on
*   - "enable_flip_flop" holds a boolean which decides if the output pin should have its flip flop turned on.
*/

#[derive(PartialEq, Debug, Clone)]
pub struct TableData {
    pub input_pins: Vec<u32>,
    pub output_pin: u32,
    pub table: Vec<bool>,
    pub enable_flip_flop: bool,
}