use crate::analyze_logic::{get_analyze_logic_subcommand, run_analyze_logic};
use crate::sim::{get_sim_subcommand, run_sim};
use clap::Command;
use qca_core::get_qca_core_version;
use std::error::Error;

mod analyze_logic;
mod sim;

fn main() -> Result<(), Box<dyn Error>> {
    let version = Box::leak(Box::new(get_qca_core_version())).as_str();
    let command = Command::new("qca-sim")
        .version(version)
        .subcommand_required(true)
        .subcommand(get_sim_subcommand())
        .subcommand(get_analyze_logic_subcommand());
    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("sim", matches)) => run_sim(matches),
        Some(("truth", matches)) => run_analyze_logic(matches),
        _ => Err("Invalid command".into()),
    }
}
