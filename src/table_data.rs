/*
*   This data structure contains following data from processed expressions.
*
*   - "m_InputPins" stores all the input pins which are used in the expression
*   - "m_OutputPin" stores the output pin
*   - "m_Table" contains the truth table for the expression and is used to generate a dnf expression later on
*   - "m_EnableDFlipFlop" holds a boolean which decides if the output pin should have its flip flop turned on.
*/

#[derive(PartialEq, Debug, Clone)]
pub struct TableData {
    pub input_pins: Vec<u32>,
    pub output_pin: u32,
    pub table: Vec<bool>,
    pub enable_flip_flop: bool,
}