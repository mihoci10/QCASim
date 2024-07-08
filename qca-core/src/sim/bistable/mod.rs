use std::{cell, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

use crate::sim::settings::{InputDescriptor, OptionsEntry};

use super::{CellType, QCACell, SimulationModelSettingsTrait, SimulationModelTrait};

pub struct BistableModel {
    clock_states: [f64; 4],
    input_states: Vec<f64>,
    cells: Box<Vec<super::QCACell>>,
    cell_input_map: HashMap<usize, usize>,
    active_layer: i8,
    polarizations: [Vec<f64>; 2],
    neighbor_indecies: Vec<Vec<usize>>,
    neighbour_kink_energy: Vec<Vec<f64>>,
    settings: BistableModelSettings
}


#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone)]
pub struct BistableModelSettings{
    #[serde_inline_default(1000)]
    num_samples: usize,

    #[serde_inline_default(100)]
    max_iter: usize,

    #[serde_inline_default(3.8e-23)]
    ampl_min: f64,
    
    #[serde_inline_default(9.8e-22)]
    ampl_max: f64,

    #[serde_inline_default(2.0)]
    ampl_fac: f64,

    #[serde_inline_default(1e-3)]
    convergence_tolerance: f64,

    #[serde_inline_default(65.0)]
    neighborhood_radius: f64,

    #[serde_inline_default(12.9)]
    relative_permitivity: f64,

    #[serde_inline_default(11.5)]
    layer_separation: f64
}

impl BistableModelSettings{
    pub fn new() -> Self{
        serde_json::from_str::<BistableModelSettings>("{}".into()).unwrap()
    }
}

impl SimulationModelSettingsTrait for BistableModelSettings{
    fn get_num_samples(&self) -> usize {
        self.num_samples
    }

    fn get_clock_ampl_min(&self) -> f64 {
        self.ampl_min
    }

    fn get_clock_ampl_max(&self) -> f64 {
        self.ampl_max
    }

    fn get_clock_ampl_fac(&self) -> f64 {
        self.ampl_fac
    }

    fn get_max_iter(&self) -> usize {
        self.max_iter
    }
}

impl BistableModel{
    pub fn new() -> Self{
        BistableModel{
            clock_states: [0.0, 0.0, 0.0, 0.0],
            input_states: vec![],
            active_layer: 0,
            cells: Box::new(vec![]),
            cell_input_map: HashMap::new(),
            polarizations: [vec![], vec![]],
            neighbor_indecies: vec![],
            neighbour_kink_energy: vec![],
            settings: BistableModelSettings::new(),
        }
    }

    fn cell_distance(cell_a: &QCACell, cell_b: &QCACell, layer_separation: f64) -> f64{
        (
            (cell_a.position[0] - cell_b.position[0]).powf(2.0) + 
            (cell_a.position[1] - cell_b.position[1]).powf(2.0) + 
            (cell_a.position[2] - cell_b.position[2]).powf(2.0)
        ).sqrt()

    }

    fn determine_kink_energy(cell_a: &QCACell, cell_b: &QCACell, permitivity: f64) -> f64{
        const QCHARGE_SQUAR_OVER_FOUR: f64 = 6.41742353846709430467559076549e-39;
        const FOUR_PI_EPSILON: f64 = 1.11265005597565794635320037482e-10;
    
        const SAME_POLARIZATION: [[f64; 4]; 4] =
        [ [  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR ],
         [ -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR ],
         [  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR ],
         [ -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR ] ];
    
         const DIFF_POLARIZATION: [[f64; 4]; 4] =
        [ [ -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR ],
         [  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR ],
         [ -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR ],
         [  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR ] ];
    
        const DOT_OFFSET_X: [f64; 4] = [-4.5, 4.5, 4.5, -4.5];
        const DOT_OFFSET_Y: [f64; 4] = [-4.5, -4.5, 4.5, 4.5];
        
        let mut energy_same: f64 = 0.0;
        let mut energy_diff: f64 = 0.0;
    
        for i in 0..4 {
            for j in 0..4 {
                let x: f64 = f64::abs(cell_a.position[0] + DOT_OFFSET_X[i] - (cell_b.position[0] + DOT_OFFSET_X[j]));
                let y: f64 = f64::abs(cell_a.position[1] + DOT_OFFSET_Y[i] - (cell_b.position[1] + DOT_OFFSET_Y[j]));
    
                let dist = 1e-9 * f64::sqrt(x * x + y * y);
    
                energy_diff += DIFF_POLARIZATION[i][j] / dist;
                energy_same += SAME_POLARIZATION[i][j] / dist;
            }
        }
    
        return (1.0 / (FOUR_PI_EPSILON * permitivity)) * (energy_diff - energy_same);
    }

    fn get_active_layer(&mut self) -> &mut Vec<f64>{
        &mut self.polarizations[self.active_layer as usize]
    }

    fn get_inactive_layer(&mut self) -> &mut Vec<f64>{
        &mut self.polarizations[i8::abs(self.active_layer - 1) as usize]
    }
}

impl SimulationModelTrait for BistableModel{

    fn get_options_list(&self) -> super::settings::OptionsList {
        vec![
            OptionsEntry::Input { 
                unique_id: "num_samples".to_string(), 
                name: "Number of samples".to_string(), 
                description: "The number of samples to be used in simulation".to_string(), 
                descriptor: InputDescriptor::NumberInput {min: Some(1.0), max: None, unit: None, whole_num: true} 
            },
            OptionsEntry::Input { 
                unique_id: "convergence_tolerance".to_string(), 
                name: "Convergence tolerance".to_string(), 
                description: "Tolerance for simulation convergence check".to_string(), 
                descriptor: InputDescriptor::NumberInput {min: Some(0.0), max: Some(1.0), unit: None, whole_num: false} 
            },
            OptionsEntry::Input { 
                unique_id: "neighborhood_radius".to_string(), 
                name: "Radius of effect".to_string(), 
                description: "Radius of effect for neighbouring cells".to_string(), 
                descriptor: InputDescriptor::NumberInput {min: Some(0.0), max: None, unit: Some("nm".into()), whole_num: false} 
            },
            OptionsEntry::Input { 
                unique_id: "relative_permitivity".to_string(), 
                name: "Relative permitivity".to_string(), 
                description: "Permitivity of the relative medium".to_string(), 
                descriptor: InputDescriptor::NumberInput {min: Some(0.0), max: None, unit: None, whole_num: false} 
            },
            OptionsEntry::Input { 
                unique_id: "max_iter".to_string(), 
                name: "Maximum iterations".to_string(), 
                description: "Maximum number of iterations per sample".to_string(), 
                descriptor: InputDescriptor::NumberInput {min: Some(0.0), max: None, unit: None, whole_num: true} 
            },
            OptionsEntry::Input { 
                unique_id: "layer_separation".to_string(), 
                name: "Layer separation".to_string(), 
                description: "Separation between layers in nm".to_string(), 
                descriptor: InputDescriptor::NumberInput {min: Some(0.0), max: None, unit: Some("nm".into()), whole_num: false} 
            }
        ]
    }
    
    fn get_deserialized_settings(&self) -> Result<String, String> {
        match serde_json::to_string(&self.settings){
            Ok(res) => Ok(res),
            Err(err) => Err(err.to_string()),
        }
    }
    
    fn set_serialized_settings(&mut self, settings_str: &String) -> Result<(), String>{
        match serde_json::from_str::<BistableModelSettings>(settings_str) {
            Ok(res) => 
            {
                self.settings = res; 
                Ok(())
            },
            Err(err) => Err(err.to_string()),
        }
    }
    
    fn get_name(&self) -> String {
        "Bistable".into()
    }
    
    fn get_unique_id(&self) -> String {
        "bistable_model".into()
    }

    fn initiate(&mut self, cells: Box<Vec<super::QCACell>>) {
        self.cells = cells;

        self.cell_input_map =  self.cells.iter()
            .enumerate()
            .filter(|(_, c)| {
                c.typ == CellType::Input
            })
            .enumerate()
            .map(|(j, (i, _))| {
                (i, j)
            }).collect();

        let tmp_polarizations: Vec<f64> = self.cells.iter().map(|c| {
            (c.dot_probability_distribution[0] -  c.dot_probability_distribution[1] + 
                c.dot_probability_distribution[2] - c.dot_probability_distribution[3]) / 
                    c.dot_probability_distribution.iter().sum::<f64>()
                
        }).collect();

        self.active_layer = 0;
        self.polarizations = [tmp_polarizations.clone(), tmp_polarizations.clone()];

        self.neighbor_indecies = vec![Vec::new(); self.cells.len()];
        self.neighbour_kink_energy = vec![Vec::new(); self.cells.len()];

        for i in 0..self.cells.len() {
            for j in 0..self.cells.len() {
                if (i != j) && 
                    BistableModel::cell_distance(
                        &self.cells[i], 
                        &self.cells[j], 
                        self.settings.layer_separation)
                    <= self.settings.neighborhood_radius {
                    self.neighbor_indecies[i].push(j);
                    let permitivity = self.settings.relative_permitivity;
                    self.neighbour_kink_energy[i].push(
                        BistableModel::determine_kink_energy(&self.cells[i], &self.cells[j], permitivity)
                    );
                }
            }
        }
    }

    fn pre_calculate(&mut self, clock_states: &[f64; 4], input_states: &Vec<f64>) {
        self.clock_states = clock_states.clone();
        self.input_states = input_states.clone();
        self.active_layer = i8::abs(self.active_layer - 1);
    }

    fn calculate(&mut self, cell_ind: usize) -> bool {
        let c = self.cells[cell_ind].clone();
        match c.typ {
            CellType::Fixed => true,
            CellType::Input => {
                let input_index = *self.cell_input_map.get(&cell_ind).unwrap();
                self.get_active_layer()[cell_ind] = *self.input_states.get(input_index).unwrap();
                true
            }
            _ => {
                let old_polarization = self.get_inactive_layer()[cell_ind];

                let mut polar_math = 0.0;
                for i in 0..self.neighbor_indecies[cell_ind].len(){
                    let neighbour_ind = self.neighbor_indecies[cell_ind][i];
                    polar_math += self.neighbour_kink_energy[cell_ind][i] * self.get_inactive_layer()[neighbour_ind];
                }

                let clock_index = (c.clock_phase_shift as i32 % 90) as usize;

                polar_math /= 2.0 * self.clock_states[clock_index]; 

                let new_polarization = 
                    if polar_math > 1000.0 {1.0}
                    else if polar_math < -1000.0 {-1.0}
                    else if f64::abs(polar_math) < 0.001 {polar_math}
                    else {polar_math / f64::sqrt(1.0 + polar_math * polar_math)};

                self.get_active_layer()[cell_ind] = new_polarization;
                f64::abs(new_polarization - old_polarization) <= self.settings.convergence_tolerance
            }
        }
    }
    
    fn get_states(&mut self) -> Vec<f64>{
        return self.get_active_layer().clone();
    }
    
    fn get_settings(&self) -> Box<dyn super::SimulationModelSettingsTrait> {
        Box::new(self.settings.clone()) as Box<dyn SimulationModelSettingsTrait>
    }

}