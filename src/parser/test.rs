use super::Token;
use crate::TableData;

use hardware_sim::LookupTable;
use logos::Logos;
use tokenizer::Tokenizer;

#[allow(dead_code)]
fn cmp_token(code: &str, output: Vec<Token>) {
    let mut tokenizer = Tokenizer::new(Token::lexer(code), vec![Token::Ignore((0, None))]);
    let mut input = Vec::<Token>::new();
    while let Some(token) = tokenizer.next() {
        input.push(token);
    }

    for i in 0..input.len() {
        assert_eq!(input[i], output[i], "at token <{}>", i);
    }
}

#[test]
fn ogal2td() {
    let o_gal = super::OGal::new(
        vec![
            ("i0", 1),
            ("i1", 2),
            ("and", 20),
            ("or", 21),
            ("xor", 22),
            ("not", 11),
        ],
        vec![
            LookupTable::new(
                vec![
                    vec![false, false, false, true],
                    vec![false, true, true, true],
                    vec![false, true, true, false],
                ],
                vec!["i0", "i1"],
                vec!["and", "or", "xor"],
                "",
            )
            .unwrap(),
            LookupTable::new(vec![vec![false, true]], vec!["i1"], vec!["not"], "").unwrap(),
        ],
        vec!["and", "not"],
    );

    assert_eq!(
        super::ogal2td(o_gal),
        Ok(vec![
            TableData::new(vec![1, 2], 20, vec![false, false, false, true], true),
            TableData::new(vec![1, 2], 21, vec![false, true, true, true], false),
            TableData::new(vec![1, 2], 22, vec![false, true, true, false], false),
            TableData::new(vec![2], 11, vec![false, true], true)
        ])
    );
}




#[test]
fn test_num() {
    let code = r"
    123 010
    102 2 349645
    1 0 101 11";

    let output = vec![
        Token::Number("123".to_string()),
        Token::Number("010".to_string()),
        Token::Number("102".to_string()),
        Token::Number("2".to_string()),
        Token::Number("349645".to_string()),
        Token::Number("1".to_string()),
        Token::Number("0".to_string()),
        Token::Number("101".to_string()),
        Token::Number("11".to_string()),
    ];

    cmp_token(code, output);
}

#[test]
fn chars() {
    let code = r"
    &|
    ^^!&
    ([
    { ; }])
    .,==.,";

    let output = vec![
        Token::And,
        Token::Or,
        Token::Xor,
        Token::Xor,
        Token::Not,
        Token::And,
        Token::RoundOpen,
        Token::SquareOpen,
        Token::CurlyOpen,
        Token::Semicolon,
        Token::CurlyClose,
        Token::SquareClose,
        Token::RoundClose,
        Token::Dot,
        Token::Comma,
        Token::Equals,
        Token::Equals,
        Token::Dot,
        Token::Comma,
    ];

    cmp_token(code, output);
}

#[test]
fn doc_example() {
    let code = r"
    pin in = 2;
    
    &1010 // comment";

    let output = vec![
        Token::Pin,
        Token::Identifier("in".to_string()),
        Token::Equals,
        Token::Number("2".to_string()),
        Token::Semicolon,
        Token::And,
        Token::Number("1010".to_string()),
    ];

    cmp_token(code, output);
}

#[test]
fn test_arrow() {
    let code = r" ->";

    let mut tokenizer = Tokenizer::new(Token::lexer(code), vec![Token::Ignore((0, None))]);
    assert_eq!(Some(Token::Arrow), tokenizer.next());
}

#[test]
fn test_identifier() {
    let code = r"
    ab ab3
    c_f_g 
    pin table fill.count
    pin1
    dff";

    let output = vec![
        Token::Identifier("ab".to_string()),
        Token::Identifier("ab3".to_string()),
        Token::Identifier("c_f_g".to_string()),
        Token::Pin,
        Token::Table,
        Token::Fill,
        Token::Dot,
        Token::Count,
        Token::Identifier("pin1".to_string()),
        Token::Dff,
    ];

    cmp_token(code, output);
}
#[test]
fn test_comments() {
    let code = r"
    // one line comment
    
    /*
    multi line comment
    */
    3";

    let mut tokenizer = Tokenizer::new(Token::lexer(code), vec![Token::Ignore((0, None))]);
    assert_eq!(Some(Token::Number("3".to_string())), tokenizer.next());
}

