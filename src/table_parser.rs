use std::usize;

use super::*;

pub fn parse(
    in_len: usize,
    out_len: usize,
    table: Vec<bool>,
    table_type: TableType,
) -> Result<Vec<Vec<bool>>, String> {
    match table_type {
        TableType::Count => parse_count(in_len, out_len, table),
        TableType::Fill { value } => parse_fill(in_len, out_len, table, value),
        TableType::Full => parse_full(in_len, out_len, table),
    }
}

fn parse_full(in_len: usize, out_len: usize, table: Vec<bool>) -> Result<Vec<Vec<bool>>, String> {
    if table.len() != pow2(in_len) * (out_len + in_len) {
        return Err("incorrect table shape".to_string());
    }
    return parse_fill(in_len, out_len, table, false);
}

fn parse_count(in_len: usize, out_len: usize, table: Vec<bool>) -> Result<Vec<Vec<bool>>, String> {
    if table.len() != pow2(in_len * out_len) {
        return Err("incorrect table shape".to_string());
    }
    let mut result = Vec::new();

    if COUNT_VERTICAL {
        for i in 0..out_len {
            let mut temp = Vec::new();
            for j in 0..pow2(in_len) {
                temp.push(table[i * pow2(in_len) + j]);
            }
            result.push(temp);
        }
    } else {
        for i in 0..out_len {
            result.push(Vec::new());
        }
        for i in 0..table.len() {
            result[i % out_len].push(table[i]);
        }
    }

    Ok(result)
}

fn parse_fill(
    in_len: usize,
    out_len: usize,
    table: Vec<bool>,
    fill: bool,
) -> Result<Vec<Vec<bool>>, String> {
    let vec_2d = split_rows(table, in_len + out_len);
    let result = match_table(vec_2d, out_len, fill);

    Ok(result)
}

fn pow2(exp: usize) -> usize {
    let mut result = 0;
    for i in 0..exp {
        result *= 2;
    }
    result
}

fn split_rows(vec: Vec<bool>, len: usize) -> Vec<Vec<bool>> {
    let mut rows = Vec::new();
    let mut index = 0;

    while vec.len() != index * len {
        for i in 0..len {
            if i == 0 {
                rows.push(vec![vec[i + len * index]]);
            } else {
                rows[i].push(vec[i + len * index]);
            }
        }
        index += 1;
    }

    rows
}

fn bool2Int(vec: Vec<bool>) -> usize {
    let mut result = 0;
    for i in 0..vec.len() {
        if vec[i] {
            result += pow2(vec.len() - i - 1);
        }
    }
    result
}

fn match_line(vec: Vec<bool>, len: usize) -> (usize, Vec<bool>) {
    let mut temp = Vec::new();

    for i in 0..(vec.len() - len) {
        temp.push(vec[i]);
    }

    let index = bool2Int(temp);
    temp = Vec::new();

    for i in 0..len {
        temp.push(vec[vec.len() - len + i]);
    }

    (index, temp)
}

fn match_table(vec_2d: Vec<Vec<bool>>, len: usize, init: bool) -> Vec<Vec<bool>> {
    let mut result = Vec::new();

    for i in 0..len {
        result.push(vec![init]);
        for _ in 1..pow2(vec_2d[0].len() - len) {
            result[i].push(init);
        }
    }

    for b in vec_2d {
        let (index, vec) = match_line(b, len);
        for i in 0..index {
            result[i][index] = vec[i];
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str2_bool(input: &str) -> Vec<bool> {
        let mut result = Vec::new();
        for c in input.chars() {
            if c == '1' {
                result.push(true);
            } else if c == '0' {
                result.push(false);
            }
        }
        result
    }

    #[test]
    fn test_fill() {
        let mut output = Vec::new();
        output.push(vec![false]);
        println!("{:?}", str2_bool("111000010100"));
        println!(
            "{:?}",
            parse_fill(2, 1, str2_bool("111000010100"), false).unwrap()
        );
        /* assert_eq!(
            parse_fill(2, 1, str2_bool("111000010100"), false).unwrap(),
            output
        );
        */
    }

    #[test]
    fn test_full() {}

    #[test]
    fn test_count() {}
}

/*
vector<TableData> TableParser::getTableDataFill(vector<bool> tableStream, vector<uint32_t> inPins, vector<uint32_t> outPins, bool fill)
{
    /*
    * splits the tableStream in to rows
    * Eaxample tableStream = 111000010100
    * tabel is
    * 11 1
    * 00 0
    * 01 0
    * 10 0
    */
    vector<vector<bool>> temp2D = splitRows(tableStream, outPins.size() + inPins.size());
    /*
    * sorts the rows like in the example above the first row is the last one
    * and completes the table if rows are not defined
    * and removes the first part of the table
    */
    vector<vector<bool>> table2D = match(temp2D, outPins.size(), fill);

    return buildTableData(table2D, inPins, outPins);
}*/
