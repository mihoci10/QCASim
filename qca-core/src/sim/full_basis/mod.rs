use core::panic;
use std::{collections::HashMap, f64::consts::PI, fs};

use nalgebra::{distance, DMatrix, DMatrixView, DVector, DVectorView, Point2, SVector, SVectorView};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

use crate::sim::settings::{InputDescriptor, OptionsEntry};

use super::{CellType, QCACell, SimulationModelSettingsTrait, SimulationModelTrait};

#[derive(Debug)]
pub struct QCACellInternal<const N: usize>{
    //Cell rotation in radians
    rotation: f64,
    dot_distance: f64,

    //The full hamilton matrix
    hamilton_matrix: DMatrix<f64>,
    //Static hamilton matrix (1, 3, 4)
    static_hamilton_matrix: DMatrix<f64>,
    //Dynamic hamilton matrix (2)
    dynamic_hamilton_matrix: DMatrix<f64>,

    //Potential energy at each dot
    dot_potential: SVector<f64, N>,
    tunneling_energy: f64,
    dot_charge_probability: SVector<f64, N>,
}

impl<const N: usize> QCACellInternal<N>{
    pub fn new(rotation: f64, dot_distance: f64, tunneling_energy: f64) -> Self{
        let tunneling_matrix = Self::generate_tunneling_matrix(tunneling_energy);
        let basis_matrix = Self::generate_basis_matrix();
        let dot_potential: SVector<f64, N> = SVector::zeros();

        let static_hamilton_matrix = 
            DMatrix::<f64>::from_iterator(N*N, N*N, (0..N*N).map(|i| {
                (0..N*N).map(|j| {
                    Self::hamilton_term_1(1.0, dot_potential.as_view(), basis_matrix.row(i).transpose().as_view(), basis_matrix.row(j).transpose().as_view())
                    +
                    Self::hamilton_term_3(43.14, basis_matrix.row(i).transpose().as_view(), basis_matrix.row(j).transpose().as_view())
                    +
                    Self::hamilton_term_4(dot_distance, rotation, 143.8, basis_matrix.row(i).transpose().as_view(), basis_matrix.row(j).transpose().as_view())
                }).collect::<Vec<f64>>()
            }).flatten());

        let dynamic_hamilton_matrix = DMatrix::<f64>::from_iterator(N*N, N*N, (0..N*N).map(|i| {
            (0..N*N).map(|j| {
                Self::hamilton_term_2(tunneling_matrix.as_view(), basis_matrix.row(i).transpose().as_view(), basis_matrix.row(j).transpose().as_view())
            }).collect::<Vec<f64>>()
        }).flatten());

        fs::write("output.txt", format!("{}", &static_hamilton_matrix + &dynamic_hamilton_matrix));

        QCACellInternal{
            rotation: rotation,
            dot_distance: dot_distance,
            hamilton_matrix: &static_hamilton_matrix + &dynamic_hamilton_matrix,
            static_hamilton_matrix: static_hamilton_matrix,
            dynamic_hamilton_matrix: dynamic_hamilton_matrix,
            dot_potential: dot_potential,
            dot_charge_probability: SVector::<f64, N>::from_vec(vec![2.0 / N as f64; N]),
            tunneling_energy: tunneling_energy,
        }
    }

    fn get_dot_position(dot_index: usize, dot_distance: f64, rotation: f64) -> Point2<f64>{
        if dot_index >= N{
            panic!("Invalid dot index!");
        }
        let radius = dot_distance * (f64::sqrt(2.0)) / 2.0;
        let alpha = PI / 4.0 - rotation + (2.0 * PI / N as f64) * (dot_index as f64 / 4.0).floor(); 

        Point2::new(
            radius * (PI / 2.0 * -(dot_index as f64) + alpha).cos(), 
            radius * (PI / 2.0 * -(dot_index as f64) + alpha).sin()
        )
    }

    fn generate_tunneling_matrix(energy: f64) -> DMatrix<f64>{
        let mut tunneling_matrix = DMatrix::<f64>::zeros(N, N);

        match N {
            8 => {
                tunneling_matrix[(0,4)] = energy;
                tunneling_matrix[(0,5)] = energy;
                tunneling_matrix[(1,5)] = energy;
                tunneling_matrix[(1,6)] = energy;
                tunneling_matrix[(2,6)] = energy;
                tunneling_matrix[(2,7)] = energy;
                tunneling_matrix[(3,7)] = energy;
                tunneling_matrix[(3,4)] = energy;
                tunneling_matrix[(4,3)] = energy;
                tunneling_matrix[(4,0)] = energy;
                tunneling_matrix[(5,0)] = energy;
                tunneling_matrix[(5,1)] = energy;
                tunneling_matrix[(6,1)] = energy;
                tunneling_matrix[(6,2)] = energy;
                tunneling_matrix[(7,2)] = energy;
                tunneling_matrix[(7,3)] = energy;
            }
            _ => {
                panic!("Unsupported cell type!");
            }
        }

        tunneling_matrix
    }

    fn generate_basis_matrix() -> DMatrix<f64> {
        DMatrix::<f64>::from_iterator(N*N, N*2, (0..N * 2).map(|i| {
            let mut column: Vec<f64>;
            if i < N {
                column = vec![
                    vec![0.0; N * (N - i - 1)],
                    vec![1.0; N],
                    vec![0.0; N * i]
                ].concat();
            } else {
                column = vec![0.0; N*N];
                (0..N).for_each(|j| {
                    column[N*j + (2*N - i - 1)] = 1.0;
                });
            }
            column
        }).flatten()) 
    }

