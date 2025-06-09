use std::error::Error;
use std::fs;
use std::fs::File;
use clap::{Arg, ArgMatches, Command};
use clap::builder::PathBufValueParser;
use indicatif::{ProgressBar, ProgressStyle};
use qca_core::design::file::{QCADesignFile, DESIGN_FILE_EXTENSION};
use qca_core::simulation::full_basis::FullBasisModel;
use qca_core::simulation::model::SimulationModelTrait;
use qca_core::simulation::{run_simulation_async, SimulationProgress};
use qca_core::simulation::file::{write_to_file, SIMULATION_FILE_EXTENSION};

pub fn get_sim_subcommand() -> Command {
    Command::new("sim")
        .about("Run the QCA simulation")
        .arg(Arg::new("filename")
            .help(format!("Input .{DESIGN_FILE_EXTENSION} filename for simulating"))
            .value_parser(PathBufValueParser::default())
            .required(true)
        )
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .help(format!("Output .{SIMULATION_FILE_EXTENSION} filename for simulation results"))
            .value_parser(PathBufValueParser::default())
            .required(false)
        )
}
pub fn run_sim(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let input = matches.get_one::<std::path::PathBuf>("filename").unwrap();
    let output =
        if let Some(output) = matches.get_one::<std::path::PathBuf>("output") {
            output
        } else{
            &input.with_extension(SIMULATION_FILE_EXTENSION)
        };

    if !input.exists() {
        return Err(format!("File does not exist: {}", input.display()).into());
    }

    let contents = fs::read_to_string(input).unwrap();

    let qca_design_file: QCADesignFile = serde_json::from_str(&contents).unwrap();
    let qca_design = qca_design_file.design;

    let mut sim_model: Box<dyn SimulationModelTrait> = Box::new(FullBasisModel::new());
    sim_model.set_serialized_settings(&qca_design.simulation_model_settings.get("full_basis_model").unwrap().to_string())?;
    let max_samples = sim_model.get_settings().get_num_samples() as u64;

    let (handle, progress_rx, _cancel_tx) = run_simulation_async(sim_model, qca_design.layers.clone(), qca_design.cell_architectures.clone());

    let progress_bar = ProgressBar::new(max_samples);
    progress_bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.bold.green} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} iterations (ETA: {eta})"
        ).unwrap()
    );
    progress_bar.set_message("Running simulation");

    for progress in progress_rx{
        match progress{
            SimulationProgress::Initializing => progress_bar.set_position(0),
            SimulationProgress::Running {..} => progress_bar.inc(1),
            SimulationProgress::Deinitializng => progress_bar.set_position(max_samples)
        }
    }
    let simulation_data = handle.join().unwrap();

    progress_bar.set_message("Writing to file");

    let file = File::create(output).unwrap();
    write_to_file(file, &qca_design, &simulation_data)?;

    progress_bar.finish_with_message("Simulation finished");
    println!("Simulation written to: {}", output.to_str().unwrap());

    Ok(())
}