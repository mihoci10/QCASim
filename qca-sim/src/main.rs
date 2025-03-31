use std::env;
use std::fs;

use qca_core::design::*;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Please pass the QCA file name as a parameter!");
        return;
    }

    let filename = &args[1]; 
    println!("Selected file: {}", filename);
    let contents = fs::read_to_string(filename).unwrap();

    let qca_design: qca_core::design::QCADesign = serde_json::from_str(&contents).unwrap();

    dbg!(qca_design);
}
