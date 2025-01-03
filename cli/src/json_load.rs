use open_gal::{CircuitConfig, TableData};
use serde_json::{json, Map, Value};

pub fn read_json(path: &str) -> Value {
    let json_str = std::fs::read_to_string(path).expect(&format!("file: {} not found", path));
    let json: Value = serde_json::from_str(&json_str).unwrap();
    json
}

pub fn td_from_json_vec(json: &Value) -> Option<Vec<TableData>> {
    let mut table_data = Vec::new();
    for val in json["TableData"].as_array()? {
        table_data.push(table_data_from_json(val)?);
    }

    Some(table_data)
}

pub fn td_to_json_vec(table_data: &Vec<TableData>) -> Value {
    let json_arr = table_data.iter().map(|td| table_data_to_json(td)).collect();
    let mut map = Map::new();
    map.insert("TableData".to_string(), Value::Array(json_arr));
    Value::Object(map)
}

pub fn circuit_config_from_json(json: &Value) -> Option<CircuitConfig> {
    let num_fuses = json["NumFuses"].as_u64()? as u32;
    let num_pins = json["TotalNumPins"].as_u64()? as u32;
    let mut inputs = Vec::new();
    for val in json["InputPins"].as_array()? {
        inputs.push(val.as_u64()? as u32);
    }
    let outputs = vec_u32_pair(&json["OutputPins"])?;
    let special_pins = vec_u32_pair(&json["SpecialPins"])?;

    Some(CircuitConfig {
        num_fuses,
        num_pins,
        inputs,
        outputs,
        special_pins,
    })
}

fn vec_u32_pair(json: &Value) -> Option<Vec<(u32, u32)>> {
    let mut result = Vec::new();
    for val in json.as_array()? {
        let pair = val.as_array()?;
        if pair.len() != 2 {
            return None;
        }
        let first = pair[0].as_u64()? as u32;
        let second = pair[1].as_u64()? as u32;
        result.push((first, second));
    }
    Some(result)
}

fn table_data_to_json(table_data: &TableData) -> Value {
    let mut map = Map::new();
    map.insert("dff".to_string(), json!(table_data.enable_flip_flop));
    map.insert("inputPins".to_string(), json!(table_data.input_pins));
    map.insert("outputPin".to_string(), json!(table_data.output_pin));
    map.insert("table".to_string(), json!(table_data.table));
    Value::Object(map)
}

fn table_data_from_json(json: &Value) -> Option<TableData> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_conversion() {
        let json = json!({
          "dff": false,
          "inputPins": [13, 11],
          "outputPin": 17,
          "table": [false, false, false, true]
        });
        let table_data = table_data_from_json(&json).unwrap();
        assert_eq!(json, table_data_to_json(&table_data));
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
        let table_data = td_from_json_vec(&json).unwrap();
        assert_eq!(json, td_to_json_vec(&table_data));
    }

    #[test]
    fn load() {
        use serde_json::json;
        let json = json!({
            "InputPins": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23],
            "NumFuses": 5892,
            "TotalNumPins": 24,
            "OutputPins": [[14, 8], [15, 10], [16,12], [17, 14], [18, 16], [19, 16], [20, 14], [21, 12], [22, 10], [23, 8]],
            "SpecialPins": [[13, 42]]
        });

        assert_eq!(
            circuit_config_from_json(&json).unwrap(),
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
