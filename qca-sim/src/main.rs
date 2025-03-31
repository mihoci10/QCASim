use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use qca_core::design::*;
use qca_core::sim::full_basis::FullBasisModel;
use qca_core::sim::model::SimulationModelTrait;
use qca_core::sim::run_simulation;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Please pass the QCA file name as a parameter!");
        return;
    }

    let filename = &args[1]; 
    println!("Selected file: {}", filename);
    let contents = fs::read_to_string(filename).unwrap();

    let qca_design: QCADesign = serde_json::from_str(&contents).unwrap();

    let mut sim_model: Box<dyn SimulationModelTrait> = Box::new(FullBasisModel::new());
    sim_model.set_serialized_settings(&qca_design.simulation_model_settings.get("full_basis_model").unwrap().to_string())
        .expect("Deserialization failed!");

    let file = Box::new(File::create(format!("{}.bin", "output")).unwrap()) as Box<dyn Write>;

    run_simulation(&mut sim_model, qca_design.layers, qca_design.cell_architectures, Some(file));
}
