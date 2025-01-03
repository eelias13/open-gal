use crate::TableData;
use bool_algebra::update_values;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

const IN_PREFIX: &str = "in_";
const OUT_PREFIX: &str = "out_";

pub fn to_wincupl(table_data: &Vec<TableData>, head: Option<&str>) -> String {
    let mut result = if let Some(head) = head {
        format!("{}\n", head)
    } else {
        String::new()
    };

    let mut in_pins = Vec::new();
    let mut out_pins = Vec::new();
    for td in table_data.clone() {
        out_pins.push(td.output_pin);
        for pin in td.input_pins.clone() {
            in_pins.push(pin);
        }
    }
    result.push_str(&pins_def(IN_PREFIX, in_pins));
    result.push_str(&pins_def(OUT_PREFIX, out_pins));
    result.push_str("\n\n");

    for td in table_data.clone() {
        result.push_str(&table_def(td));
    }

    result
}

fn table_def(table_data: TableData) -> String {
    let mut hasher = DefaultHasher::new();
    let mut result = String::new();

    table_data.hash(&mut hasher);
    let td_hash = hasher.finish();

    result.push_str(&format!("Field {}{}f = [", IN_PREFIX, td_hash));
    for in_pin in table_data.input_pins.clone() {
        result.push_str(&format!("{}{}p, ", IN_PREFIX, in_pin));
    }
    result.pop();
    result.pop();
    result.push_str("];\n");

    result.push_str(&format!(
        "Field {}{}f = {}{}p;\n",
        OUT_PREFIX, td_hash, OUT_PREFIX, table_data.output_pin
    ));

    result.push_str(&format!(
        "Table {}{}f => {}{}f ",
        IN_PREFIX, td_hash, OUT_PREFIX, td_hash
    ));
    result.push_str("{\n");

    result.push_str(&build_table(table_data.table));
    result.push_str("}\n\n");

    result
}

fn pins_def(prefix: &str, pins: Vec<u32>) -> String {
    let pins = remove_duplicates(pins);
    let mut result = String::new();
    for pin in pins {
        result.push_str(&format!("Pin {} = {}{}p;\n", pin, prefix, pin));
    }
    result
}

fn build_table(table: Vec<bool>) -> String {
    let mut result = String::new();
    let mut count = vec![false; (table.len() as f64).sqrt() as usize];
    for b in table {
        result.push_str("  'b'");
        for val in count.clone() {
            if val {
                result.push('1');
            } else {
                result.push('0');
            }
        }
        result.push_str(" => 'b'");
        if b {
            result.push('1');
        } else {
            result.push('0');
        }
        result.push_str(";\n");
        update_values(&mut count);
    }
    result
}

fn remove_duplicates(arr: Vec<u32>) -> Vec<u32> {
    let mut out = Vec::new();
    for i in 0..arr.len() {
        if !arr[(i + 1)..arr.len()].contains(&arr[i]) {
            out.push(arr[i])
        }
    }

    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn remove_duplicates() {
        let vec = vec![1, 2, 1, 1, 4, 3, 3, 1];
        assert_eq!(super::remove_duplicates(vec), vec![2, 4, 3, 1])
    }
}
