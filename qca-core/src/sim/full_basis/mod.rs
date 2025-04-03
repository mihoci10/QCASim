use std::{cell, collections::HashMap};

use nalgebra::{distance, DMatrix, DMatrixView, DVector, DVectorView, Point3, Schur};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use crate::sim::cell::{polarization_to_dot_probability_distribution, QCACell};
use crate::sim::model::SimulationModelSettingsTrait;
use crate::sim::settings::{InputDescriptor, OptionsEntry};

use super::{CellType, QCACellArchitecture, QCACellIndex, QCALayer, SimulationModelTrait};

fn calculate_vq(relative_permittivity: f64) -> f64 {
    const CHARGE: f64 = 1.6021e-19;
    const VACUUM_PERMITTIVITY: f64 = 8.8542e-12;

    // CHARGE.powf(2.0) / (4.0 * PI * VACUUM_PERMITTIVITY * relative_permittivity)
    120.9
}

#[derive(Debug, Clone)]
pub struct QCACellInternal{
    cell: Box<QCACell>,
    z_position: f64,

    //The full hamilton matrix
    hamilton_matrix: DMatrix<f64>,
    //Static hamilton matrix (1, 3, 4)
    static_hamilton_matrix: DMatrix<f64>,
    //Dynamic hamilton matrix (2)
    dynamic_hamilton_matrix: DMatrix<f64>,

    //Potential energy at each dot
    dot_potential: DVector<f64>,
    dot_charge_probability: DVector<f64>,

    basis_matrix: DMatrix<f64>
}

impl QCACellInternal{
    pub fn new(cell: Box<QCACell>, layer: &QCALayer, cell_architecture: &QCACellArchitecture, relative_permittivity: f64) -> Self{
        let n: usize = cell_architecture.dot_count as usize;
        let tunneling_matrix = Self::generate_tunneling_matrix(&cell_architecture, 1.0);
        let basis_matrix = Self::generate_basis_matrix(n);
        let dot_potential: DVector<f64> = DVector::zeros(n);

        let vq = calculate_vq(relative_permittivity);
        let eq = vq / (cell_architecture.dot_diameter / 3.0);

        let static_hamilton_matrix = 
            DMatrix::<f64>::from_iterator(n*n, n*n, (0..n*n).map(|i| {
                (0..n*n).map(|j| {
                    Self::hamilton_term_1(n, 1.0, dot_potential.as_view(), basis_matrix.row(i).transpose().as_view(), basis_matrix.row(j).transpose().as_view())
                    +
                    Self::hamilton_term_3(n, eq, basis_matrix.row(i).transpose().as_view(), basis_matrix.row(j).transpose().as_view())
                    +
                    Self::hamilton_term_4(&cell, &layer, &cell_architecture, vq, basis_matrix.row(i).transpose().as_view(), basis_matrix.row(j).transpose().as_view())
                }).collect::<Vec<f64>>()
            }).flatten());

        let dynamic_hamilton_matrix = DMatrix::<f64>::from_iterator(n*n, n*n, (0..n*n).map(|i| {
            (0..n*n).map(|j| {
                Self::hamilton_term_2(n, tunneling_matrix.as_view(), basis_matrix.row(i).transpose().as_view(), basis_matrix.row(j).transpose().as_view())
            }).collect::<Vec<f64>>()
        }).flatten());

        QCACellInternal{
            cell: cell,
            z_position: layer.z_position,
            hamilton_matrix: &static_hamilton_matrix + &dynamic_hamilton_matrix,
            static_hamilton_matrix: static_hamilton_matrix,
            dynamic_hamilton_matrix: dynamic_hamilton_matrix,
            dot_potential: dot_potential,
            dot_charge_probability: DVector::<f64>::from_vec(vec![2.0 / n as f64; n]),
            basis_matrix: basis_matrix
        }
    }

    fn get_dot_position(dot_index: usize, cell: &Box<QCACell>, layer: &QCALayer, cell_architecture: &QCACellArchitecture) -> Point3<f64>{
        let x = cell_architecture.dot_positions[dot_index][0];
        let y = cell_architecture.dot_positions[dot_index][1];

        Point3::new(
            cell.position[0] + x * cell.rotation.cos() - y * cell.rotation.sin(),
            cell.position[1] + y * cell.rotation.cos() + x * cell.rotation.sin(),
            layer.z_position
        )
    }

