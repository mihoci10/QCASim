use core::{arch, panic};
use std::{cell::{self, Cell}, collections::HashMap, f64::consts::PI, fs, os::windows::io::IntoRawSocket, sync::Arc};

use nalgebra::{distance, DMatrix, DMatrixView, DVector, DVectorView, Point2, Point3, SVector, SVectorView};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

use crate::sim::settings::{InputDescriptor, OptionsEntry};

use super::{CellType, QCACell, QCACellArchitecture, SimulationModelSettingsTrait, SimulationModelTrait};

#[derive(Debug)]
pub struct QCACellInternal{
    cell: Box<QCACell>,
    architecture: Box<QCACellArchitecture>,

    //The full hamilton matrix
    hamilton_matrix: DMatrix<f64>,
    //Static hamilton matrix (1, 3, 4)
    static_hamilton_matrix: DMatrix<f64>,
    //Dynamic hamilton matrix (2)
    dynamic_hamilton_matrix: DMatrix<f64>,

    //Potential energy at each dot
    dot_potential: DVector<f64>,
    dot_charge_probability: DVector<f64>,
}

impl QCACellInternal{
    pub fn new(cell: Box<QCACell>, architecture: Box<QCACellArchitecture>) -> Self{
        let n: usize = architecture.dot_count as usize;
        let tunneling_matrix = Self::generate_tunneling_matrix(&architecture, 1.0);
        let basis_matrix = Self::generate_basis_matrix(n);
        let dot_potential: DVector<f64> = DVector::zeros(n);

        let static_hamilton_matrix = 
            DMatrix::<f64>::from_iterator(n*n, n*n, (0..n*n).map(|i| {
                (0..n*n).map(|j| {
                    Self::hamilton_term_1(n, 1.0, dot_potential.as_view(), basis_matrix.row(i).transpose().as_view(), basis_matrix.row(j).transpose().as_view())
                    +
                    Self::hamilton_term_3(n, 43.14, basis_matrix.row(i).transpose().as_view(), basis_matrix.row(j).transpose().as_view())
                    +
                    Self::hamilton_term_4(&cell, &architecture, 143.8, basis_matrix.row(i).transpose().as_view(), basis_matrix.row(j).transpose().as_view())
                }).collect::<Vec<f64>>()
            }).flatten());

        let dynamic_hamilton_matrix = DMatrix::<f64>::from_iterator(n*n, n*n, (0..n*n).map(|i| {
            (0..n*n).map(|j| {
                Self::hamilton_term_2(n, tunneling_matrix.as_view(), basis_matrix.row(i).transpose().as_view(), basis_matrix.row(j).transpose().as_view())
            }).collect::<Vec<f64>>()
        }).flatten());

        QCACellInternal{
            cell: cell,
            architecture: architecture,
            hamilton_matrix: &static_hamilton_matrix + &dynamic_hamilton_matrix,
            static_hamilton_matrix: static_hamilton_matrix,
            dynamic_hamilton_matrix: dynamic_hamilton_matrix,
            dot_potential: dot_potential,
            dot_charge_probability: DVector::<f64>::from_vec(vec![2.0 / n as f64; n]),
        }
    }

    fn get_dot_position(dot_index: usize, cell: &Box<QCACell>, architecture: &Box<QCACellArchitecture>) -> Point3<f64>{
        let x = architecture.dot_positions[dot_index][0];
        let y = architecture.dot_positions[dot_index][1];

        Point3::new(
            x * cell.rotation.cos() - y * cell.rotation.sin(),
            y * cell.rotation.cos() + x * cell.rotation.sin(),
            0.0
        )
    }

    fn generate_tunneling_matrix(architecture: &Box<QCACellArchitecture>, energy: f64) -> DMatrix<f64>{
        let mut tunneling_matrix = DMatrix::<f64>::zeros(
            architecture.dot_count as usize, 
            architecture.dot_count as usize
        );

        architecture.dot_tunnels.iter().for_each(|(a, b)| {
            tunneling_matrix[(*a as usize, *b as usize)] = energy;
            tunneling_matrix[(*b as usize, *a as usize)] = energy;
        });

        tunneling_matrix
    }

