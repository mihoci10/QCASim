use std::{cell, f64::consts::{self, PI}, io::Write, ops::Rem};

use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use self::settings::OptionsList;

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum CellType{
    Normal, Input, Output, Fixed
}


#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct QCACellIndex{
    pub layer: usize,
    pub cell: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct QCACell{
    pub position: [f64; 2],
    pub rotation: f64,
    pub typ: CellType,
    pub clock_phase_shift: f64, 
    pub dot_probability_distribution: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QCACellArchitecture{
    pub side_length: f64,
    pub dot_diameter: f64,
    pub dot_count: u8,
    pub dot_positions: Vec<[f64; 2]>,
    pub dot_tunnels: Vec<(u8, u8)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QCALayer{
    pub z_position: f64,
    pub cell_architecture: QCACellArchitecture,
    pub cells: Vec<QCACell>,
}

impl QCACellIndex{
    pub fn new(layer: usize, cell: usize) -> Self{
        QCACellIndex{
            layer: layer,
            cell: cell
        }
    }
}

impl QCALayer{
    pub fn new(z_position: f64, cell_architecture: QCACellArchitecture) -> Self{
        QCALayer{
            z_position: z_position,
            cell_architecture: cell_architecture,
            cells: Vec::<QCACell>::new()
        }
    }
}

impl QCACellArchitecture {
    pub fn new(side_length: f64, dot_diameter: f64, dot_count: u8, dot_radius: f64) -> Self{
        QCACellArchitecture{
            side_length: side_length,
            dot_diameter: dot_diameter,
            dot_count: dot_count,
            dot_positions: (0..dot_count).map(|i| {
                let angle = (2.0 * PI / dot_count as f64) * i as f64;
                [
                    angle.cos() * dot_radius,
                    angle.sin() * dot_radius,
                ]
            }).collect(),
            dot_tunnels: (0..dot_count).map(|i| {
                (
                    (i as i16 - 1).rem_euclid(dot_count as i16) as u8,
                    (i as i16 + 1).rem_euclid(dot_count as i16) as u8,
                )
            }).collect()
        }
    }
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

    fn initiate(&mut self, layers: Box<Vec<QCALayer>>);
    fn pre_calculate(&mut self, clock_states: &[f64; 4], input_states: &Vec<f64>);
    fn calculate(&mut self, cell_index: QCACellIndex) -> bool;

    fn get_states(&self, cell_index: QCACellIndex) -> Vec<f64>;
}

//pub mod bistable;
pub mod full_basis;

fn get_clock_values(num_samples: usize, cur_sample: usize, num_inputs: usize, ampl_min: f64, ampl_max: f64, ampl_fac: f64) -> [f64; 4]{
    // let prefactor = (ampl_max - ampl_min) * ampl_fac;
    // let repetitions = f64::powi(2.0, num_inputs as i32);
    // let clock_shift = ampl_max - ampl_min;

    // (0..4).map(|i| {
    //     (prefactor * (repetitions * (cur_sample as f64) * ((2.0 * consts::PI) / num_samples as f64) - (consts::PI * (i as f64) / 2.0) - consts::PI).cos() + clock_shift)
    //     .clamp(ampl_min, ampl_max)
    // }).collect::<Vec<f64>>().try_into().unwrap()

    (0..4).map(|i| {
        let mut clock = ((cur_sample as f64 / num_samples as f64) - (i as f64 * 0.25));
        clock = clock.rem_euclid(1.0);
        if clock < 0.25{
            (1.0 + ((1.0 - clock / 0.25) * PI).cos()) / 2.0
        }
        else if clock < 0.5 {
            1.0
        }
        else if clock < 0.75 {
            (1.0 + (PI * (clock - 0.5) / 0.25).cos()) / 2.0
        }
        else {
            0.0
        }
    }).map(|v| {
        -((ampl_max - ampl_min) * (1.0 - v) + ampl_min)
    }).collect::<Vec<f64>>().try_into().unwrap()
}

fn get_input_values(num_samples: usize, cur_sample: usize, num_inputs: usize) -> Vec<f64>{
    (0..num_inputs).map(|i| {
        (-1.0 * (f64::powi(2.0, i as i32) * cur_sample as f64 * ((2.0 * consts::PI) / num_samples as f64)).sin()).signum()
    }).collect()
}

pub fn run_simulation(sim_model: &mut Box<dyn SimulationModelTrait>, layers: Vec<QCALayer>, mut stream: Option<Box<dyn Write>> )
{
    //TODO: ugly workaround
    let n: usize = layers[0].cell_architecture.dot_count as usize;
    let num_inputs: usize = layers.iter().map(|layer| layer.cells.iter().filter(|c| c.typ == CellType::Input).count()).sum();
    let num_outputs: usize = layers.iter().map(|layer| layer.cells.iter().filter(|c| c.typ == CellType::Output).count()).sum();
    let model_settings = sim_model.get_settings();

    sim_model.initiate( Box::new(layers.clone()));
    
    if let Some(s) = &mut stream {
        let _ = s.write(&(num_inputs + num_outputs).to_le_bytes());
        let _ = s.write(&(n / 4).to_le_bytes());
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

            for l in 0..layers.len() { 
                for c in 0..layers[l].cells.len() {
                    stable &= sim_model.calculate(QCACellIndex::new(l, c));
                }
            }

            j += 1;
        }

        if let Some(s) = &mut stream {
            for f in clock_states.iter(){
                let _ = s.write(&f.to_le_bytes());
            }

            for t in [CellType::Input, CellType::Output]{
                for l in 0..layers.len() { 
                    for c in 0..layers[l].cells.len() {
                        let cell_index = QCACellIndex::new(l, c);
                        if layers[l].cells[c].typ == t {
                            let distribution = sim_model.get_states(cell_index);
                            for p in 0..n/4 {
                               let val: f64 = 
                                (distribution[p + 0] + distribution[p + (n/2)] - distribution[p + (n/4)] - distribution[p + (n/2) + (n/4)]) 
                                / distribution.iter().sum::<f64>();
                               let _ = s.write(&val.to_le_bytes());
                            }
                        }
                    }
                }
            }
        }
    };
}   