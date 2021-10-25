use crate::{CircuitConfig, TableData};

const MAX_INPUTS: u32 = 64;

#[derive(PartialEq, Debug, Clone)]

pub struct Pin {
    pub inverted: bool,
    pub pin_num: u32,
}

impl Pin {
    pub fn new(inverted: bool, pin_num: u32) -> Self {
        Self { inverted, pin_num }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Row {
    pub pins: Vec<Pin>,
}

impl Row {
    pub fn new(bits: Vec<bool>, inputs: Vec<u32>) -> Self {
        let mut pins = Vec::new();

        for i in 0..inputs.len() {
            if bits[inputs.len() - 1 - i] == false {
                pins.push(Pin::new(true, inputs[i]));
            } else {
                pins.push(Pin::new(false, inputs[i]));
            }
        }

        Self { pins }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Expression {
    pub out_pin: u32,
    pub enable_flip_flop: bool,
    pub rows: Vec<Row>,
}

impl Expression {
    pub fn new(truth_table: &TableData, config: &CircuitConfig) -> Result<Self, String> {
        truth_table.valid(config)?;

        let mut rows = Vec::new();

        for (_i, &val) in truth_table.table.clone().iter().enumerate() {
            if val {
                rows.push(Row::new(Vec::new(), truth_table.input_pins.clone()));
                // Vec::new() = bitset<MAX_INPUTS>(i)
            }
        }
        Ok(Self {
            out_pin: truth_table.output_pin,
            enable_flip_flop: truth_table.enable_flip_flop,
            rows,
        })
    }
}
