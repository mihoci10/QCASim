use crate::objects::architecture::QCACellArchitecture;
use crate::objects::cell::QCACellIndex;
use crate::objects::layer::QCALayer;
use crate::simulation::settings::OptionsList;
use std::collections::HashMap;

pub trait SimulationModelSettingsTrait {
    fn get_max_iterations(&self) -> usize;
    fn get_convergence_tolerance(&self) -> f64;
}

pub trait ClockGeneratorSettingsTrait {
    fn get_num_cycles(&self) -> usize;
    fn get_amplitude_min(&self) -> f64;
    fn get_amplitude_max(&self) -> f64;
    fn get_extend_last_cycle(&self) -> bool;
    fn get_samples_per_input(&self) -> usize;
}

pub trait SimulationModelTrait: Sync + Send {
    fn get_name(&self) -> String;
    fn get_unique_id(&self) -> String;
    fn get_model_settings(&self) -> Box<dyn SimulationModelSettingsTrait>;
    fn get_clock_generator_settings(&self) -> Box<dyn ClockGeneratorSettingsTrait>;
    fn get_model_options_list(&self) -> OptionsList;
    fn get_clock_generator_options_list(&self) -> OptionsList;
    fn serialize_model_settings(&self) -> Result<String, String>;
    fn deserialize_model_settings(&mut self, settings_str: &String) -> Result<(), String>;
    fn serialize_clock_generator_settings(&self) -> Result<String, String>;
    fn deserialize_clock_generator_settings(&mut self, settings_str: &String)
        -> Result<(), String>;

    fn initiate(
        &mut self,
        layers: Box<Vec<QCALayer>>,
        qca_architetures_map: HashMap<String, QCACellArchitecture>,
    );
    fn pre_calculate(&mut self, clock_states: &[f64; 4], input_states: &Vec<f64>);
    fn calculate(&mut self, cell_index: QCACellIndex) -> bool;

    fn get_states(&self, cell_index: &QCACellIndex) -> Vec<f64>;
}
