use super::{CellType, QCACellArchitecture, QCACellIndex, QCALayer, SimulationModelTrait};
use crate::objects::cell::{polarization_to_dot_probability_distribution, QCACell};
use crate::simulation::model::{ClockGeneratorSettingsTrait, SimulationModelSettingsTrait};
use crate::simulation::settings::{InputDescriptor, OptionsEntry, OptionsList};
use nalgebra::{distance, DMatrix, DMatrixView, DVector, DVectorView, Point3, Schur};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use std::collections::HashMap;
use std::mem;

const E_CHARGE: f64 = 1.602_176_634e-19; // Coulombs [C]
const EPS_0: f64 = 8.854_187_8128e-12; // F/m [C^2 / N·m^2]
const EV_PER_J: f64 = 1.0 / 1.602_176_634e-19; // eV per Joule

fn calculate_vq(relative_permittivity: f64) -> f64 {
    let u_joule = E_CHARGE.powi(2) / (4.0 * std::f64::consts::PI * EPS_0 * relative_permittivity);
    let vq_m = u_joule * EV_PER_J; // Joule/m -> eV/m
    (vq_m * 1_000.0) / 1e-9 // eV/m -> meV/nm
}

#[derive(Debug, Clone)]
pub struct QCACellInternal {
    cell: Box<QCACell>,
    _z_position: f64,

    //The full hamilton matrix
    hamilton_matrix: DMatrix<f64>,
    //Static hamilton matrix (1, 3, 4)
    static_hamilton_matrix: DMatrix<f64>,
    //Dynamic hamilton matrix (2)
    dynamic_hamilton_matrix: DMatrix<f64>,

    //Potential energy at each dot
    dot_potential: DVector<f64>,
    dot_charge_probability: DVector<f64>,

    basis_matrix: DMatrix<f64>,
}

impl QCACellInternal {
    pub fn new(
        cell: Box<QCACell>,
        layer: &QCALayer,
        cell_architecture: &QCACellArchitecture,
        relative_permittivity: f64,
    ) -> Self {
        let n: usize = cell_architecture.dot_count as usize;
        let tunneling_matrix = Self::generate_tunneling_matrix(&cell_architecture, 1.0);
        let basis_matrix = Self::generate_basis_matrix(n);
        let dot_potential: DVector<f64> = DVector::zeros(n);

        let vq = calculate_vq(relative_permittivity);
        let eq = vq / (cell_architecture.dot_diameter / 3.0);

        let static_hamilton_matrix = DMatrix::<f64>::from_iterator(
            n * n,
            n * n,
            (0..n * n)
                .map(|i| {
                    (0..n * n)
                        .map(|j| {
                            Self::hamilton_term_1(
                                n,
                                1.0,
                                dot_potential.as_view(),
                                basis_matrix.row(i).transpose().as_view(),
                                basis_matrix.row(j).transpose().as_view(),
                            ) + Self::hamilton_term_3(
                                n,
                                eq,
                                basis_matrix.row(i).transpose().as_view(),
                                basis_matrix.row(j).transpose().as_view(),
                            ) + Self::hamilton_term_4(
                                &cell,
                                &layer,
                                &cell_architecture,
                                vq,
                                basis_matrix.row(i).transpose().as_view(),
                                basis_matrix.row(j).transpose().as_view(),
                            )
                        })
                        .collect::<Vec<f64>>()
                })
                .flatten(),
        );

        let dynamic_hamilton_matrix = DMatrix::<f64>::from_iterator(
            n * n,
            n * n,
            (0..n * n)
                .map(|i| {
                    (0..n * n)
                        .map(|j| {
                            Self::hamilton_term_2(
                                n,
                                tunneling_matrix.as_view(),
                                basis_matrix.row(i).transpose().as_view(),
                                basis_matrix.row(j).transpose().as_view(),
                            )
                        })
                        .collect::<Vec<f64>>()
                })
                .flatten(),
        );

        QCACellInternal {
            cell: cell,
            _z_position: layer.z_position,
            hamilton_matrix: &static_hamilton_matrix + &dynamic_hamilton_matrix,
            static_hamilton_matrix: static_hamilton_matrix,
            dynamic_hamilton_matrix: dynamic_hamilton_matrix,
            dot_potential: dot_potential,
            dot_charge_probability: DVector::<f64>::from_vec(vec![2.0 / n as f64; n]),
            basis_matrix: basis_matrix,
        }
    }

