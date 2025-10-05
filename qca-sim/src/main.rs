use crate::analyze_logic::{get_analyze_logic_subcommand, run_analyze_logic};
use crate::sim::{get_sim_subcommand, run_sim};
use clap::builder::{PathBufValueParser, PossibleValuesParser};
use clap::{Arg, Command};
use qca_core::get_qca_core_version;
use qca_core::objects::log::create_file_logger;
use std::error::Error;
use std::str::FromStr;

mod analyze_logic;
mod sim;

fn main() -> Result<(), Box<dyn Error>> {
    let version = Box::leak(Box::new(get_qca_core_version())).as_str();
    let command = Command::new("qca-sim")
        .version(version)
        .subcommand_required(true)
        .subcommand(get_sim_subcommand())
        .subcommand(get_analyze_logic_subcommand())
        .arg(
            Arg::new("log_file")
                .short('l')
                .long("log-file")
                .help("Filename of the produced logs")
                .value_parser(PathBufValueParser::default())
                .required(false),
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .help("Maximum level of the produced logs")
                .default_value("info")
                .value_parser(PossibleValuesParser::new([
                    "trace", "debug", "info", "warn", "error",
                ]))
                .required(false),
        )
        .arg_required_else_help(true);
    let matches = command.get_matches();

    if let Some(filepath) = matches.get_one::<std::path::PathBuf>("log_file") {
        let logger = create_file_logger(filepath.to_str().unwrap()).unwrap();
        log::set_boxed_logger(Box::new(logger))?;
        let log_level = matches.get_one::<String>("log_level").unwrap();
        log::set_max_level(log::LevelFilter::from_str(log_level.as_str())?);
    }

    match matches.subcommand() {
        Some(("sim", matches)) => run_sim(matches),
        Some(("truth", matches)) => run_analyze_logic(matches),
        _ => Err("Invalid command".into()),
    }
}
