use std::collections::HashMap;
use crate::objects::architecture::QCACellArchitecture;
use crate::objects::cell::{QCACellIndex};
use crate::objects::layer::QCALayer;
use crate::simulation::settings::OptionsList;

pub trait SimulationModelSettingsTrait{
    fn get_num_samples(&self) -> usize;
    fn get_clock_ampl_min(&self) -> f64;
    fn get_clock_ampl_max(&self) -> f64;
    fn get_clock_ampl_fac(&self) -> f64;
    fn get_max_iter(&self) -> usize;
}

pub trait SimulationModelTrait: Sync + Send{
    fn get_name(&self) -> String;
    fn get_unique_id(&self) -> String;

    fn get_settings(&self) -> Box<dyn SimulationModelSettingsTrait>;

    fn get_options_list(&self) -> OptionsList;
    fn get_deserialized_settings(&self) -> Result<String, String>;
    fn set_serialized_settings(&mut self, settings_str: &String) -> Result<(), String>;

    fn initiate(&mut self, layers: Box<Vec<QCALayer>>, qca_architetures_map: HashMap<String, QCACellArchitecture>);
    fn pre_calculate(&mut self, clock_states: &[f64; 4], input_states: &Vec<f64>);
    fn calculate(&mut self, cell_index: QCACellIndex) -> bool;

    fn get_states(&self, cell_index: QCACellIndex) -> Vec<f64>;
}