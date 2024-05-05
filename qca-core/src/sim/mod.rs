use std::f64::consts;

use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use self::settings::OptionsList;

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum CellType{
    Normal, Input, Output, Fixed
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct QCACell{
    pub pos_x: f64,
    pub pos_y: f64,
    pub z_index: i32,
    pub typ: CellType,
    pub clock_phase_shift: f64,
    pub polarization: f64,
}

pub mod settings;

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

    fn initiate(&mut self, cells: Box<Vec<QCACell>>);
    fn pre_calculate(&mut self, clock_states: [f64; 4]);
    fn calculate(&mut self, cell_ind: usize) -> bool;

    fn get_states(&mut self) -> Vec<f64>;
}

pub mod bistable;

fn generate_clock_signal(num_samples: usize, num_inputs: usize, phase_shift_deg: f64, ampl_min: f64, ampl_max: f64, ampl_fac: f64) -> Vec<f64>{
    let prefactor = (ampl_max - ampl_min) * ampl_fac;
    let repetitions = f64::powi(2.0, num_inputs as i32);
    let clock_shift = ampl_max - ampl_min;
    let phase_shift_rad = phase_shift_deg.to_radians();

    (0..num_samples).map(|i| {
        (prefactor * (repetitions * (i as f64) * ((2.0 * consts::PI) / num_samples as f64) - phase_shift_rad).cos() + clock_shift)
        .clamp(ampl_min, ampl_max)
    }).collect()
}

pub fn run_simulation(sim_model: &mut Box<dyn SimulationModelTrait>, cells: Vec<QCACell>){
    let num_inputs = cells.iter().filter(|c| c.typ == CellType::Input).count();
    let model_settings = sim_model.get_settings();

    let clock_arrays: [Vec<f64>; 4] = [
        generate_clock_signal(
            model_settings.get_num_samples(), 
            num_inputs, 
            0.0, 
            model_settings.get_clock_ampl_min(), 
            model_settings.get_clock_ampl_max(), 
            model_settings.get_clock_ampl_fac()
        ),
        generate_clock_signal(
            model_settings.get_num_samples(), 
            num_inputs, 
            90.0, 
            model_settings.get_clock_ampl_min(), 
            model_settings.get_clock_ampl_max(), 
            model_settings.get_clock_ampl_fac()
        ),
        generate_clock_signal(
            model_settings.get_num_samples(), 
            num_inputs, 
            180.0, 
            model_settings.get_clock_ampl_min(), 
            model_settings.get_clock_ampl_max(), 
            model_settings.get_clock_ampl_fac()
        ),
        generate_clock_signal(
            model_settings.get_num_samples(), 
            num_inputs, 
            270.0, 
            model_settings.get_clock_ampl_min(), 
            model_settings.get_clock_ampl_max(), 
            model_settings.get_clock_ampl_fac()
        ),
    ];

    sim_model.initiate(Box::new(cells.clone()));

    for i in 0..model_settings.get_num_samples() {
        sim_model.pre_calculate(
            [
                *clock_arrays[0].get(i as usize).unwrap(), 
                *clock_arrays[1].get(i as usize).unwrap(), 
                *clock_arrays[2].get(i as usize).unwrap(), 
                *clock_arrays[3].get(i as usize).unwrap()
            ]
        );

        let mut stable = false;
        let mut j = 0;
        while !stable && j < model_settings.get_max_iter()  {
            stable = true;

            for i in 0..cells.len() { 
                stable &= sim_model.calculate(i)
            }

            j += 1;
        }
    };
}