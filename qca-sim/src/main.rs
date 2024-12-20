use std::env;
use std::fs;

use qca_core::sim::*;
use qca_core::sim::bistable::BistableModel;

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

    if let Ok(cells) = serde_json::from_str::<Vec<QCACell>>(&contents) {    
        println!("Running simulation...");
        run_simulation(&mut model, cells, None);

        println!("Simulation results:");
    }
}
