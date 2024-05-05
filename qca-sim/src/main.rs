use std::env;
use std::fs;

use qca_core::sim::*;
use qca_core::sim::bistable::BistableModel;
use qca_core::datafile::*;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Please pass the QCA file name as a parameter!");
        return;
    }

    let filename = &args[1]; 
    println!("Selected file: {}", filename);
    let contents = fs::read_to_string(filename).unwrap();

    let mut model: Box<dyn SimulationModelTrait> = Box::new(BistableModel::new());
    let cells = cells_deserialize(&contents);
    
    println!("Running simulation...");
    run_simulation(&mut model, cells);

    println!("Simulation results:");
}