    fn get_dot_position(
        dot_index: usize,
        cell: &Box<QCACell>,
        layer: &QCALayer,
        cell_architecture: &QCACellArchitecture,
    ) -> Point3<f64> {
        let x = cell_architecture.dot_positions[dot_index][0];
        let y = cell_architecture.dot_positions[dot_index][1];

        Point3::new(
            cell.position[0] + x * cell.rotation.cos() - y * cell.rotation.sin(),
            cell.position[1] + y * cell.rotation.cos() + x * cell.rotation.sin(),
            layer.z_position,
        )
    }

    fn generate_tunneling_matrix(
        cell_architecture: &QCACellArchitecture,
        energy: f64,
    ) -> DMatrix<f64> {
        let mut tunneling_matrix = DMatrix::<f64>::zeros(
            cell_architecture.dot_count as usize,
            cell_architecture.dot_count as usize,
        );

        cell_architecture.dot_tunnels.iter().for_each(|(a, b)| {
            tunneling_matrix[(*a as usize, *b as usize)] = energy;
            tunneling_matrix[(*b as usize, *a as usize)] = energy;
        });

        tunneling_matrix
    }

    fn generate_basis_matrix(dot_count: usize) -> DMatrix<f64> {
        DMatrix::<f64>::from_iterator(
            dot_count * dot_count,
            dot_count * 2,
            (0..dot_count * 2)
                .map(|i| {
                    let mut column: Vec<f64>;
                    if i < dot_count {
                        column = vec![
                            vec![0.0; dot_count * (dot_count - i - 1)],
                            vec![1.0; dot_count],
                            vec![0.0; dot_count * i],
                        ]
                        .concat();
                    } else {
                        column = vec![0.0; dot_count * dot_count];
                        (0..dot_count).for_each(|j| {
                            column[dot_count * j + (2 * dot_count - i - 1)] = 1.0;
                        });
                    }
                    column
                })
                .flatten(),
        )
    }

    fn count_operator(
        dot_count: usize,
        dot_index: usize,
        spin: i32,
        basis_vector: DVectorView<f64>,
    ) -> f64 {
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

    fn capture_operator(dot_count: usize, dot_index: usize, basis_vector: DVectorView<f64>) -> f64 {
        if (basis_vector[dot_index] == 1.0) && (basis_vector[dot_index + dot_count] == 1.0) {
            1.0
        } else {
            0.0
        }
    }

    fn coulumb_operator(
        dot_count: usize,
        dot_i: usize,
        dot_j: usize,
        spin: i32,
        basis_vector: DVectorView<f64>,
    ) -> f64 {
        if dot_i == dot_j {
            panic!("Dot indicies cannot be equal!")
        }

        let n_spin;
        if spin == 1 {
            n_spin = 0
        } else {
            n_spin = 1
        };

        Self::count_operator(dot_count, dot_i, spin, basis_vector)
            * Self::count_operator(dot_count, dot_j, n_spin, basis_vector)
    }

    fn tunneling_operator(
        dot_count: usize,
        dot_i: usize,
        dot_j: usize,
        spin: i32,
        basis_vector: DVectorView<f64>,
    ) -> DVector<f64> {
        let mut tunneling_vector: DVector<f64> = basis_vector.clone_owned();

        if spin == 1 {
            tunneling_vector.swap((dot_i, 0), (dot_j, 0));
        } else {
            tunneling_vector.swap((dot_i + dot_count, 0), (dot_j + dot_count, 0));
        }

        if tunneling_vector == basis_vector {
            tunneling_vector = DVector::<f64>::zeros(dot_count * dot_count);
        }

        tunneling_vector
    }

    fn hamilton_term_1(
        dot_count: usize,
        e0: f64,
        dot_potential: DVectorView<f64>,
        basis_vector_i: DVectorView<f64>,
        basis_vector_j: DVectorView<f64>,
    ) -> f64 {
        (0..dot_count)
            .map(|i| {
                (0..=1)
                    .map(|spin| {
                        if basis_vector_i == basis_vector_j {
                            Self::count_operator(dot_count, i, spin, basis_vector_j)
                                * (e0 + dot_potential[i])
                        } else {
                            0.0
                        }
                    })
                    .sum::<f64>()
            })
            .sum()
    }

    fn hamilton_term_2(
        dot_count: usize,
        tunneling_matrix: DMatrixView<f64>,
        basis_vector_i: DVectorView<f64>,
        basis_vector_j: DVectorView<f64>,
    ) -> f64 {
        (0..dot_count)
            .map(|i| {
                (0..i)
                    .map(|j| {
                        if tunneling_matrix[(i, j)] != 0.0 {
                            (0..=1)
                                .map(|spin| {
                                    if Self::tunneling_operator(
                                        dot_count,
                                        i,
                                        j,
                                        spin,
                                        basis_vector_j,
                                    ) == basis_vector_i
                                    {
                                        tunneling_matrix[(i, j)]
                                    } else {
                                        0.0
                                    }
                                })
                                .sum()
                        } else {
                            0.0
                        }
                    })
                    .sum::<f64>()
            })
            .sum()
    }

    fn hamilton_term_3(
        dot_count: usize,
        eq: f64,
        basis_vector_i: DVectorView<f64>,
        basis_vector_j: DVectorView<f64>,
    ) -> f64 {
        (0..dot_count)
            .map(|i| {
                if basis_vector_i == basis_vector_j {
                    Self::capture_operator(dot_count, i, basis_vector_j) * eq
                } else {
                    0.0
                }
            })
            .sum()
    }

    fn hamilton_term_4(
        cell: &Box<QCACell>,
        layer: &QCALayer,
        cell_architecture: &QCACellArchitecture,
        vq: f64,
        basis_vector_i: DVectorView<f64>,
        basis_vector_j: DVectorView<f64>,
    ) -> f64 {
        (0..cell_architecture.dot_count as usize)
            .map(|i| {
                (0..i)
                    .map(|j| {
                        (0..=1)
                            .map(|spin| {
                                if basis_vector_i == basis_vector_j {
                                    Self::coulumb_operator(
                                        cell_architecture.dot_count as usize,
                                        i,
                                        j,
                                        spin,
                                        basis_vector_j,
                                    ) * (vq
                                        / distance(
                                            &Self::get_dot_position(
                                                i,
                                                cell,
                                                layer,
                                                cell_architecture,
                                            ),
                                            &Self::get_dot_position(
                                                j,
                                                cell,
                                                layer,
                                                cell_architecture,
                                            ),
                                        ))
                                } else {
                                    0.0
                                }
                            })
                            .sum::<f64>()
                    })
                    .sum::<f64>()
            })
            .sum()
    }
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone)]
pub struct ICHAModelSettings {
    #[serde_inline_default(1_000)]
    max_iterations: usize,

