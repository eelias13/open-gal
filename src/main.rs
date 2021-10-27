mod circuit_config;
mod table_data;
pub mod translator;

pub use circuit_config::CircuitConfig;
pub use table_data::TableData;
pub use translator::core::to_jedec;

#[cfg(cli)]
pub use utils::read_json;

#[cfg(cli)]
use clap::{App, AppSettings, Arg};

#[cfg(cli)]
fn main() {
    let app = App::new("open-gal")
        .about("open-gal is a hdl for gals")
        .version("0.1.0")
        .author("elias <maierelias13@gmail.com>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("api")
                .about("(like `cargo new`) creats a rust project with a specific folder structur")
                .arg(
                    Arg::new("project_name")
                        .required(true)
                        .about("the name of your new project"),
                ),
        );

    let matches = app.get_matches();
    match matches.subcommand() {}
    let table_data = get_table_data("./tableData.json").unwrap();
    let json = to_json(&table_data);
    let circuit_config = CircuitConfig::load("./Configs/g22v10.json").unwrap();
    println!("table_data: {:?}", table_data);
    println!("json: {}", json);
}

#[cfg(not(cli))]
fn main() {
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

    print!("{}", to_jedec(&table_data, &config).unwrap());
}
