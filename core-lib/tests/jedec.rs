#[cfg(test)]
mod tests {
    use open_gal::{to_jedec, CircuitConfig, TableData};

    #[test]
    #[ignore]
    fn jedec() {
        let jedec_res = vec![
            "\x02",
            "Created by open-gal 0.1.0",
            "*QP24",
            "*QF5892",
            "*G0",
            "*F0",
            "*L00032 00000000000011111111111111111111",
            "*L00064 11111111111111111111111111111011",
            "*L00096 10111111111111111111111111111111",
            "*L00128 11111111011110111111111111111111",
            "*L00160 11111111111111111111011101111111",
            "*L00192 11111111111111111111111111110000",
            "*L02144 00000000000011111111111111111111",
            "*L02176 11111111111111111111111111111111",
            "*L02208 11111111111111111111111111111111",
            "*L02240 01101111111111111111111111111111",
            "*L02272 11111111111110010000000000000000",
            "*L02880 00000000000000000000000011111111",
            "*L02912 11111111111111111111111111111111",
            "*L02944 11111111111111111111111111111111",
            "*L02976 11111111111101101111111111111111",
            "*L03008 11111111111111111111111110011111",
            "*L03040 11111111111111111111111111111111",
            "*L03072 11110101000000000000000000000000",
            "*L03648 00001111111111111111111111111111",
            "*L03680 11111111111111111111111111111111",
            "*L03712 11111111111111111111111101010000",
            "*L05792 00000000000000001000000011111100",
            "*C4641",
            "\x030000",
        ];

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

        let jedec = to_jedec(&table_data, &config, None).unwrap();
        for (i, line) in jedec.lines().clone().into_iter().enumerate() {
            assert_eq!(line, jedec_res[i]);
        }
    }

    #[test]
    #[ignore]
    fn wincuple_simple() {
        let table_data = vec![TableData {
            input_pins: vec![1, 2],
            output_pin: 23,
            table: vec![false, false, false, true],
            enable_flip_flop: false,
        }];

        let head = vec![
            "CUPL(WM)        5.0a  Serial# 60008009",
            "Device          g22v10  Library DLIB-h-40-1",
            "Created         Thu Oct 28 13:35:25 2021",
            "Name            Name",
            "Partno          00",
            "Revision        01",
            "Date            28/10/2021",
            "Designer        Engineer",
            "Company         None",
            "Assembly        None",
            "Location",
            "",
        ]
        .join("\n");

        let wc_jedec = vec![
            "*QP24",
            "*QF5892",
            "*G0",
            "*F0",
            "*L00032 00000000000011111111111111111111",
            "*L00064 11111111111111111111111101110111",
            "*L00096 11111111111111111111111111111111",
            "*L00128 11110000000000000000000000000000",
            "*L05792 00000000000000001100000000000000",
            "*L05824 00000011000000110000001000000000",
            "*C0CA7",
            "*\x03800B",
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

        assert_eq!(
            to_jedec(&table_data, &config, Some(head.clone())),
            Ok(format!("\x02{}{}", head, wc_jedec))
        );
    }
}