    fn generate_basis_matrix(dot_count: usize) -> DMatrix<f64> {
        DMatrix::<f64>::from_iterator(dot_count * dot_count, dot_count*2, (0..dot_count * 2).map(|i| {
            let mut column: Vec<f64>;
            if i < dot_count {
                column = vec![
                    vec![0.0; dot_count * (dot_count - i - 1)],
                    vec![1.0; dot_count],
                    vec![0.0; dot_count * i]
                ].concat();
            } else {
                column = vec![0.0; dot_count * dot_count];
                (0..dot_count).for_each(|j| {
                    column[dot_count * j + (2 * dot_count - i - 1)] = 1.0;
                });
            }
            column
        }).flatten()) 
    }

    fn count_operator(dot_count: usize, dot_index: usize, spin: i32, basis_vector: DVectorView<f64>) -> f64{
        let i: usize;
        if spin == 1 {
            i = dot_index;
        } else {
            i = dot_index + dot_count;
        }
        if basis_vector[i] == 1.0 {
            1.0
        } else {
            0.0
        }
    }

    fn capture_operator(dot_count: usize, dot_index: usize, basis_vector: DVectorView<f64>) -> f64{
        if (basis_vector[dot_index] == 1.0) && (basis_vector[dot_index + dot_count] == 1.0) {
            1.0
        } else {
            0.0
        }
    }

    fn coulumb_operator(dot_count: usize, dot_i: usize, dot_j: usize, spin: i32, basis_vector: DVectorView<f64>) -> f64{
        if dot_i == dot_j {
            panic!("Dot indicies cannot be equal!")
        }

        let n_spin;
        if spin == 1 {
            n_spin = 0
        }
        else {
            n_spin = 1
        };

        Self::count_operator(dot_count, dot_i, spin, basis_vector) *
        Self::count_operator(dot_count, dot_j, n_spin, basis_vector)
    }

    fn tunneling_operator(dot_count: usize, dot_i: usize, dot_j: usize, spin: i32, basis_vector: DVectorView<f64>) -> DVector<f64>{
        let mut tunneling_vector: DVector<f64> = basis_vector.clone_owned();

        if spin == 1{
            tunneling_vector.swap((dot_i, 0), (dot_j, 0));
        } else {
            tunneling_vector.swap((dot_i + dot_count, 0), (dot_j + dot_count, 0));
        }

        if tunneling_vector == basis_vector{
            tunneling_vector = DVector::<f64>::zeros(dot_count * dot_count);
        }

        tunneling_vector
    }

    fn hamilton_term_1(
        dot_count: usize,
        e0: f64, 
        dot_potential: DVectorView<f64>, 
        basis_vector_i: DVectorView<f64>, 
        basis_vector_j: DVectorView<f64>) -> f64 {
            (0..dot_count).map(|i| {
                (0..=1).map(|spin| {
                    if basis_vector_i == basis_vector_j{
                        Self::count_operator(dot_count, i, spin, basis_vector_j) * (e0 + dot_potential[i])
                    } else{
                        0.0
                    }
                }).sum::<f64>()
            }).sum()
    }

    fn hamilton_term_2(
        dot_count: usize,
        tunneling_matrix: DMatrixView<f64>, 
        basis_vector_i: DVectorView<f64>, 
        basis_vector_j: DVectorView<f64>) -> f64 {
            (0..dot_count).map(|i| {
                (0..i).map(|j| {
                    if tunneling_matrix[(i, j)] != 0.0 {
                        (0..=1).map(|spin| {
                            if Self::tunneling_operator(dot_count, i, j, spin, basis_vector_j) == basis_vector_i{
                                tunneling_matrix[(i, j)]
                            }
                            else {
                                0.0
                            }
                        }).sum()
                    } else {
                        0.0
                    }
                }).sum::<f64>()
            }).sum()
    }

    fn hamilton_term_3(
        dot_count: usize,
        eq: f64, 
        basis_vector_i: DVectorView<f64>, 
        basis_vector_j: DVectorView<f64>) -> f64 {
            (0..dot_count).map(|i| {
                if basis_vector_i == basis_vector_j{
                    Self::capture_operator(dot_count, i, basis_vector_j) * eq
                } else {
                    0.0
                }
            }).sum()
    }

    fn hamilton_term_4(
        cell: &Box<QCACell>,
        architecture: &Box<QCACellArchitecture>,
        vq: f64, 
        basis_vector_i: DVectorView<f64>, 
        basis_vector_j: DVectorView<f64>) -> f64 {
            (0..architecture.dot_count as usize).map(|i| {
                (0..i).map(|j| {
                    (0..=1).map(|spin| {
                        if basis_vector_i == basis_vector_j{
                            Self::coulumb_operator(architecture.dot_count as usize, i, j, spin, basis_vector_j) *
                            (vq / distance(&Self::get_dot_position(i, cell, architecture), &Self::get_dot_position(j, cell, architecture)))
                        }
                        else{
                            0.0
                        }
                    }).sum::<f64>()
                }).sum::<f64>()
            }).sum()
    }
}

