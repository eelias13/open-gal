use crate::{CircuitConfig, TableData};

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

#[derive(Debug, Clone)]
pub struct Row {
    pub pins: Vec<Pin>,
}

impl Row {
    pub fn new(mut bits: Vec<bool>, inputs: Vec<u32>) -> Self {
        let mut pins = Vec::new();
        bits.reverse();

        for i in 0..inputs.len() {
            pins.push(Pin::new(!bits[i], inputs[i]));
        }

        Self { pins }
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Row) -> bool {
        for pin in self.pins.clone() {
            if !other.pins.contains(&pin) {
                return false;
            }
        }
        true
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

        for (i, &val) in truth_table.table.clone().iter().enumerate() {
            if val {
                let bits = uint_to_bool_vec(i as u32);
                rows.push(Row::new(bits, truth_table.input_pins.clone()));
            }
        }
        Ok(Self {
            out_pin: truth_table.output_pin,
            enable_flip_flop: truth_table.enable_flip_flop,
            rows,
        })
    }
}

fn uint_to_bool_vec(num: u32) -> Vec<bool> {
    let mut result = Vec::with_capacity(32);
    let string_rep = format!("{:#034b}", num);

    // skip 0b
    for c in string_rep.chars().skip(2) {
        let val = match c {
            '0' => false,
            '1' => true,
            _ => panic!("unexpected char {} expected 1 or 0", c), // only has 0  or 1
        };
        result.push(val);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_convesion() {
        let bits = uint_to_bool_vec(5); // 5 -> 0101
        assert_eq!(bits.len(), 32);
        assert_eq!(
            bits,
            vec![
                false, false, false, false, false, false, false, false, //
                false, false, false, false, false, false, false, false, //
                false, false, false, false, false, false, false, false, //
                false, false, false, false, false, true, false, true
            ]
        );

        assert_eq!(
            uint_to_bool_vec(238934),
            vec![
                false, false, false, false, false, false, false, false, //
                false, false, false, false, false, false, true, true, //
                true, false, true, false, false, true, false, true, //
                false, true, false, true, false, true, true, false //
            ]
        );

        assert_eq!(
            uint_to_bool_vec(0xff),
            vec![
                false, false, false, false, false, false, false, false, //
                false, false, false, false, false, false, false, false, //
                false, false, false, false, false, false, false, false, //
                true, true, true, true, true, true, true, true //
            ]
        );

        assert_eq!(uint_to_bool_vec(std::u32::MIN), vec![false; 32]);
        assert_eq!(uint_to_bool_vec(std::u32::MAX), vec![true; 32]);
    }

    #[test]
    fn expression_new() {
        let config = CircuitConfig::new(
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

        let table_data = vec![
            TableData::new(
                vec![3, 2],
                23,
                //   00    01     10     11
                vec![true, true, false, true],
                true,
            ),
            TableData::new(vec![10, 11], 23, vec![false, false, true, false], true),
            TableData::new(vec![10, 11], 17, vec![false, false, false, true], false),
            TableData::new(vec![10, 11], 19, vec![false, true, true, false], false),
            TableData::new(vec![10, 11], 18, vec![false, true, true, true], false),
        ];

        let expressions = vec![
            Expression {
                out_pin: 23,
                enable_flip_flop: true,
                rows: vec![
                    Row {
                        pins: vec![Pin::new(true, 3), Pin::new(true, 2)],
                    },
                    Row {
                        pins: vec![Pin::new(false, 3), Pin::new(true, 2)],
                    },
                    Row {
                        pins: vec![Pin::new(false, 3), Pin::new(false, 2)],
                    },
                ],
            },
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
        ];

        for i in 0..table_data.len() {
            assert_eq!(
                Expression::new(&table_data[i], &config),
                Ok(expressions[i].clone())
            );
        }
    }
}