#[test]
fn test_parse_func() {
    // code is only for error, it does not influence the test
    let code = "(a|b&d|(c^!1));";
    let output = vec![
        Token::RoundOpen,
        Token::Identifier("a".to_string()),
        Token::Or,
        Token::Identifier("b".to_string()),
        Token::And,
        Token::Identifier("d".to_string()),
        Token::Or,
        Token::RoundOpen,
        Token::Identifier("c".to_string()),
        Token::Xor,
        Token::Not,
        Token::Number("1".to_string()),
        Token::RoundClose,
        Token::RoundClose,
        Token::Semicolon,
    ];

    cmp_token(code, output);

    // (a|b&d|(c^!1))
    let output: Vec<bool_algebra::Token> = vec![
        bool_algebra::Token::Open,
        bool_algebra::Token::Var("a".to_string()),
        bool_algebra::Token::Or,
        bool_algebra::Token::Var("b".to_string()),
        bool_algebra::Token::And,
        bool_algebra::Token::Var("d".to_string()),
        bool_algebra::Token::Or,
        bool_algebra::Token::Open,
        bool_algebra::Token::Var("c".to_string()),
        bool_algebra::Token::Xor,
        bool_algebra::Token::Not,
        bool_algebra::Token::One,
        bool_algebra::Token::Close,
        bool_algebra::Token::Close,
    ];

    // let mut atomizer = Atomizer::new(code, tokens);
    //
    // let input = atomizer.parse_func().unwrap();
    // assert_eq!(input.len(), output.len());
    // for i in 0..input.len() {
    //     assert_eq!(input[i], output[i], "at {}", i);
    // }
}

#[test]
fn identifier() {
    // code is only for error, it does not influence the test
    let code = r"
    a = b & c;
    a.dff;";
    let output = vec![
        Token::Identifier("a".to_string()),
        Token::Equals,
        Token::Identifier("b".to_string()),
        Token::And,
        Token::Identifier("c".to_string()),
        Token::Semicolon,
        Token::Identifier("a".to_string()),
        Token::Dot,
        Token::Dff,
        Token::Semicolon,
    ];

    cmp_token(code, output);

    // let mut atomizer = Atomizer::new(&code, &tokens);
    // let input = atomizer.atomize().unwrap();
    // let output = vec![
    //     Atom::new(
    //         &tokens,
    //         0,
    //         10,
    //         AtomType::BoolFunc {
    //             in_names: vec!["a".to_string()],
    //             func: vec![
    //                 bool_algebra::Token::Var("b".to_string()),
    //                 bool_algebra::Token::And,
    //                 bool_algebra::Token::Var("c".to_string()),
    //             ],
    //         },
    //     ),
    //     Atom::new(
    //         &tokens,
    //         10,
    //         4,
    //         AtomType::Dff {
    //             names: vec!["a".to_string()],
    //         },
    //     ),
    // ];
    //
    // assert_eq!(input.len(), output.len());
    // for i in 0..input.len() {
    //     assert_eq!(input[i], output[i]);
    // }
}

#[test]
fn parse_identifier() {
    // code is only for error, it does not influence the test
    let code = r"
    a, b, c 
    pin[0..3]";

    let output = vec![
        Token::Identifier("a".to_string()),
        Token::Comma,
        Token::Identifier("b".to_string()),
        Token::Comma,
        Token::Identifier("c".to_string()),
        Token::Pin,
        Token::SquareOpen,
        Token::Number("0".to_string()),
        Token::Dot,
        Token::Dot,
        Token::Number("3".to_string()),
        Token::SquareClose,
    ];

    cmp_token(code, output);

    // let mut atomizer = Atomizer::new(&code, &tokens);
    //
    // let mut input = atomizer.parse_identifiers().unwrap();
    // let mut output = vec!["a", "b", "c"];
    // assert_eq!(input.len(), output.len());
    // for i in 0..input.len() {
    //     assert_eq!(input[i], output[i]);
    // }
    //
    // input = atomizer.parse_identifiers().unwrap();
    // output = vec!["pin0", "pin1", "pin2", "pin3"];
    // assert_eq!(input.len(), output.len());
    // for i in 0..input.len() {
    //     assert_eq!(input[i], output[i]);
    // }
}