    #[serde_inline_default(1e-6)]
    convergence_tolerance: f64,

    #[serde_inline_default(12.9)]
    relative_permitivity: f64,

    #[serde_inline_default(1_000)]
    schur_max_iterations: usize,

    #[serde_inline_default(1e-6)]
    schur_convergence_tolerance: f64,
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone)]
pub struct ICHAClockGeneratorSettings {
    #[serde_inline_default(1)]
    num_cycles: usize,

    #[serde_inline_default(0.0000237177)]
    amplitude_min: f64,

    #[serde_inline_default(2.0)]
    amplitude_max: f64,

    #[serde_inline_default(0)]
    extra_periods: usize,

    #[serde_inline_default(20)]
    samples_per_input: usize,
}

pub struct ICHAModel {
    clock_states: [f64; 4],
    input_states: Vec<f64>,
    model_settings: ICHAModelSettings,
    clock_generator_settings: ICHAClockGeneratorSettings,
    layer_map: HashMap<usize, QCALayer>,
    cell_architectures_map: HashMap<String, QCACellArchitecture>,
    cell_input_map: HashMap<QCACellIndex, usize>,
    index_cells_static_map: HashMap<QCACellIndex, QCACellInternal>,
    index_cells_read_map: HashMap<QCACellIndex, QCACellInternal>,
    index_cells_write_map: HashMap<QCACellIndex, QCACellInternal>,
}

impl ICHAModelSettings {
    pub fn new() -> Self {
        serde_json::from_str::<ICHAModelSettings>("{}".into()).unwrap()
    }
}

impl ICHAClockGeneratorSettings {
    pub fn new() -> Self {
        serde_json::from_str::<ICHAClockGeneratorSettings>("{}".into()).unwrap()
    }
}

impl SimulationModelSettingsTrait for ICHAModelSettings {
    fn get_max_iterations(&self) -> usize {
        self.max_iterations
    }
    fn get_convergence_tolerance(&self) -> f64 {
        self.convergence_tolerance
    }
}

