use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CircuitConfig {
    pub num_fuses: u32,
    pub num_pins: u32,
    pub inputs: Vec<u32>,
    pub outputs: Vec<(u32, u32)>,
    pub special_pins: Vec<(u32, u32)>,
}

impl CircuitConfig {
    pub fn new(
        num_fuses: u32,
        num_pins: u32,
        inputs: Vec<u32>,
        outputs: Vec<(u32, u32)>,
        special_pins: Vec<(u32, u32)>,
    ) -> Self {
        Self {
            num_fuses,
            num_pins,
            inputs,
            outputs,
            special_pins,
        }
    }
}