    fn generate_tunneling_matrix(cell_architecture: &QCACellArchitecture , energy: f64) -> DMatrix<f64>{
        let mut tunneling_matrix = DMatrix::<f64>::zeros(
            cell_architecture.dot_count as usize,
            cell_architecture.dot_count as usize
        );

        cell_architecture.dot_tunnels.iter().for_each(|(a, b)| {
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
        layer: &QCALayer,
        cell_architecture: &QCACellArchitecture,
        vq: f64, 
        basis_vector_i: DVectorView<f64>, 
        basis_vector_j: DVectorView<f64>) -> f64 {
            (0..cell_architecture.dot_count as usize).map(|i| {
                (0..i).map(|j| {
                    (0..=1).map(|spin| {
                        if basis_vector_i == basis_vector_j{
                            Self::coulumb_operator(cell_architecture.dot_count as usize, i, j, spin, basis_vector_j) *
                            (vq / distance(&Self::get_dot_position(i, cell, layer, cell_architecture), &Self::get_dot_position(j, cell, layer, cell_architecture)))
                        }
                        else{
                            0.0
                        }
                    }).sum::<f64>()
                }).sum::<f64>()
            }).sum()
    }
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone)]
pub struct FullBasisModelSettings{
    #[serde_inline_default(100)]
    num_samples: usize,

    #[serde_inline_default(100)]
    max_iter: usize,

    #[serde_inline_default(0.0000237177)]
    ampl_min: f64,

    #[serde_inline_default(2.0)]
    ampl_max: f64,

    #[serde_inline_default(2.0)]
    ampl_fac: f64,

    #[serde_inline_default(1e-3)]
    convergence_tolerance: f64,

    #[serde_inline_default(10.0)]
    relative_permitivity: f64,
}

pub struct FullBasisModel {
    clock_states: [f64; 4],
    input_states: Vec<f64>,
    settings: FullBasisModelSettings,
    index_cells_map: HashMap<QCACellIndex, QCACellInternal>,
    layer_map: HashMap<usize, QCALayer>,
    cell_architectures_map: HashMap<String, QCACellArchitecture>,
    cell_input_map: HashMap<QCACellIndex, usize>,
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
            settings: FullBasisModelSettings::new(),
            index_cells_map: HashMap::new(),
            layer_map: HashMap::new(),
            cell_architectures_map: HashMap::new(),
            cell_input_map: HashMap::new()
        }
    }
}

impl SimulationModelTrait for FullBasisModel{
    fn get_name(&self) -> String {
        "Full basis".into()
    }

    fn get_unique_id(&self) -> String {
        "full_basis_model".into()
    }

    fn get_settings(&self) -> Box<dyn SimulationModelSettingsTrait> {
        Box::new(self.settings.clone()) as Box<dyn SimulationModelSettingsTrait>
    }

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

    fn initiate(&mut self, layers: Box<Vec<QCALayer>>, qca_architetures_map: HashMap<String, QCACellArchitecture>) {
        self.index_cells_map.clear();
        self.layer_map.clear();
        self.cell_architectures_map = qca_architetures_map.clone();

        let mut cell_input_cnt = 0;

        layers.iter().enumerate().for_each(|(i, layer)| {
            self.layer_map.insert(i, layer.clone());
            layer.cells.iter().enumerate().for_each(|(j, c)| {
                self.index_cells_map.insert(
                    QCACellIndex::new(i, j),
                    QCACellInternal::new(Box::new(c.clone()), layer, qca_architetures_map.get(&layer.cell_architecture_id).unwrap(), self.settings.relative_permitivity)
                );
                if c.typ == CellType::Input {
                    self.cell_input_map.insert(QCACellIndex::new(i, j), cell_input_cnt);
                    cell_input_cnt += 1;
                }
            })
        });
    }

    fn pre_calculate(&mut self, clock_states: &[f64; 4], input_states: &Vec<f64>) {
        self.clock_states = clock_states.clone();
        self.input_states = input_states.clone();
    }

