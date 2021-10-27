use crate::CircuitConfig;

/// This data structure contains following data from processed expressions.
///
/// - "input_pins" stores all the input pins which are used in the expression
/// - "output_pin" stores the output pin
/// - "table" contains the truth table for the expression and is used to generate a dnf expression later on
/// - "enable_flip_flop" holds a boolean which decides if the output pin should have its flip flop turned on.
#[derive(PartialEq, Debug, Clone)]
pub struct TableData {
    pub input_pins: Vec<u32>,
    pub output_pin: u32,
    pub table: Vec<bool>,
    pub enable_flip_flop: bool,
}

impl TableData {
    pub fn new(
        input_pins: Vec<u32>,
        output_pin: u32,
        table: Vec<bool>,
        enable_flip_flop: bool,
    ) -> Self {
        Self {
            input_pins,
            output_pin,
            table,
            enable_flip_flop,
        }
    }

    pub fn valid(&self, config: &CircuitConfig) -> Result<(), String> {
        if self.input_pins.len() > config.inputs.len() {
            return Err("Too many input pins".to_string());
        } else if self.table.len() != 2_usize.pow(self.input_pins.len() as u32) {
            return Err("Truth table size doesn't match input bits".to_string());
        }
        for pin in self.input_pins.clone() {
            if !config.inputs.contains(&pin) {
                return Err(format!("input pin {} is not deficient in config", pin));
            }
        }

        let output1: Vec<u32> = config
            .outputs
            .clone()
            .iter()
            .map(|(e, _)| e.clone())
            .collect();
        let output2: Vec<u32> = config
            .outputs
            .clone()
            .iter()
            .map(|(_, e)| e.clone())
            .collect();

        if !output1.contains(&self.output_pin) && !output2.contains(&self.output_pin) {
            return Err(format!(
                "input pin {} is not deficient in config",
                self.output_pin
            ));
        }
        Ok(())
    }
}
