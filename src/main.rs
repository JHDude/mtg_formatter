extern crate clap;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::PathBuf;

use clap::{App, Arg};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MtgRecord {
    #[serde(rename = "QuantityX")]
    quantity: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Edition code")]
    edition: String,
    #[serde(rename = "Foil")]
    foil: Option<String>,
}

impl MtgRecord {
    fn archidekt_txt(&self, cmdr_name: Option<&str>) -> String {
        let foil = String::from(match self.foil {
            Some(_) => " *F*",
            _ => "",
        });
        match cmdr_name {
            Some(name) => {
                if name == self.name {
                    format!(
                        "{} {} ({}){} {}",
                        self.quantity, self.name, self.edition, foil, "[Commander{top}]"
                    )
                } else {
                    format!("{} {} ({}){}", self.quantity, self.name, self.edition, foil)
                }
            }
            None => format!("{} {} ({}){}", self.quantity, self.name, self.edition, foil),
        }
    }
}

fn has_header(path: &PathBuf) -> Result<bool, Box<dyn Error>> {
    let file = File::open(path)?;
    let first_line = io::BufReader::new(file).lines().into_iter().next();
    match first_line {
        Some(x) => match x {
            Ok(y) => {
                if y == "QuantityX,Name,Edition code,Foil" {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }
            Err(parsing_error) => return Err(Box::new(parsing_error)),
        },
        None => return Ok(false),
    }
}

// Function for parsing a CSV created by DelverLens, and converting it into a Vector of MtgRecords
fn parse_csv(path: &PathBuf) -> Result<Vec<MtgRecord>, Box<dyn Error>> {
    // let mut reader = csv::Reader::from_path(path)?;
    let mut reader = if has_header(&path)? {
        csv::Reader::from_path(path)?
    } else {
        csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(path)?
    };
    let mut output = Vec::<MtgRecord>::new();
    for result in reader.deserialize() {
        let record = result?;
        output.push(record);
    }
    Ok(output)
}

// Create a Vector of Strings for later writing operations
fn format_records(records: Vec<MtgRecord>, commander_name: Option<&str>) -> Vec<String> {
    records
        .into_iter()
        .map(|x| x.archidekt_txt(commander_name))
        .collect::<Vec<String>>()
}

// Write formatted Strings to specified output file
fn write_formatted_records(path: PathBuf, records: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut buffer_out = File::create(path)?;
    match write!(buffer_out, "{}", records.join("\n")) {
        Ok(()) => Ok(()),
        Err(x) => Err(Box::new(x)),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("mtg_formatter")
                            .version("0.1")
                            .author("Jacob Hassold <jhdude3@gmail.com>")
                            .about("A simple command-line based utility to convert DelverLens CSV exports to the Archidekt import text format")
                            .arg(Arg::with_name("path")
                                .value_name("PATH")
                                .help("The path of the csv file to convert")
                                .required(true)
                                .takes_value(true))
                            .arg(Arg::with_name("commander_name")
                                .short("c")
                                .required(false)
                                .value_name("COMMANDER_NAME")
                                .help("Name of the card that is the commander of the deck. If omitted or misspelled, no card is labeled commander")
                                .takes_value(true))
                            .get_matches();
    let path = matches.value_of("path").unwrap(); // Unwrap is safe here, as this is a required parameter
    let commander_name = matches.value_of("commander_name");
    let mut out_path = PathBuf::from(path);
    let in_path = PathBuf::from(path);

    // Attempt to set the extension for the output file
    let extension_set_success = out_path.set_extension("txt");
    if !extension_set_success {
        panic!("Passed-in path was not a file!");
    };

    let parse_result = parse_csv(&in_path)?;
    let formatted_results = format_records(parse_result, commander_name);
    let final_result = write_formatted_records(out_path, formatted_results);
    match final_result {
        Ok(()) => {
            println!("Converted successfully! But be sure to check for translation issues when importing!");
            Ok(())
        }
        Err(x) => Err(x),
    }
}