    fn calculate(&mut self, cell_ind: QCACellIndex) -> bool {
        let layer = self.layer_map.get(&cell_ind.layer).unwrap();
        let cell_architecture = self.cell_architectures_map.get(&layer.cell_architecture_id).unwrap();
        let n = cell_architecture.dot_count as usize;
        let ro_plus = 2.0 / n as f64;

        let mut internal_cell = self.index_cells_map.get(&cell_ind).unwrap().clone();
        let clock_index = (internal_cell.cell.clock_phase_shift.rem_euclid(360.0) / 90.0) as usize;
        let clock_value = self.clock_states[clock_index];

        let old_charge_probability = internal_cell.dot_charge_probability.clone();

        internal_cell.hamilton_matrix =
            &internal_cell.static_hamilton_matrix + &internal_cell.dynamic_hamilton_matrix * clock_value;

        match internal_cell.cell.typ {
            CellType::Input => {
                let cell_state_num = n / 4;
                let input_i = self.cell_input_map.get(&cell_ind).unwrap();
                let input = self.input_states[(cell_state_num * input_i)..(cell_state_num * input_i + cell_state_num)].to_vec();
                let input_distribution = polarization_to_dot_probability_distribution(input.as_slice());
                internal_cell.dot_charge_probability = DVector::from_vec(input_distribution);
            },
            CellType::Fixed => {
                internal_cell.dot_charge_probability = DVector::from_vec(internal_cell.cell.dot_probability_distribution.clone());
            },
            CellType::Normal | CellType::Output => {
                internal_cell.dot_potential = DVector::zeros(n);
                for (ind, c) in self.index_cells_map.iter(){
                    if *ind != cell_ind{
                        for i in 0..n {
                            for j in 0..n{
                                let dot_pos_i = QCACellInternal::get_dot_position(i, &internal_cell.cell, layer, cell_architecture);
                                let dot_pos_j = QCACellInternal::get_dot_position(j, &c.cell, layer, cell_architecture);

                                let distance = distance(&dot_pos_i,& dot_pos_j);

                                internal_cell.dot_potential[i] += (
                                    calculate_vq(self.settings.relative_permitivity) *
                                    (c.dot_charge_probability[j] - ro_plus)
                                ) / distance;
                            }
                        }
                    }
                }
            }
        }

        if matches!(internal_cell.cell.typ, CellType::Normal | CellType::Output)
        {

            for i in 0..n*n {
                internal_cell.hamilton_matrix[(i, i)] = QCACellInternal::hamilton_term_1(
                    n, 1.0,
                    internal_cell.dot_potential.as_view(),
                    internal_cell.basis_matrix.row(i).transpose().as_view(),
                    internal_cell.basis_matrix.row(i).transpose().as_view(),
                ) + &internal_cell.static_hamilton_matrix[(i, i)];
            }

            if (clock_value - self.settings.ampl_max).abs() >= 1e-3 {
                if let Some(decomposition) = Schur::try_new(internal_cell.hamilton_matrix.clone(), 1e-3, 100){
                    if let Some(eigenvalues) =  decomposition.eigenvalues() {
                        let sorted_eigenvalue = eigenvalues.iter()
                            .enumerate().min_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap();

                        let psi = decomposition.unpack().0.column(sorted_eigenvalue.0)
                            .map(|value| value.powf(2.0));

                        internal_cell.dot_charge_probability = DVector::from_vec((0..n).map(|i| {
                            let mut charge_probability = 0.0;
                            for j in 0..n*n{
                                for spin in 0..=1 {
                                    charge_probability +=
                                        QCACellInternal::count_operator(
                                            n, i, spin,
                                            internal_cell.basis_matrix.row(j).transpose().as_view()
                                        )
                                        *
                                        psi[j];
                                }
                            }
                            charge_probability
                        }).collect());
                    }
                }
            }
        }

        let mut stable: bool = true;
        for i in 0..n as usize {
            if (internal_cell.dot_charge_probability[i] - old_charge_probability[i]).abs() > self.settings.convergence_tolerance{
                stable = false;
            }
        }

        self.index_cells_map.insert(cell_ind, internal_cell);

        return stable;
    }
    
    fn get_states(&self, cell_ind: QCACellIndex) -> Vec<f64> {
        self.index_cells_map.get(&cell_ind).unwrap().dot_charge_probability.data.as_vec().to_vec()
    }

}