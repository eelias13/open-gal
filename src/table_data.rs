use crate::CircuitConfig;
#[cfg(cli)]
use serde_json::{json, Map, Value};

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

#[cfg(cli)]
impl TableData {
    fn to_json(&self) -> Value {
        let mut map = Map::new();
        map.insert("dff".to_string(), json!(self.enable_flip_flop));
        map.insert("inputPins".to_string(), json!(self.input_pins));
        map.insert("outputPin".to_string(), json!(self.output_pin));
        map.insert("table".to_string(), json!(self.table));
        Value::Object(map)
    }

    fn from_json(json: &Value) -> Option<Self> {
        let mut input_pins = Vec::new();
        for val in json["inputPins"].as_array()? {
            input_pins.push(val.as_u64()? as u32);
        }
        let output_pin = json["outputPin"].as_u64()? as u32;
        let mut table = Vec::new();
        for val in json["table"].as_array()? {
            table.push(val.as_bool()?);
        }
        let enable_flip_flop = json["dff"].as_bool()?;

        let table_data = TableData {
            input_pins,
            output_pin,
            table,
            enable_flip_flop,
        };

        Some(table_data)
    }
}

#[cfg(cli)]
pub fn from_json_vec(json: &Value) -> Option<Vec<TableData>> {
    let mut table_data = Vec::new();
    for val in json["TableData"].as_array()? {
        table_data.push(TableData::from_json(val)?);
    }

    Some(table_data)
}

#[cfg(cli)]
pub fn to_json_vec(table_data: &Vec<TableData>) -> Value {
    let json_arr = table_data.iter().map(|td| TableData::to_json(td)).collect();
    let mut map = Map::new();
    map.insert("TableData".to_string(), Value::Array(json_arr));
    Value::Object(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(cli)]
    #[test]
    fn json_conversion() {
        let json = json!({
          "dff": false,
          "inputPins": [13, 11],
          "outputPin": 17,
          "table": [false, false, false, true]
        });
        let table_data = json2table_data(&json).unwrap();
        assert_eq!(json, table_data2json(&table_data));
    }

    #[test]
    #[cfg(cli)]
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
        let table_data = from_json().unwrap();
        assert_eq!(json, to_json(&table_data));
    }
}
