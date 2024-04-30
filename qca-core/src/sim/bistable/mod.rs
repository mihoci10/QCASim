use std::collections::HashMap;

use crate::sim::settings::{InputDescriptor, OptionsEntry};

use super::{settings::{OptionValue, OptionsValueList}, CellType, QCACell, SimulationModelInstanceTrait, SimulationModelTrait};

pub struct BistableModel {
    options_value_list: OptionsValueList
}

pub struct BistableModelInstance {
    clock_states: [f64; 4],
    cells: Box<Vec<super::QCACell>>,
    active_layer: i8,
    polarizations: [Vec<f64>; 2],
    neighbor_indecies: Vec<Vec<usize>>,
    neighbour_kink_energy: Vec<Vec<f64>>
}

impl BistableModel{
    pub fn new() -> Self{
        BistableModel{
            options_value_list: HashMap::from([
                ("cell_size".to_string(), OptionValue::Number { value: 18.0 })
            ])
        }
    }
}

impl BistableModelInstance{

    pub fn new() -> Self{
        BistableModelInstance{
            clock_states: [0.0, 0.0, 0.0, 0.0],
            active_layer: 0,
            cells: Box::new(vec![]),
            polarizations: [vec![], vec![]],
            neighbor_indecies: vec![],
            neighbour_kink_energy: vec![],
        }
    }

    fn determine_kink_energy(cell_a: &QCACell, cell_b: &QCACell) -> f64{
        const QCHARGE_SQUAR_OVER_FOUR: f64 = 6.417423538e-39;
        const FOUR_PI_EPSILON: f64 = 1.112650056e-10;
    
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
    
        const DOT_OFFSET_X: [f64; 4] = [-9.0, 9.0, 9.0, -9.0];
        const DOT_OFFSET_Y: [f64; 4] = [-9.0, -9.0, 9.0, 9.0];
        
        let mut energy_same: f64 = 0.0;
        let mut energy_diff: f64 = 0.0;
    
        for i in 0..4 {
            for j in 0..4 {
                let x: f64 = f64::abs(cell_a.pos_x + DOT_OFFSET_X[i] - (cell_b.pos_x + DOT_OFFSET_X[j]));
                let y: f64 = f64::abs(cell_a.pos_y + DOT_OFFSET_Y[i] - (cell_b.pos_y + DOT_OFFSET_Y[j]));
    
                let dist = 1e-9 * f64::sqrt(x * x + y * y);
    
                energy_diff += DIFF_POLARIZATION[i][j] / dist;
                energy_same += SAME_POLARIZATION[i][j] / dist;
            }
        }
    
        return (1.0 / (FOUR_PI_EPSILON * 12.900000)) * (energy_diff - energy_same);
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
            OptionsEntry::Header { label: "Cell structure".to_string() },
            OptionsEntry::Break,
            OptionsEntry::Input { 
                unique_id: "cell_size".to_string(), 
                name: "Size".to_string(), 
                description: "Side dimension of the cell in nm".to_string(), 
                descriptor: InputDescriptor::NumberInput {} }
        ]
    }
    
    fn get_options_value_list(&self) -> super::settings::OptionsValueList {
        self.options_value_list.clone()
    }
    
    fn set_options_value_list(&mut self, options_value_list: super::settings::OptionsValueList) {
        self.options_value_list = options_value_list;
    }
    
    fn get_name(&self) -> String {
        "Bistable".into()
    }
    
    fn get_unique_id(&self) -> String {
        "bistable_model".into()
    }
    
    fn create_instance(&self) -> Box<dyn super::SimulationModelInstanceTrait> {
        Box::new(BistableModelInstance::new())
    }

}

impl SimulationModelInstanceTrait for BistableModelInstance{
    fn initiate(&mut self, cells: Box<Vec<super::QCACell>>) {
        self.cells = cells;

        let tmp_polarizations: Vec<f64> = self.cells.iter().map(|c| {
            c.polarization
        }).collect();

        self.active_layer = 0;
        self.polarizations = [tmp_polarizations.clone(), tmp_polarizations.clone()];

        self.neighbor_indecies = vec![Vec::new(); self.cells.len()];
        self.neighbour_kink_energy = vec![Vec::new(); self.cells.len()];

        for i in 0..self.cells.len() {
            for j in 0..self.cells.len() {
                if i != j {
                    self.neighbor_indecies[i].push(j);
                    self.neighbour_kink_energy[i].push(
                        BistableModelInstance::determine_kink_energy(&self.cells[i], &self.cells[j])
                    );
                }
            }
        }
    }

    fn pre_calculate(&mut self, clock_states: [f64; 4]) {
        self.clock_states = clock_states;
        self.active_layer = i8::abs(self.active_layer - 1);
    }

    fn calculate(&mut self, cell_ind: usize) -> bool {
        let c = &self.cells[cell_ind];
        match c.typ {
            CellType::Fixed => true,
            _ => {
                let old_polarization = self.get_active_layer()[cell_ind];

                let mut polar_math = 0.0;
                for i in 0..self.neighbor_indecies[cell_ind].len(){
                    let neighbour_ind = self.neighbor_indecies[cell_ind][i];
                    polar_math += self.neighbour_kink_energy[cell_ind][i] * self.get_inactive_layer()[neighbour_ind];
                }

                polar_math /= 2.0 * 9.800000e-022;

                let new_polarization = 
                    if polar_math > 1000.0 {1.0}
                    else if polar_math < -1000.0 {-1.0}
                    else if f64::abs(polar_math) < 0.001 {polar_math}
                    else {polar_math / f64::sqrt(1.0 + polar_math * polar_math)};

                self.get_active_layer()[cell_ind] = new_polarization;
                f64::abs(new_polarization - old_polarization) <= 0.001
            }
        }
    }
    
    fn get_states(&mut self) -> Vec<f64>{
        return self.get_active_layer().clone();
    }
}