impl ClockGeneratorSettingsTrait for ICHAClockGeneratorSettings {
    fn get_num_cycles(&self) -> usize {
        self.num_cycles
    }
    fn get_amplitude_min(&self) -> f64 {
        self.amplitude_min
    }
    fn get_amplitude_max(&self) -> f64 {
        self.amplitude_max
    }
    fn get_extra_periods(&self) -> usize {
        self.extra_periods
    }
    fn get_samples_per_input(&self) -> usize {
        self.samples_per_input
    }
}

impl ICHAModel {
    pub fn new() -> Self {
        ICHAModel {
            clock_states: [0.0, 0.0, 0.0, 0.0],
            input_states: vec![],
            model_settings: ICHAModelSettings::new(),
            clock_generator_settings: ICHAClockGeneratorSettings::new(),
            layer_map: HashMap::new(),
            cell_architectures_map: HashMap::new(),
            cell_input_map: HashMap::new(),
            index_cells_static_map: HashMap::new(),
            index_cells_read_map: HashMap::new(),
            index_cells_write_map: HashMap::new(),
        }
    }
}

impl SimulationModelTrait for ICHAModel {
    fn get_name(&self) -> String {
        "Intercellular Hartree Approximation".into()
    }

    fn get_unique_id(&self) -> String {
        "icha_model".into()
    }

    fn get_model_settings(&self) -> Box<dyn SimulationModelSettingsTrait> {
        Box::new(self.model_settings.clone()) as Box<dyn SimulationModelSettingsTrait>
    }

    fn get_clock_generator_settings(&self) -> Box<dyn ClockGeneratorSettingsTrait> {
        Box::new(self.clock_generator_settings.clone()) as Box<dyn ClockGeneratorSettingsTrait>
    }

    fn get_model_options_list(&self) -> OptionsList {
        vec![
            OptionsEntry::Input {
                unique_id: "max_iterations".into(),
                name: "Max Iterations".into(),
                description: "Maximum number of iterations for the simulation".into(),
                descriptor: InputDescriptor::NumberInput {
                    min: Some(1.0),
                    max: None,
                    unit: None,
                    whole_num: true,
                },
            },
            OptionsEntry::Input {
                unique_id: "convergence_tolerance".into(),
                name: "Convergence Tolerance".into(),
                description: "Tolerance value for convergence check".into(),
                descriptor: InputDescriptor::NumberInput {
                    min: Some(0.0),
                    max: None,
                    unit: None,
                    whole_num: false,
                },
            },
            OptionsEntry::Input {
                unique_id: "relative_permitivity".into(),
                name: "Relative Permittivity".into(),
                description: "Relative permittivity value for electric field calculations".into(),
                descriptor: InputDescriptor::NumberInput {
                    min: Some(0.0),
                    max: None,
                    unit: None,
                    whole_num: false,
                },
            },
            OptionsEntry::Header {
                label: "Schur decomposition calculation settings".into(),
            },
            OptionsEntry::Input {
                unique_id: "schur_max_iterations".into(),
                name: "Maximum iterations".into(),
                description: "Maximum iterations for Schur decomposition".into(),
                descriptor: InputDescriptor::NumberInput {
                    min: Some(1.0),
                    max: None,
                    unit: None,
                    whole_num: true,
                },
            },
            OptionsEntry::Input {
                unique_id: "schur_convergence_tolerance".into(),
                name: "Convergence tolerance".into(),
                description: "Tolerance value for Schur decomposition convergence".into(),
                descriptor: InputDescriptor::NumberInput {
                    min: Some(0.0),
                    max: None,
                    unit: None,
                    whole_num: false,
                },
            },
        ]
    }

    fn get_clock_generator_options_list(&self) -> OptionsList {
        vec![
            OptionsEntry::Input {
                unique_id: "num_cycles".into(),
                name: "Number of Cycles".into(),
                description: "Number of clock cycles for simulation".into(),
                descriptor: InputDescriptor::NumberInput {
                    min: Some(1.0),
                    max: None,
                    unit: None,
                    whole_num: true,
                },
            },
            OptionsEntry::Input {
                unique_id: "amplitude_min".into(),
                name: "Amplitude Minimum".into(),
                description: "Minimum amplitude for clock signals".into(),
                descriptor: InputDescriptor::NumberInput {
                    min: None,
                    max: None,
                    unit: None,
                    whole_num: false,
                },
            },
            OptionsEntry::Input {
                unique_id: "amplitude_max".into(),
                name: "Amplitude Maximum".into(),
                description: "Maximum amplitude for clock signals".into(),
                descriptor: InputDescriptor::NumberInput {
                    min: None,
                    max: None,
                    unit: None,
                    whole_num: false,
                },
            },
            OptionsEntry::Input {
                unique_id: "extra_periods".into(),
                name: "Extra clock periods".into(),
                description: "Number of extra clock periods at the end".into(),
                descriptor: InputDescriptor::NumberInput {
                    min: Some(0.0),
                    max: None,
                    unit: None,
                    whole_num: true,
                },
            },
            OptionsEntry::Input {
                unique_id: "samples_per_input".into(),
                name: "Samples Per Input".into(),
                description: "Number of samples to take per input value".into(),
                descriptor: InputDescriptor::NumberInput {
                    min: Some(1.0),
                    max: None,
                    unit: None,
                    whole_num: true,
                },
            },
        ]
    }

