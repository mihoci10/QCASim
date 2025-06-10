use std::error::Error;
use std::fs::File;
use clap::{Arg, ArgMatches, Command};
use clap::builder::PathBufValueParser;
use qca_core::simulation::file::{read_from_file, SIMULATION_FILE_EXTENSION};

pub fn get_analyze_logic_subcommand() -> Command {
    Command::new("truth")
        .about("Analyze logic analysis on QCA simulation")
        .arg(Arg::new("filename")
            .help(format!("Input .{SIMULATION_FILE_EXTENSION} filename for analysis"))
            .value_parser(PathBufValueParser::default())
            .required(true)
        )
}

pub fn run_analyze_logic(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let input = matches.get_one::<std::path::PathBuf>("filename").unwrap();
    if !input.exists() {
        return Err(format!("File does not exist: {}", input.display()).into());
    }
    
    let input_file = File::open(input).unwrap();
    let (design, simulation) = read_from_file(input_file)?;
    
    Ok(())
}