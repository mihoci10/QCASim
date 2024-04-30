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
    fn get_name(&self) -> String;
    fn get_unique_id(&self) -> String;

    fn get_options_list(&self) -> OptionsList;
    fn get_options_value_list(&self) -> OptionsValueList;
    fn set_options_value_list(&mut self, options_value_list: OptionsValueList);

    fn create_instance(&self) -> Box<dyn SimulationModelInstanceTrait>;
}

pub trait SimulationModelInstanceTrait{
    fn initiate(&mut self, cells: Box<Vec<QCACell>>);
    fn pre_calculate(&mut self, clock_states: [f64; 4]);
    fn calculate(&mut self, cell_ind: usize) -> bool;

    fn get_states(&mut self) -> Vec<f64>;
}

pub mod bistable;

pub fn run_simulation(sim_model: &mut Box<dyn SimulationModelTrait>, cells: Vec<QCACell>) -> Box<dyn SimulationModelInstanceTrait>{
    let mut model_inst: Box<dyn SimulationModelInstanceTrait> = sim_model.create_instance();

    model_inst.initiate(Box::new(cells.clone()));
    for _ in 0..10 {
        model_inst.pre_calculate([0.0, 0.0, 0.0, 0.0]);

        let mut stable = false;
        while !stable {
            stable = true;

            for i in 0..cells.len() { 
                stable &= model_inst.calculate(i)
            }
        }
    };

    return model_inst;
}