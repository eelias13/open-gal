mod json_load;

use clap::{App, AppSettings, Arg};
use json_load::*;
use std::fs;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let app = App::new("open-gal")
    .about("open-gal is a hardware description language for generic array logic chips (gal)")
    .version("0.1.0")
    .author("elias <maierelias13@gmail.com>")
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .subcommand(
        App::new("code2td")
            .about("converts the open-gal source code to the intermediate representation table data")
            .args(&[
                Arg::new("open-gal code")
                    .required(true)
                    .about("this is your open-gal sorce code"),
                Arg::new("table data json")
                    .required(true)
                    .about("is an  intermediate representation"),
                Arg::new("gal type")
                    .required(false)
                    .about("the path to your gal type json file"),    
                ]
            ),
    ).subcommand(
        App::new("td2jedec")
            .about("converts the intermediate representation table data to a jedec file")
            .args(&[
                Arg::new("table data json")
                    .required(true)
                    .about("is an  intermediate representation"),
                Arg::new("jedec filename")
                    .required(true)
                    .about("the name of your jedec file"),
                Arg::new("gal type")
                    .required(true)
                    .about("the path to your gal type json file"),    
                ]
            ),
    ).subcommand(
        App::new("code2jedec")
            .about("converts the open-gal source code to a jedec file")
            .args(&[
                Arg::new("open-gal code")
                    .required(true)
                    .about("this is your open-gal sorce code"),
                Arg::new("jedec filename")
                    .required(true)
                    .about("the name of your jedec file"),
                Arg::new("gal type")
                    .required(true)
                    .about("the path to your gal type json file"),
            ])
   );

    let matches = app.get_matches();
    match matches.subcommand() {
        Some(("code2td", arg_m)) => {
            let code_file = match arg_m.value_of("open-gal code") {
                Some(value) => value,
                None => panic!("<open-gal code> is required"),
            };
            let td_name = match arg_m.value_of("table data json") {
                Some(value) => value,
                None => panic!("<table data json> is required"),
            };
            let config_file = arg_m.value_of("gal type");

            match code2td(code_file, td_name, config_file) {
                Ok(()) => (),
                Err(e) => panic!("{}", e),
            }
        }
        Some(("td2jedec", arg_m)) => {
            let td_file = match arg_m.value_of("table data json") {
                Some(value) => value,
                None => panic!("<table data json> is required"),
            };
            let jedec_name = match arg_m.value_of("jedec filename") {
                Some(value) => value,
                None => panic!("<open-gal code> is required"),
            };
            let config_file = match arg_m.value_of("gal type") {
                Some(value) => value,
                None => panic!("<gal type> is required"),
            };

            match td2jedec(td_file, config_file, jedec_name) {
                Ok(()) => (),
                Err(e) => panic!("{}", e),
            }
        }
        Some(("code2jedec", arg_m)) => {
            let code_file = match arg_m.value_of("open-gal code") {
                Some(value) => value,
                None => panic!("<open-gal code> is required"),
            };
            let jedec_name = match arg_m.value_of("jedec filename") {
                Some(value) => value,
                None => panic!("<open-gal code> is required"),
            };
            let config_file = match arg_m.value_of("gal type") {
                Some(value) => value,
                None => panic!("<gal type> is required"),
            };

            match code2jedec(code_file, config_file, jedec_name) {
                Ok(()) => (),
                Err(e) => panic!("{}", e),
            }
        }
        _ => panic!(
            "no handle for subcommand available\n Subcommands are: code2td, td2jedec, code2jedec"
        ),
    }
}

fn code2td(code_file: &str, td_name: &str, config_file: Option<&str>) -> Result<(), String> {
    let mut data = Vec::new();
    let file = match File::open(code_file) {
        Ok(file) => file,
        Err(_) => return Err(format!("unable to read source code files {}", code_file)),
    };
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().into_iter().enumerate() {
        let temp = match line {
            Ok(value) => value.clone(),
            Err(_) => return Err(format!("unable to read line {} in file {}", i, code_file)),
        };
        data.push(temp);
    }

    let table_data = open_gal::parse(data.iter().map(|s| &**s).collect())?;

    if let Some(config_file) = config_file {
        let json = read_json(config_file);
        let config = match circuit_config_from_json(&json) {
            Some(value) => value,
            None => return Err(format!("couldn't read json of file {}", config_file)),
        };

        for td in table_data.clone() {
            td.valid(&config)?;
        }
    }

    let json = td_to_json_vec(&table_data);

    let data = match serde_json::to_string(&json) {
        Ok(value) => value,
        Err(_) => return Err(format!("unable to covert tabel data to json")),
    };

    match fs::write(td_name, data) {
        Ok(()) => Ok(()),
        Err(_) => Err(format!(
            "unable write tabal data file (file name {})",
            td_name
        )),
    }
}

fn td2jedec(td_file: &str, config_file: &str, jedec_name: &str) -> Result<(), String> {
    let json = read_json(td_file);
    let table_data = match td_from_json_vec(&json) {
        Some(value) => value,
        None => return Err(format!("couldn't read json of file {}", td_file)),
    };

    let json = read_json(config_file);
    let config = match circuit_config_from_json(&json) {
        Some(value) => value,
        None => return Err(format!("couldn't read json of file {}", config_file)),
    };

    let jedec = open_gal::to_jedec(&table_data, &config)?;
    match fs::write(jedec_name, jedec) {
        Ok(()) => Ok(()),
        Err(_) => Err(format!(
            "Unable write jedec file (file name {})",
            jedec_name
        )),
    }
}

fn code2jedec(code_file: &str, config_file: &str, jedec_name: &str) -> Result<(), String> {
    let json = read_json(config_file);
    let config = match circuit_config_from_json(&json) {
        Some(value) => value,
        None => return Err(format!("couldn't read json of file {}", config_file)),
    };

    let mut data = Vec::new();
    let file = match File::open(code_file) {
        Ok(file) => file,
        Err(_) => return Err(format!("unable to read source code files {}", code_file)),
    };
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().into_iter().enumerate() {
        let temp = match line {
            Ok(value) => value.clone(),
            Err(_) => return Err(format!("unable to read line {} in file {}", i, code_file)),
        };
        data.push(temp);
    }

    let table_data = open_gal::parse(data.iter().map(|s| &**s).collect::<Vec<_>>())?;

    let jedec = open_gal::to_jedec(&table_data, &config, None)?;
    match fs::write(jedec_name, jedec) {
        Ok(()) => Ok(()),
        Err(_) => Err(format!(
            "Unable write jedec file (file name {})",
            jedec_name
        )),
    }
}
