mod tests {
    use open_gal::*;

    #[test]
    fn easy_gal() {
        let data = vec![
            "pin 13 = i0;",
            "pin 11 = i1;",
            "pin 17 = and;",
            "pin 18 = or;",
            "pin 19 = xor;",
            "",
            "table(i0, i1 -> and) {",
            "    00 0",
            "    01 0",
            "    10 0",
            "    11 1",
            "}",
            "",
            "table(i0, i1 -> xor).count {",
            "    0",
            "    1",
            "    1",
            "    0",
            "}",
            "",
            "table(i0, i1 -> or).fill(1) {",
            "    00 0",
            "    01 1",
            "    10 1",
            "}",
            "",
            "pin 23 = a;",
            "pin 3 = b;",
            "pin 2 = c;",
            "",
            "a = (!b | (c));",
            "a.dff;",
        ];

        let input = parse(data).unwrap();
        let output = vec![
            TableData {
                input_pins: vec![13, 11],
                output_pin: 17,
                table: vec![false, false, false, true],
                enable_flip_flop: false,
            },
            TableData {
                input_pins: vec![13, 11],
                output_pin: 19,
                table: vec![false, true, true, false],
                enable_flip_flop: false,
            },
            TableData {
                input_pins: vec![13, 11],
                output_pin: 18,
                table: vec![false, true, true, true],
                enable_flip_flop: false,
            },
            TableData {
                input_pins: vec![3, 2],
                output_pin: 23,
                table: vec![true, true, false, true],
                enable_flip_flop: true,
            },
        ];

        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i], "at {}", i);
        }
    }

    #[test]
    fn open_gal() {
        let data = vec![
            "pin 1, 2 = i[0..1];",
            "pin [13..16] = and, or, xor, not;",
            "table(i0, i1 -> and).fill(0) {",
            "    11 1",
            "}",
            "",
            "table(i0, i1 -> or).fill(1) {",
            "    00 0",
            "}",
            "",
            "table(i0, i1 -> xor ).count {",
            "    0",
            "    1",
            "    1",
            "    0",
            "}",
            "",
            "table(i0 -> not) {",
            "    01",
            "    10",
            "}",
        ];
        let input = parse(data).unwrap();
        let output = vec![
            TableData {
                input_pins: vec![1, 2],
                output_pin: 13,
                table: vec![false, false, false, true],
                enable_flip_flop: false,
            },
            TableData {
                input_pins: vec![1, 2],
                output_pin: 14,
                table: vec![false, true, true, true],
                enable_flip_flop: false,
            },
            TableData {
                input_pins: vec![1, 2],
                output_pin: 15,
                table: vec![false, true, true, false],
                enable_flip_flop: false,
            },
            TableData {
                input_pins: vec![1],
                output_pin: 16,
                table: vec![true, false],
                enable_flip_flop: false,
            },
        ];

        assert_eq!(input.len(), output.len());
        for i in 0..input.len() {
            assert_eq!(input[i], output[i], "at {}", i);
        }
    }
}
