use clap::builder::PathBufValueParser;
use clap::{Arg, ArgMatches, Command};
use qca_core::analysis::truth_table::generate_truth_table;
use qca_core::objects::cell::QCACellIndex;
use qca_core::simulation::file::{read_from_file, SIMULATION_FILE_EXTENSION};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::str::FromStr;

const THRESHOLD_MIN: f64 = 0.0;
const THRESHOLD_MAX: f64 = 1.0;
const DEFAULT_THRESHOLD: &str = "0.05";
const DEFAULT_VALUE_THRESHOLD: &str = "0.8";

fn validate_threshold(s: &str) -> Result<f64, String> {
    let value = s
        .parse::<f64>()
        .map_err(|_| format!("'{}' is not a valid number", s))?;

    if !(THRESHOLD_MIN..=THRESHOLD_MAX).contains(&value) {
        Err(format!(
            "Value must be between {} and {}, got {}",
            THRESHOLD_MIN, THRESHOLD_MAX, value
        ))
    } else {
        Ok(value)
    }
}

fn validate_cell_delay(s: &str) -> Result<(String, usize), String> {
    let parts: Vec<&str> = s.split(':').collect();

    if parts.len() != 2 {
        return Err("Format must be '<CellIndex|CellLabel>:<ClockDelay>'".to_string());
    }

    let [cell_id, delay_str] = parts.as_slice() else {
        unreachable!("Already checked length is 2")
    };

    let delay = delay_str.parse::<usize>().map_err(|_| {
        format!(
            "Clock delay '{}' is not a valid positive integer",
            delay_str
        )
    })?;

    Ok((cell_id.to_string(), delay))
}

pub fn get_analyze_logic_subcommand() -> Command {
    Command::new("truth")
        .about("Analyze logic analysis on QCA simulation")
        .arg(
            Arg::new("filename")
                .help(format!(
                    "Input .{} filename for analysis",
                    SIMULATION_FILE_EXTENSION
                ))
                .value_parser(PathBufValueParser::default())
                .required(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::new("clock-threshold")
                .help("Clock threshold for analyzing clock zones")
                .long("clock-threshold")
                .short('t')
                .default_value(DEFAULT_THRESHOLD)
                .value_parser(validate_threshold)
                .value_name("THRESHOLD"),
        )
        .arg(
            Arg::new("cell-threshold")
                .help("Cell threshold for analyzing cell state")
                .long("cell-threshold")
                .short('c')
                .default_value(DEFAULT_THRESHOLD)
                .value_parser(validate_threshold)
                .value_name("THRESHOLD"),
        )
        .arg(
            Arg::new("value-threshold")
                .help("Value threshold for analyzing cell value")
                .long("value-threshold")
                .short('v')
                .default_value(DEFAULT_VALUE_THRESHOLD)
                .value_parser(validate_threshold)
                .value_name("THRESHOLD"),
        )
        .arg(
            Arg::new("clock-delay")
                .help("Clock delay for cell outputs (format: <CellIndex|CellLabel>:<ClockDelay>)")
                .long("clock-delay")
                .short('d')
                .value_parser(validate_cell_delay)
                .value_name("<CellIndex|CellLabel>:<ClockDelay>")
                .action(clap::ArgAction::Append), // Allow multiple values
        )
}

pub fn run_analyze_logic(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let input = matches.get_one::<std::path::PathBuf>("filename").unwrap();
    let clock_threshold = *matches.get_one::<f64>("clock-threshold").unwrap();
    let cell_threshold = *matches.get_one::<f64>("cell-threshold").unwrap();
    let value_threshold = *matches.get_one::<f64>("value-threshold").unwrap();

    if !input.exists() {
        return Err(format!("File does not exist: {}", input.display()).into());
    }

    let input_file = File::open(input).unwrap();
    let (design, simulation) = read_from_file(input_file)?;
    let cells = &simulation.metadata.stored_cells;

    let cell_clock_delay: HashMap<QCACellIndex, usize> = matches
        .get_many::<(String, usize)>("clock-delay")
        .map(|vals| {
            vals.cloned()
                .map(|(cell_str, delay)| {
                    // Try to parse as QCACellIndex first
                    let cell_index = QCACellIndex::from_str(&cell_str).or_else(|_| {
                        // If parsing fails, search by label
                        design
                            .layers
                            .iter()
                            .enumerate()
                            .flat_map(|(layer_idx, layer)| {
                                layer
                                    .cells
                                    .iter()
                                    .enumerate()
                                    .map(move |(cell_idx, cell)| (layer_idx, cell_idx, cell))
                            })
                            .find(|(_, _, cell)| {
                                cell.label
                                    .as_ref()
                                    .map_or(false, |label| label == &cell_str)
                            })
                            .map(|(layer, cell, _)| QCACellIndex { layer, cell })
                            .ok_or_else(|| format!("Could not find cell with label '{}'", cell_str))
                    })?;
                    Ok((cell_index, delay))
                })
                .collect::<Result<HashMap<_, _>, String>>()
        })
        .unwrap_or_else(|| Ok(HashMap::new()))?;

    let truth_table = generate_truth_table(
        &design,
        &simulation,
        &cells,
        cell_clock_delay,
        clock_threshold,
        cell_threshold,
        value_threshold,
    );
    println!("{}", truth_table);

    Ok(())
}
