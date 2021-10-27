use crate::translator::*;
use crate::{CircuitConfig, TableData};

use super::dnf::Expression;

pub fn to_jedec(truth_tables: &Vec<TableData>, config: &CircuitConfig) -> Result<String, String> {
    let mut exprs = Vec::new();
    for truth_table in truth_tables {
        exprs.push(Expression::new(truth_table, config)?);
    }

    let fuses = fuses::build(&exprs, config)?;

    Ok(jedec::jedec(config.num_pins, config.num_fuses, fuses))
}

#[cfg(test)]
mod tests {
    use super::{CircuitConfig, TableData};

    #[test]
    fn to_jedec() {
        let jedec_res = vec![
            "\x02",
            "Created by open-gal 0.1.0",
            "*QP24",
            "*QF5892",
            "*G0",
            "*F0",
            "*L00032 0000000000011111111111111111111",
            "*L00064 1111111111111111111111111111011",
            "*L00096 1111111111111111111111111111111",
            "*L00128 1111111011110111111111111111111",
            "*L00160 1111111111111110000000000000000",
            "*L02144 0000000000011111111111111111111",
            "*L02176 1111111111111111111111111111111",
            "*L02208 1111111111111111111111111110111",
            "*L02240 0111111111111111111111111111111",
            "*L02272 1111111101101110000000000000000",
            "*L02880 0000000000000000000000011111111",
            "*L02912 1111111111111111111111111111111",
            "*L02944 1111111111111111111111111111111",
            "*L02976 1111111011110111111111111111111",
            "*L03008 1111111111111111111101101111111",
            "*L03040 1111111111111111111111111111111",
            "*L03072 1110111000000000000000000000000",
            "*L03648 0001111111111111111111111111111",
            "*L03680 1111111111111111111111111111111",
            "*L03712 1111111111111111111011101110000",
            "*L05792 0000000000000001000000011111100",
            "*C0000",
            "\x030000",
        ]
        .join("\n");

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
            TableData::new(vec![10, 11], 23, vec![false, false, true, false], true),
            TableData::new(vec![10, 11], 17, vec![false, false, false, true], false),
            TableData::new(vec![10, 11], 19, vec![false, true, true, false], false),
            TableData::new(vec![10, 11], 18, vec![false, true, true, true], false),
            TableData::new(vec![3, 2], 23, vec![true, true, false, true], true),
            TableData::new(vec![3, 2], 23, vec![false, true, true, false], true),
        ];

        assert_eq!(super::to_jedec(&table_data, &config), Ok(jedec_res));
    }
}
