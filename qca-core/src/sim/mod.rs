use std::{cell, f64::consts, io::Write};

use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use self::settings::OptionsList;

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum CellType{
    Normal, Input, Output, Fixed
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
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
    fn pre_calculate(&mut self, clock_states: &[f64; 4], input_states: &Vec<f64>);
    fn calculate(&mut self, cell_ind: usize) -> bool;

    fn get_states(&mut self) -> Vec<f64>;
}

pub mod bistable;
pub mod full_basis;

fn get_clock_values(num_samples: usize, cur_sample: usize, num_inputs: usize, ampl_min: f64, ampl_max: f64, ampl_fac: f64) -> [f64; 4]{
    let prefactor = (ampl_max - ampl_min) * ampl_fac;
    let repetitions = f64::powi(2.0, num_inputs as i32);
    let clock_shift = ampl_max - ampl_min;

    (0..4).map(|i| {
        (prefactor * (repetitions * (cur_sample as f64) * ((2.0 * consts::PI) / num_samples as f64) - (consts::PI * (i as f64) / 2.0)).cos() + clock_shift)
        .clamp(ampl_min, ampl_max)
    }).collect::<Vec<f64>>().try_into().unwrap()
}

fn get_input_values(num_samples: usize, cur_sample: usize, num_inputs: usize) -> Vec<f64>{
    (0..num_inputs).map(|i| {
        (-1.0 * (f64::powi(2.0, i as i32) * cur_sample as f64 * ((2.0 * consts::PI) / num_samples as f64)).sin()).signum()
    }).collect()
}

pub fn run_simulation(sim_model: &mut Box<dyn SimulationModelTrait>, cells: Vec<QCACell>, mut stream: Option<Box<dyn Write>> )
{
    let num_inputs = cells.iter().filter(|c| c.typ == CellType::Input).count();
    let num_outputs = cells.iter().filter(|c| c.typ == CellType::Output).count();
    let model_settings = sim_model.get_settings();

    sim_model.initiate(Box::new(cells.clone()));
    
    if let Some(s) = &mut stream {
        let _ = s.write(&(num_inputs + num_outputs).to_le_bytes());
    }

    for i in 0..model_settings.get_num_samples() {
        let clock_states = get_clock_values(
            model_settings.get_num_samples(), 
            i,
            num_inputs, 
            model_settings.get_clock_ampl_min(),
            model_settings.get_clock_ampl_max(), 
            model_settings.get_clock_ampl_fac()
        );

        let input_states = get_input_values(
            model_settings.get_num_samples(), 
            i,
            num_inputs
        );

        let mut stable = false;
        let mut j = 0;
        while !stable && j < model_settings.get_max_iter()  {
            stable = true;

            sim_model.pre_calculate(
                &clock_states,
                &input_states
            );

            for i in 0..cells.len() { 
                stable &= sim_model.calculate(i)
            }

            j += 1;
        }

        if let Some(s) = &mut stream {
            for f in clock_states.iter(){
                let _ = s.write(&f.to_le_bytes());
            }

            for t in [CellType::Input, CellType::Output]{
                for (_, f) in sim_model.get_states().iter().enumerate()
                                    .filter(|(i, _)| cells.get(*i).unwrap().typ == t)
                {
                    let _ = s.write(&f.to_le_bytes());
                }
            }
        }
    };
}   