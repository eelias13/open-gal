#[cfg(cli)]
use serde_json::Value;

#[derive(PartialEq, Debug, Clone)]
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

    #[cfg(cli)]
    pub fn from_json(json: &Value) -> Option<Self> {
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
}

#[cfg(cli)]
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

#[cfg(test)]
mod tests {

    #[cfg(cli)]
    #[test]
    fn load() {
        use serde_json::json;
        let json = json!(
            "InputPins": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23],
            "NumFuses": 5892,
            "TotalNumPins": 24,
            "OutputPins": [[14, 8], [15, 10], [16,12], [17, 14], [18, 16], [19, 16], [20, 14], [21, 12], [22, 10], [23, 8]],
            "SpecialPins": [[13, 42]]

        );
        assert_eq!(
            CircuitConfig::from_json(json).unwrap(),
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
