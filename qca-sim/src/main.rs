use std::error::Error;
use clap::Command;
use qca_core::get_qca_core_version;
use crate::sim::{get_sim_subcommand, run_sim};

mod sim;

fn main() -> Result<(), Box<dyn Error>> {
    let version = Box::leak(Box::new(get_qca_core_version())).as_str();
    let command = Command::new("qca-sim")
        .version(version)
        .subcommand_required(true)
        .subcommand(get_sim_subcommand());
    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("sim", matches)) => {
            return run_sim(matches);
        }
        Some(("analyze", matches)) => {

        }
        _ => panic!("Invalid command"),
    }   
    
    Ok(())
}