    fn count_operator(dot_index: usize, spin: i32, basis_vector: DVectorView<f64>) -> f64{
        let i: usize;
        if spin == 1 {
            i = dot_index;
        } else {
            i = dot_index + N;
        }
        if basis_vector[i] == 1.0 {
            1.0
        } else {
            0.0
        }
    }

    fn capture_operator(dot_index: usize, basis_vector: DVectorView<f64>) -> f64{
        if (basis_vector[dot_index] == 1.0) && (basis_vector[dot_index + N] == 1.0) {
            1.0
        } else {
            0.0
        }
    }

    fn coulumb_operator(dot_i: usize, dot_j: usize, spin: i32, basis_vector: DVectorView<f64>) -> f64{
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

        Self::count_operator(dot_i, spin, basis_vector) *
        Self::count_operator(dot_j, n_spin, basis_vector)
    }

    fn tunneling_operator(dot_i: usize, dot_j: usize, spin: i32, basis_vector: DVectorView<f64>) -> DVector<f64>{
        let mut tunneling_vector: DVector<f64> = basis_vector.clone_owned();

        if spin == 1{
            tunneling_vector.swap((dot_i, 0), (dot_j, 0));
        } else {
            tunneling_vector.swap((dot_i + N, 0), (dot_j + N, 0));
        }

        if tunneling_vector == basis_vector{
            tunneling_vector = DVector::<f64>::zeros(N*N);
        }

        tunneling_vector
    }

    fn hamilton_term_1(
        e0: f64, 
        dot_potential: SVectorView<f64, N>, 
        basis_vector_i: DVectorView<f64>, 
        basis_vector_j: DVectorView<f64>) -> f64 {
            (0..N).map(|i| {
                (0..=1).map(|spin| {
                    if basis_vector_i == basis_vector_j{
                        Self::count_operator(i, spin, basis_vector_j) * (e0 + dot_potential[i])
                    } else{
                        0.0
                    }
                }).sum::<f64>()
            }).sum()
    }

    fn hamilton_term_2(
        tunneling_matrix: DMatrixView<f64>, 
        basis_vector_i: DVectorView<f64>, 
        basis_vector_j: DVectorView<f64>) -> f64 {
            (0..N).map(|i| {
                (0..i).map(|j| {
                    if tunneling_matrix[(i, j)] != 0.0 {
                        (0..=1).map(|spin| {
                            if Self::tunneling_operator(i, j, spin, basis_vector_j) == basis_vector_i{
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
        eq: f64, 
        basis_vector_i: DVectorView<f64>, 
        basis_vector_j: DVectorView<f64>) -> f64 {
            (0..N).map(|i| {
                if basis_vector_i == basis_vector_j{
                    Self::capture_operator(i, basis_vector_j) * eq
                } else {
                    0.0
                }
            }).sum()
    }

    fn hamilton_term_4(
        dot_distance: f64, 
        rotation: f64,
        vq: f64, 
        basis_vector_i: DVectorView<f64>, 
        basis_vector_j: DVectorView<f64>) -> f64 {
            (0..N).map(|i| {
                (0..i).map(|j| {
                    (0..=1).map(|spin| {
                        if basis_vector_i == basis_vector_j{
                            Self::coulumb_operator(i, j, spin, basis_vector_j) *
                            (vq / distance(&Self::get_dot_position(i, dot_distance, rotation), &Self::get_dot_position(j, dot_distance, rotation)))
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
    cells: Box<Vec<super::QCACell>>,
    cell_input_map: HashMap<usize, usize>,
    active_layer: i8,
    polarizations: [Vec<f64>; 2],
    neighbor_indecies: Vec<Vec<usize>>,
    neighbour_kink_energy: Vec<Vec<f64>>,
    settings: FullBasisModelSettings
}


#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone)]
pub struct FullBasisModelSettings{
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
            active_layer: 0,
            cells: Box::new(vec![]),
            cell_input_map: HashMap::new(),
            polarizations: [vec![], vec![]],
            neighbor_indecies: vec![],
            neighbour_kink_energy: vec![],
            settings: FullBasisModelSettings::new(),
        }
    }

    fn cell_distance(cell_a: &QCACell, cell_b: &QCACell, layer_separation: f64) -> f64{
        (
            (cell_a.pos_x - cell_b.pos_x).powf(2.0) + 
            (cell_a.pos_y - cell_b.pos_y).powf(2.0) + 
            (layer_separation * (cell_a.z_index - cell_b.z_index) as f64).powf(2.0)
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
                let x: f64 = f64::abs(cell_a.pos_x + DOT_OFFSET_X[i] - (cell_b.pos_x + DOT_OFFSET_X[j]));
                let y: f64 = f64::abs(cell_a.pos_y + DOT_OFFSET_Y[i] - (cell_b.pos_y + DOT_OFFSET_Y[j]));
    
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
            c.polarization
        }).collect();

        self.active_layer = 0;
        self.polarizations = [tmp_polarizations.clone(), tmp_polarizations.clone()];

        self.neighbor_indecies = vec![Vec::new(); self.cells.len()];
        self.neighbour_kink_energy = vec![Vec::new(); self.cells.len()];

        for i in 0..self.cells.len() {
            for j in 0..self.cells.len() {
                if (i != j) && 
                    FullBasisModel::cell_distance(
                        &self.cells[i], 
                        &self.cells[j], 
                        self.settings.layer_separation)
                    <= self.settings.neighborhood_radius {
                    self.neighbor_indecies[i].push(j);
                    let permitivity = self.settings.relative_permitivity;
                    self.neighbour_kink_energy[i].push(
                        FullBasisModel::determine_kink_energy(&self.cells[i], &self.cells[j], permitivity)
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
        let c = self.cells[cell_ind];
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