pub struct FullBasisModel {
    clock_states: [f64; 4],
    input_states: Vec<f64>,
    cells: Vec<QCACellInternal>,
    architecture: Box<QCACellArchitecture>,
    settings: FullBasisModelSettings
}


#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone)]
pub struct FullBasisModelSettings{
    #[serde_inline_default(1000)]
    num_samples: usize,

    #[serde_inline_default(100)]
    max_iter: usize,

    #[serde_inline_default(-2.0)]
    ampl_min: f64,
    
    #[serde_inline_default(0.0)]
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

impl FullBasisModelSettings{
    pub fn new() -> Self{
        serde_json::from_str::<FullBasisModelSettings>("{}".into()).unwrap()
    }
}

impl SimulationModelSettingsTrait for FullBasisModelSettings{
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

impl FullBasisModel{
    pub fn new() -> Self{
        FullBasisModel{
            clock_states: [0.0, 0.0, 0.0, 0.0],
            input_states: vec![],
            cells: vec![],
            settings: FullBasisModelSettings::new(),
        }
    }
}

impl SimulationModelTrait for FullBasisModel{

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
        match serde_json::from_str::<FullBasisModelSettings>(settings_str) {
            Ok(res) => 
            {
                self.settings = res; 
                Ok(())
            },
            Err(err) => Err(err.to_string()),
        }
    }
    
    fn get_name(&self) -> String {
        "Full basis".into()
    }
    
    fn get_unique_id(&self) -> String {
        "full_basis_model".into()
    }

    fn initiate(&mut self, architecture: Box<QCACellArchitecture>, cells: Box<Vec<QCACell>>) {
        self.cells = cells.iter().map(|c| {
            QCACellInternal::new(Box::new(c.clone()), architecture)
        }).collect();
        self.architecture = architecture;     
    }

    fn pre_calculate(&mut self, clock_states: &[f64; 4], input_states: &Vec<f64>) {
        self.clock_states = clock_states.clone();
        self.input_states = input_states.clone();
    }

    fn calculate(&mut self, cell_ind: usize) -> bool {
        let n = self.architecture.dot_count as usize;
        let internal_cell = &self.cells[cell_ind];
        
        let old_charge_probability = internal_cell.dot_charge_probability.clone();

        if internal_cell.cell.typ == CellType::Normal {
            let clock_index = (internal_cell.cell.clock_phase_shift as i32 % 90) as usize;
            internal_cell.hamilton_matrix = 
                internal_cell.static_hamilton_matrix + internal_cell.dynamic_hamilton_matrix * self.clock_states[clock_index];
        }

        match internal_cell.cell.typ {
            CellType::Input => todo!(),
            CellType::Fixed => todo!(),
            CellType::Normal | CellType::Output => {
                internal_cell.dot_potential = DVector::zeros(n);
                for (ind, c) in self.cells.iter().enumerate(){
                    if ind != cell_ind{
                        for i in 0..n {
                            for j in 0..n{
                                let dot_off_i = QCACellInternal::get_dot_position(i, &internal_cell.cell, &internal_cell.architecture);
                                let dot_off_j = QCACellInternal::get_dot_position(j, &c.cell, &c.architecture);
                                let cell_pos_i = Point3::new(internal_cell.cell.position[0], internal_cell.cell.position[1], internal_cell.cell.position[2]);
                                let cell_pos_j = Point3::new(c.cell.position[0], c.cell.position[1], c.cell.position[2]);

                                let distance = distance(cell_pos_i + dot_off_i, p2)
                            }
                        }
                    }
                }
            },
        }

        let mut stable: bool = true;
        for i in 0..self.architecture.dot_count as usize {
            if (internal_cell.dot_charge_probability[i] - old_charge_probability[i]).abs() > self.settings.convergence_tolerance{
                stable = false;
            }
        }

        return stable;
    }
    
    fn get_settings(&self) -> Box<dyn super::SimulationModelSettingsTrait> {
        Box::new(self.settings.clone()) as Box<dyn SimulationModelSettingsTrait>
    }

}