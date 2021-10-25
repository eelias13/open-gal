mod circuit_config;
mod dnf;
mod fuses;
mod table_data;
mod utils;

pub use circuit_config::CircuitConfig;
pub use dnf::Expression;
pub use table_data::TableData;
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
    let num: u32 = 5;
    println!("{:#064b}", num);
    println!("hello world");
}
