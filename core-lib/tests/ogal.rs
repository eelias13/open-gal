use hardware_sim::LookupTable;
use open_gal::OGal;

#[test]

fn pin() {
    let code = r"
    pin 11 = not;
    pin 1, 2 = i[0..1];
    pin [20..22] = and, or, xor;";

    let o_gal = OGal::new(
        vec![
            ("i0", 1),
            ("i1", 2),
            ("and", 20),
            ("or", 21),
            ("xor", 22),
            ("not", 11),
        ],
        Vec::new(),
        Vec::new(),
    );

    assert_eq!(Ok(o_gal), OGal::parse(code));
}

#[test]
fn dff() {
    let code = r"
    a.dff;
    i[0..2].dff;
    c,d.dff;";

    let o_gal = OGal::new(
        Vec::new(),
        Vec::new(),
        vec!["a", "i0", "i1", "i2", "c", "d"],
    );

    assert_eq!(Ok(o_gal), OGal::parse(code));
}

#[test]
fn func() {
    let code = r"
    and = i0 & i1;
    or  = i0 | i1;
    xor = i0 ^ i1;";

    let o_gal = OGal::new(
        Vec::new(),
        vec![
            LookupTable::new(
                vec![vec![false, false, false, true]],
                vec!["i0", "i1"],
                vec!["and"],
                "",
            )
            .unwrap(),
            LookupTable::new(
                vec![vec![false, true, true, true]],
                vec!["i0", "i1"],
                vec!["or"],
                "",
            )
            .unwrap(),
            LookupTable::new(
                vec![vec![false, true, true, false]],
                vec!["i0", "i1"],
                vec!["xor"],
                "",
            )
            .unwrap(),
        ],
        Vec::new(),
    );

    assert_eq!(Ok(o_gal), OGal::parse(code));
}

#[test]
fn table_full() {
    let code = r"
    table(i0, i1 -> and, or, xor) {
        00 000
        01 011
        10 011
        11 110
    }";

    let o_gal = OGal::new(
        Vec::new(),
        vec![LookupTable::new(
            vec![
                vec![false, false, false, true],
                vec![false, true, true, true],
                vec![false, true, true, false],
            ],
            vec!["i0", "i1"],
            vec!["and", "or", "xor"],
            "",
        )
        .unwrap()],
        Vec::new(),
    );

    assert_eq!(Ok(o_gal), OGal::parse(code));
}

#[test]
fn table_count() {
    let code = r"
    table(i0, i1 -> and, or, xor).count {
        000
        011
        011
        110
    }";

    let o_gal = OGal::new(
        Vec::new(),
        vec![LookupTable::new(
            vec![
                vec![false, false, false, true],
                vec![false, true, true, true],
                vec![false, true, true, false],
            ],
            vec!["i0", "i1"],
            vec!["and", "or", "xor"],
            "",
        )
        .unwrap()],
        Vec::new(),
    );

    assert_eq!(Ok(o_gal), OGal::parse(code));
}

#[test]
fn table_fill() {
    let code = r"
    table(i0, i1 -> and, or, xor).fill(0) {
        01 011
        10 011
        11 110
    }";

    let o_gal = OGal::new(
        Vec::new(),
        vec![LookupTable::new(
            vec![
                vec![false, false, false, true],
                vec![false, true, true, true],
                vec![false, true, true, false],
            ],
            vec!["i0", "i1"],
            vec!["and", "or", "xor"],
            "",
        )
        .unwrap()],
        Vec::new(),
    );

    assert_eq!(Ok(o_gal), OGal::parse(code));
}
