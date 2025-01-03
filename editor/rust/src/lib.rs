use open_gal::CircuitConfig;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rs_compile(code: String, config: &str) -> JsValue {
    let config = match config {
        "g22v10" => CircuitConfig::new(
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
        ),
        _ => {
            let err: Result<String, String> =
                Err(format!("CircuitConfig {} is not define", config));
            return serde_wasm_bindgen::to_value(&err).unwrap();
        }
    };

    if let Ok(truth_tables) = open_gal::parse(&code) {
        serde_wasm_bindgen::to_value(&open_gal::to_jedec(&truth_tables, &config, None)).unwrap()
    } else {
        return serde_wasm_bindgen::to_value(&open_gal::parse(&code)).unwrap();
    }
}

#[wasm_bindgen]
pub fn rs_transpile(code: String) -> JsValue {
    if let Ok(table_data) = open_gal::parse(&code) {
        let result: Result<String, String> = Ok(open_gal::to_wincupl(&table_data, None));
        serde_wasm_bindgen::to_value(&result).unwrap()
    } else {
        serde_wasm_bindgen::to_value(&open_gal::parse(&code)).unwrap()
    }
}

#[wasm_bindgen]
pub fn rs_tabledata(code: String) -> JsValue {
    serde_wasm_bindgen::to_value(&open_gal::parse(&code)).unwrap()
}