    fn serialize_model_settings(&self) -> Result<String, String> {
        match serde_json::to_string(&self.model_settings) {
            Ok(res) => Ok(res),
            Err(err) => Err(err.to_string()),
        }
    }

    fn deserialize_model_settings(&mut self, settings_str: &String) -> Result<(), String> {
        match serde_json::from_str::<ICHAModelSettings>(settings_str) {
            Ok(res) => {
                self.model_settings = res;
                Ok(())
            }
            Err(err) => Err(err.to_string()),
        }
    }

    fn serialize_clock_generator_settings(&self) -> Result<String, String> {
        match serde_json::to_string(&self.clock_generator_settings) {
            Ok(res) => Ok(res),
            Err(err) => Err(err.to_string()),
        }
    }

    fn deserialize_clock_generator_settings(
        &mut self,
        settings_str: &String,
    ) -> Result<(), String> {
        match serde_json::from_str::<ICHAClockGeneratorSettings>(settings_str) {
            Ok(res) => {
                self.clock_generator_settings = res;
                Ok(())
            }
            Err(err) => Err(err.to_string()),
        }
    }

    fn initiate(
        &mut self,
        layers: Box<Vec<QCALayer>>,
        qca_architetures_map: HashMap<String, QCACellArchitecture>,
    ) {
        self.index_cells_read_map.clear();
        self.index_cells_write_map.clear();
        self.layer_map.clear();
        self.cell_architectures_map = qca_architetures_map.clone();

        let mut cell_input_cnt = 0;

        layers.iter().enumerate().for_each(|(i, layer)| {
            self.layer_map.insert(i, layer.clone());
            layer.cells.iter().enumerate().for_each(|(j, c)| {
                let mut internal = QCACellInternal::new(
                    Box::new(c.clone()),
                    layer,
                    qca_architetures_map
                        .get(&layer.cell_architecture_id)
                        .unwrap(),
                    self.model_settings.relative_permitivity,
                );

                match c.typ {
                    CellType::Normal | CellType::Output => {
                        self.index_cells_write_map
                            .insert(QCACellIndex::new(i, j), internal);
                    }
                    CellType::Input => {
                        self.index_cells_static_map
                            .insert(QCACellIndex::new(i, j), internal.clone());
                        self.cell_input_map
                            .insert(QCACellIndex::new(i, j), cell_input_cnt);
                        cell_input_cnt += 1;
                    }
                    CellType::Fixed => {
                        internal.dot_charge_probability =
                            DVector::from_vec(internal.cell.dot_probability_distribution.clone());
                        self.index_cells_static_map
                            .insert(QCACellIndex::new(i, j), internal.clone());
                    }
                }
            })
        });

        self.index_cells_read_map = self.index_cells_write_map.clone();
    }

    fn pre_calculate(&mut self, clock_states: &[f64; 4], input_states: &Vec<f64>) {
        self.clock_states = clock_states.clone();
        self.input_states = input_states.clone();
        mem::swap(
            &mut self.index_cells_read_map,
            &mut self.index_cells_write_map,
        );
        self.index_cells_write_map = self.index_cells_read_map.clone();

        self.index_cells_static_map
            .iter_mut()
            .for_each(|(ind, cell)| {
                if let Some(input_i) = self.cell_input_map.get(&ind) {
                    let layer = self.layer_map.get(&ind.layer).unwrap();
                    let cell_architecture = self
                        .cell_architectures_map
                        .get(&layer.cell_architecture_id)
                        .unwrap();
                    let n = cell_architecture.dot_count as usize;
                    let cell_state_num = n / 4;

                    let input = self.input_states
                        [(cell_state_num * input_i)..(cell_state_num * input_i + cell_state_num)]
                        .to_vec();
                    let input_distribution =
                        polarization_to_dot_probability_distribution(input.as_slice());
                    cell.dot_charge_probability = DVector::from_vec(input_distribution);
                }
            });
    }

