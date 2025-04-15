use std::env;
use std::fs;
use qca_core::design::file::QCADesign;
use qca_core::simulation::full_basis::FullBasisModel;
use qca_core::simulation::model::SimulationModelTrait;
use qca_core::simulation::{run_simulation_async, SimulationProgress};
use qca_core::simulation::file::{SIMULATION_FILE_EXTENSION, write_to_file};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Please pass a .qcd file path as a parameter!");
        return;
    }

    let filename = &args[1]; 
    println!("Selected file: {}", filename);
    let contents = fs::read_to_string(filename).unwrap();

    let qca_design: QCADesign = serde_json::from_str(&contents).unwrap();

    let mut sim_model: Box<dyn SimulationModelTrait> = Box::new(FullBasisModel::new());
    sim_model.set_serialized_settings(&qca_design.simulation_model_settings.get("full_basis_model").unwrap().to_string())
        .expect("Deserialization failed!");

    let (handle, progress_rx, cancel_tx) = run_simulation_async(sim_model, qca_design.layers, qca_design.cell_architectures);

    for progress in progress_rx{
        match progress{
            SimulationProgress::Initializing => println!("Initializing"),
            SimulationProgress::Running { current_sample, total_samples } => {println!("\rSample {} of {}", current_sample, total_samples)}
            SimulationProgress::Deinitializng => {println!("Finishing")}
        }
    }

    let simulation_data = handle.join().unwrap();

    write_to_file(format!("output.{}", SIMULATION_FILE_EXTENSION).as_str(), &serde_json::from_str(&contents).unwrap(), &simulation_data).unwrap();
}