#[test]
fn test_parse_num() {
    // code is only for error, it does not influence the test
    let code = r"
    1, 2, 3, 10
    [0..3]";

    let output = vec![
        Token::Number("1".to_string()),
        Token::Comma,
        Token::Number("2".to_string()),
        Token::Comma,
        Token::Number("3".to_string()),
        Token::Comma,
        Token::Number("10".to_string()),
        Token::SquareOpen,
        Token::Number("0".to_string()),
        Token::Dot,
        Token::Dot,
        Token::Number("3".to_string()),
        Token::SquareClose,
    ];

    cmp_token(code, output);

    // let mut atomizer = Atomizer::new(&code, &tokens);
    //
    // let mut input = atomizer.parse_num().unwrap();
    // let mut output = vec![1, 2, 3, 10];
    //
    // assert_eq!(input.len(), output.len());
    // for i in 0..input.len() {
    //     assert_eq!(input[i], output[i]);
    // }
    //
    // input = atomizer.parse_num().unwrap();
    // output = vec![0, 1, 2, 3];
    // assert_eq!(input.len(), output.len());
    // for i in 0..input.len() {
    //     assert_eq!(input[i], output[i]);
    // }
}

#[test]
fn test_pin() {
    let code = "pin 3 = a;";
    let output = vec![
        Token::Pin,
        Token::Number("3".to_string()),
        Token::Equals,
        Token::Identifier("a".to_string()),
        Token::Semicolon,
    ];

    cmp_token(code, output);

    // let mut atomizer = Atomizer::new(&code, &tokens);
    // let input = atomizer.atomize().unwrap();
    // assert_eq!(input.len(), 1);
    // assert_eq!(
    //     input[0],
    //     Atom::new(
    //         &tokens,
    //         0,
    //         8,
    //         AtomType::Pin {
    //             pins: vec![3],
    //             names: vec!["a".to_string()]
    //         },
    //     )
    // );
}
#[test]
fn test_table() {
    let code = r"
    table(i0, i1 -> and) {
        00 0
        01 0
        10 0
        11 1
    }";

    let output = vec![
        Token::Table,
        Token::RoundOpen,
        Token::Identifier("i0".to_string()),
        Token::Comma,
        Token::Identifier("i1".to_string()),
        Token::Arrow,
        Token::Identifier("and".to_string()),
        Token::RoundClose,
        Token::CurlyOpen,
        Token::Number("00".to_string()),
        Token::Number("0".to_string()),
        Token::Number("01".to_string()),
        Token::Number("0".to_string()),
        Token::Number("10".to_string()),
        Token::Number("0".to_string()),
        Token::Number("11".to_string()),
        Token::Number("1".to_string()),
        Token::CurlyClose,
    ];

    cmp_token(code, output);

    // let mut atomizer = Atomizer::new(&code, &tokens);
    // let input = atomizer.atomize().unwrap();
    // assert_eq!(input.len(), 1);
    // assert_eq!(
    //     input[0],
    //     Atom::new(
    //         &tokens,
    //         0,
    //         tokens.len() - 1,
    //         AtomType::Table {
    //             in_names: vec!["i0".to_string(), "i1".to_string()],
    //             out_names: vec!["and".to_string()],
    //             table: vec![
    //                 false, false, false, false, true, false, true, false, false, true, true,
    //                 true
    //             ],
    //             table_type: TableType::Full,
    //         },
    //     )
    // );
}