    fn calculate(&mut self, cell_ind: QCACellIndex) -> bool {
        let cell_option = self.index_cells_read_map.get(&cell_ind);

        if cell_option.is_none() {
            return true;
        }

        let mut internal_cell = cell_option.unwrap().clone();
        let old_charge_probability = internal_cell.dot_charge_probability.clone();

        let layer = self.layer_map.get(&cell_ind.layer).unwrap();
        let cell_architecture = self
            .cell_architectures_map
            .get(&layer.cell_architecture_id)
            .unwrap();
        let n = cell_architecture.dot_count as usize;
        let ro_plus = 2.0 / n as f64;

        let clock_index = (internal_cell.cell.clock_phase_shift.rem_euclid(360.0) / 90.0) as usize;
        let clock_value = self.clock_states[clock_index];

        internal_cell.hamilton_matrix = &internal_cell.static_hamilton_matrix
            + &internal_cell.dynamic_hamilton_matrix * clock_value;

        internal_cell.dot_potential = DVector::zeros(n);
        for (ind, c) in self
            .index_cells_static_map
            .iter()
            .chain(self.index_cells_read_map.iter())
        {
            if *ind != cell_ind {
                for i in 0..n {
                    for j in 0..n {
                        let dot_pos_i = QCACellInternal::get_dot_position(
                            i,
                            &internal_cell.cell,
                            layer,
                            cell_architecture,
                        );
                        let dot_pos_j =
                            QCACellInternal::get_dot_position(j, &c.cell, layer, cell_architecture);

                        let distance = distance(&dot_pos_i, &dot_pos_j);

                        internal_cell.dot_potential[i] +=
                            (calculate_vq(self.model_settings.relative_permitivity)
                                * (c.dot_charge_probability[j] - ro_plus))
                                / distance;
                    }
                }
            }
        }

        for i in 0..n * n {
            internal_cell.hamilton_matrix[(i, i)] = QCACellInternal::hamilton_term_1(
                n,
                1.0,
                internal_cell.dot_potential.as_view(),
                internal_cell.basis_matrix.row(i).transpose().as_view(),
                internal_cell.basis_matrix.row(i).transpose().as_view(),
            ) + &internal_cell.static_hamilton_matrix
                [(i, i)];
        }

        if (clock_value - self.clock_generator_settings.amplitude_max).abs() >= 1e-3 {
            if let Some(decomposition) = Schur::try_new(
                internal_cell.hamilton_matrix.clone(),
                self.model_settings.schur_convergence_tolerance,
                self.model_settings.schur_max_iterations,
            ) {
                if let Some(eigenvalues) = decomposition.eigenvalues() {
                    let sorted_eigenvalue = eigenvalues
                        .iter()
                        .enumerate()
                        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                        .unwrap();

                    let psi = decomposition
                        .unpack()
                        .0
                        .column(sorted_eigenvalue.0)
                        .map(|value| value.powf(2.0));

                    internal_cell.dot_charge_probability = DVector::from_vec(
                        (0..n)
                            .map(|i| {
                                let mut charge_probability = 0.0;
                                for j in 0..n * n {
                                    for spin in 0..=1 {
                                        charge_probability += QCACellInternal::count_operator(
                                            n,
                                            i,
                                            spin,
                                            internal_cell.basis_matrix.row(j).transpose().as_view(),
                                        ) * psi[j];
                                    }
                                }
                                charge_probability
                            })
                            .collect(),
                    );
                }
            }
        }

        let mut stable: bool = true;
        for i in 0..n as usize {
            if (internal_cell.dot_charge_probability[i] - old_charge_probability[i]).abs()
                > self.model_settings.convergence_tolerance
            {
                stable = false;
            }
        }

        self.index_cells_write_map.insert(cell_ind, internal_cell);

        stable
    }

    fn get_states(&self, cell_ind: &QCACellIndex) -> Vec<f64> {
        if let Some(c) = self.index_cells_write_map.get(cell_ind) {
            return c.dot_charge_probability.data.as_vec().to_vec();
        }
        if let Some(c) = self.index_cells_read_map.get(cell_ind) {
            return c.dot_charge_probability.data.as_vec().to_vec();
        }
        if let Some(c) = self.index_cells_static_map.get(cell_ind) {
            return c.dot_charge_probability.data.as_vec().to_vec();
        }
        panic!("Cell not found");
    }
}
