mod json_load;

use clap::{Parser, Subcommand};
use json_load::{CircuitConfigWrapper, TableDataWrapper};
use open_gal::{CircuitConfig, TableData};
use std::fs;
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser)]
#[command(
    name = "open-gal",
    version = "0.1.0",
    author = "Elias <maierelias13@gmail.com>",
    about = "open-gal is a hardware description language for generic array logic chips (GAL)",
    subcommand_required = true
)]
struct App {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Converts the open-gal source code to the intermediate representation table data
    Code2Td {
        /// This is your open-gal source code
        code: String,

        /// Is an intermediate representation
        table_data_json: String,

        /// The path to your GAL type JSON file
        gal_type: Option<String>,
    },

    /// Converts the intermediate representation table data to a JEDEC file
    Td2Jedec {
        /// Is an intermediate representation
        table_data_json: String,

        /// The name of your JEDEC file
        jedec_filename: String,

        /// The path to your GAL type JSON file
        gal_type: String,
    },

    /// Converts the open-gal source code to a JEDEC file
    Code2Jedec {
        /// This is your open-gal source code
        code: String,

        /// The name of your JEDEC file
        jedec_filename: String,

        /// The path to your GAL type JSON file
        gal_type: String,
    },
}

fn main() -> Result<(), String> {
    let app = App::parse();

    match app.command {
        Commands::Code2Td {
            code,
            table_data_json,
            gal_type,
        } => code2td(&code, &table_data_json, gal_type.as_deref()),

        Commands::Td2Jedec {
            table_data_json,
            jedec_filename,
            gal_type,
        } => td2jedec(&table_data_json, &gal_type, &jedec_filename),

        Commands::Code2Jedec {
            code,
            jedec_filename,
            gal_type,
        } => code2jedec(&code, &gal_type, &jedec_filename),
    }
}

fn code2td(code_file: &str, td_name: &str, config_file: Option<&str>) -> Result<(), String> {
    let mut file = File::open(code_file)
        .map_err(|_| format!("unable to read source code file {}", code_file))?;
    let mut code = String::new();
    file.read_to_string(&mut code)
        .map_err(|err| format!("{err:?}"))?;

    let table_data = open_gal::parse(&code)?;

    if let Some(config_file) = config_file {
        let config = fs::read_to_string(&config_file)
            .map_err(|_| format!("unable to read file {}", config_file))?;
        let config: CircuitConfig = serde_json::from_str::<CircuitConfigWrapper>(&config)
            .map_err(|err| format!("couldn't read json of file {config_file}. Error: {err:?}",))?
            .try_into()
            .map_err(|err| format!("could not map to struct Error: {err:?}"))?;

        for td in table_data.iter() {
            td.valid(&config)?;
        }
    }

    let mut tds: Vec<TableDataWrapper> = Vec::new();
    for td in table_data {
        let td: TableDataWrapper = TableDataWrapper {
            enable_flip_flop: td.enable_flip_flop,
            input_pins: td.input_pins,
            output_pin: td.output_pin,
            table: td.table,
        };
        tds.push(td);
    }

    let json = serde_json::to_string_pretty(&tds)
        .map_err(|_| format!("unable to covert table data to json"))?;

    fs::write("output.json", json)
        .map_err(|_| format!("unable write table data file (file name {})", td_name))?;

    return Ok(());
}

fn td2jedec(td_file: &str, config_file: &str, jedec_name: &str) -> Result<(), String> {
    let json_data = fs::read_to_string(td_file).map_err(|err| format!("{err:?}"))?;
    let table_data: Vec<TableDataWrapper> = serde_json::from_str(&json_data)
        .map_err(|_| format!("couldn't read json of file {}", td_file))?;

    let config = fs::read_to_string(&config_file)
        .map_err(|_| format!("unable to read file {}", config_file))?;
    let config: CircuitConfig = serde_json::from_str::<CircuitConfigWrapper>(&config)
        .map_err(|err| format!("couldn't read json of file {config_file}. Error: {err:?}",))?
        .try_into()
        .map_err(|err| format!("could not map to struct Error: {err:?}"))?;

    let mut truth_tables = Vec::new();

    for td in table_data {
        let td: TableData = td.try_into().map_err(|err| format!("{err:?}"))?;
        truth_tables.push(td);
    }

    let jedec = open_gal::to_jedec(&truth_tables, &config, None)?;

    fs::write(jedec_name, jedec)
        .map_err(|_| format!("Unable write jedec file (file name {})", jedec_name))?;

    return Ok(());
}

fn code2jedec(code_file: &str, config_file: &str, jedec_name: &str) -> Result<(), String> {
    let config = fs::read_to_string(&config_file)
        .map_err(|_| format!("unable to read file {}", config_file))?;
    let config: CircuitConfig = serde_json::from_str::<CircuitConfigWrapper>(&config)
        .map_err(|err| format!("couldn't read json of file {config_file}. Error: {err:?}",))?
        .try_into()
        .map_err(|err| format!("could not map to struct Error: {err:?}"))?;

    let mut file = File::open(code_file)
        .map_err(|_| format!("unable to read source code file {}", code_file))?;
    let mut code = String::new();
    file.read_to_string(&mut code)
        .map_err(|err| format!("{err:?}"))?;

    let table_data = open_gal::parse(&code)?;

    let jedec = open_gal::to_jedec(&table_data, &config, None)?;
    match fs::write(jedec_name, jedec) {
        Ok(()) => Ok(()),
        Err(_) => Err(format!(
            "Unable write jedec file (file name {})",
            jedec_name
        )),
    }
}
