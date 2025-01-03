mod tests {
    use open_gal::{to_wincupl, TableData};
    #[test]
    fn wincupl() {
        let head = vec![
            "Name     template ; ",
            "PartNo   00 ; ",
            "Date     10/10/10 ;",
            "Revision 01 ;",
            "Designer Engineer ;",
            "Company  None ;",
            "Assembly None ; ",
            "Location  ;",
            "Device   v750c ;",
        ];
        let table_data = vec![TableData {
            input_pins: vec![1, 2],
            output_pin: 23,
            table: vec![false, false, false, true],
            enable_flip_flop: false,
        }];

        let mut out_vec = Vec::new();

        to_wincupl(&table_data, Some(&head.clone().join("\n")))
            .lines()
            .for_each(|x| out_vec.push(x.to_string()));

        let out_vec: Vec<&str> = out_vec.iter().map(|s| &**s).collect();
        let code = vec![
            "Name     template ; ",
            "PartNo   00 ; ",
            "Date     10/10/10 ;",
            "Revision 01 ;",
            "Designer Engineer ;",
            "Company  None ;",
            "Assembly None ; ",
            "Location  ;",
            "Device   v750c ;",
            "Pin 1 = in_1p;",
            "Pin 2 = in_2p;",
            "Pin 23 = out_23p;",
            "",
            "",
            "Field in_1204071043837228674f = [in_1p, in_2p];",
            "Field out_1204071043837228674f = out_23p;",
            "Table in_1204071043837228674f => out_1204071043837228674f {",
            "  'b'00 => 'b'0;",
            "  'b'01 => 'b'0;",
            "  'b'10 => 'b'0;",
            "  'b'11 => 'b'1;",
            "}",
            "",
        ];

        assert_eq!(out_vec.len(), code.len());
        for line in 0..out_vec.len() {
            assert_eq!(out_vec[line], code[line]);
            println!("{}", out_vec[line]);
        }
    }
}

/*
Name     Name ;
PartNo   00 ;
Date     28/10/2021 ;
Revision 01 ;
Designer Engineer ;
Company  Atmel-WinCUPL ;
Assembly None ;
Location  ;
Device   g22v10 ;
// Device   v750c ;

Pin 1 = in_1;
Pin 2 = in_2;
Pin 23 = out_23;


Field in_1204071043837228674x = [in_1, in_2];
Field out_1204071043837228674x = out_23;
Table in_1204071043837228674x => out_1204071043837228674x {
  'b'00 => 'b'0;
  'b'01 => 'b'0;
  'b'10 => 'b'0;
  'b'11 => 'b'1;
}
*/

/*
Pin 1 = a;
Pin 2 = b;
Pin 23 = c;

c = a & b;
*/
