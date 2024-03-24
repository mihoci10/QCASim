use serde::{Serialize, Deserialize};

use self::settings::{OptionsList, OptionsValueList};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum CellType{
    Normal, Input, Output, Fixed
}
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum CellClock{
    First, Second, Third, Fourth 
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct QCACell{
    pub pos_x: f64,
    pub pos_y: f64,
    pub typ: CellType,
    pub clock: CellClock,
    pub polarization: f64,
}

pub mod settings;

pub trait SimulationModelTrait{
    fn get_options_list(&self) -> OptionsList;
    fn get_options_value_list(&self) -> OptionsValueList;
    fn set_options_value_list(&mut self, options_value_list: OptionsValueList);

    fn initiate(&mut self, cells: Box<Vec<QCACell>>);
    fn pre_calculate(&mut self, clock_states: [f64; 4]);
    fn calculate(&mut self, cell_ind: usize) -> bool;

    fn get_states(&mut self) -> Vec<f64>;
}

pub mod bistable;

pub struct Simulator{
    sim_model: Box<dyn SimulationModelTrait>,
    cells: Vec<QCACell>,
}

impl Simulator{

    pub fn new(sim_model: Box<dyn SimulationModelTrait>, cells: Vec<QCACell>) -> Simulator{
        Simulator{sim_model: sim_model, cells: cells}
    }

    pub fn run(&mut self){
        self.sim_model.initiate(Box::new(self.cells.clone()));
        for _ in 0..10 {
            self.sim_model.pre_calculate([0.0, 0.0, 0.0, 0.0]);

            let mut stable = false;
			while !stable {
				stable = true;

				for i in 0..self.cells.len() { 
					stable &= self.sim_model.calculate(i)
                }
			}
        }
    }

    pub fn get_results(&mut self) -> Vec<f64> {
        self.sim_model.get_states()
    }

}