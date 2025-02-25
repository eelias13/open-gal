use open_gal::{CircuitConfig, TableData};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TableDataWrapper {
    #[serde(rename = "inputPins")]
    pub input_pins: Vec<u32>,
    #[serde(rename = "outputPin")]
    pub output_pin: u32,
    #[serde(rename = "table")]
    pub table: Vec<bool>,
    #[serde(rename = "dff")]
    pub enable_flip_flop: bool,
}

impl From<TableDataWrapper> for TableData {
    fn from(wrapper: TableDataWrapper) -> Self {
        Self {
            enable_flip_flop: wrapper.enable_flip_flop,
            input_pins: wrapper.input_pins,
            output_pin: wrapper.output_pin,
            table: wrapper.table,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CircuitConfigWrapper {
    // pub num_fuses: u32,
    // pub num_pins: u32,
    // pub inputs: Vec<u32>,
    // pub outputs: Vec<(u32, u32)>,
    // pub special_pins: Vec<(u32, u32)>,
    #[serde(rename = "InputPins")]
    pub inputs: Vec<u32>,

    #[serde(rename = "NumFuses")]
    pub num_fuses: u32,

    #[serde(rename = "TotalNumPins")]
    pub num_pins: u32,

    #[serde(rename = "OutputPins")]
    pub outputs: Vec<(u32, u32)>,

    #[serde(rename = "SpecialPins")]
    pub special_pins: Vec<(u32, u32)>,
}

impl From<CircuitConfigWrapper> for CircuitConfig {
    fn from(wrapper: CircuitConfigWrapper) -> Self {
        Self {
            num_fuses: wrapper.num_fuses,
            num_pins: wrapper.num_pins,
            inputs: wrapper.inputs,
            outputs: wrapper.outputs,
            special_pins: wrapper.special_pins,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn json_conversion() {
        let json = json!({
          "dff": false,
          "inputPins": [13, 11],
          "outputPin": 17,
          "table": [false, false, false, true]
        });

        let table_data: TableDataWrapper = serde_json::from_str(&json.to_string()).unwrap();
        assert_eq!(json, serde_json::to_value(&table_data).unwrap());
    }

    #[test]
    fn json_conversion_arr() {
        let json = json!({
          "TableData": [
            {
              "dff": false,
              "inputPins": [13, 11],
              "outputPin": 17,
              "table": [false, false, false, true]
            },
            {
              "dff": false,
              "inputPins": [13, 11],
              "outputPin": 19,
              "table": [false, true, true, false]
            },
            {
              "dff": false,
              "inputPins": [13, 11],
              "outputPin": 18,
              "table": [false, true, true, true]
            },
            {
              "dff": true,
              "inputPins": [3, 2],
              "outputPin": 23,
              "table": [true, true, false, true]
            }
          ]
        });

        let table_data: Vec<TableDataWrapper> = serde_json::from_str(&json.to_string()).unwrap();
        assert_eq!(json, serde_json::to_value(&table_data).unwrap());
    }

    #[test]
    fn load() {
        let json = json!({
            "InputPins": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23],
            "NumFuses": 5892,
            "TotalNumPins": 24,
            "OutputPins": [[14, 8], [15, 10], [16,12], [17, 14], [18, 16], [19, 16], [20, 14], [21, 12], [22, 10], [23, 8]],
            "SpecialPins": [[13, 42]]
        });

        let circuit_config: CircuitConfig =
            serde_json::from_str::<CircuitConfigWrapper>(&json.to_string())
                .unwrap()
                .try_into()
                .unwrap();

        assert_eq!(
            circuit_config,
            CircuitConfig {
                inputs: vec![
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23
                ],
                num_fuses: 5892,
                num_pins: 24,
                outputs: vec![
                    (14, 8),
                    (15, 10),
                    (16, 12),
                    (17, 14),
                    (18, 16),
                    (19, 16),
                    (20, 14),
                    (21, 12),
                    (22, 10),
                    (23, 8)
                ],
                special_pins: vec![(13, 42)]
            }
        );
    }
}
