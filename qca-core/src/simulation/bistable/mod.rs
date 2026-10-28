use super::{CellType, QCACellArchitecture, SimulationModelTrait};
use crate::objects::cell::{
    dot_probability_distribution_to_polarization, polarization_to_dot_probability_distribution,
    QCACell, QCACellIndex,
};
use crate::objects::layer::QCALayer;
use crate::simulation::model::{ClockGeneratorSettingsTrait, SimulationModelSettingsTrait};
use crate::simulation::settings::{InputDescriptor, OptionsEntry, OptionsList};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use std::{
    collections::{BTreeMap, HashMap},
    mem,
};

struct BistableNeighbor {
    cell_index: QCACellIndex,
    kink_energy: f64,
}

pub struct BistableModel {
    clock_states: [f64; 4],
    input_states: Vec<f64>,
    index_cells_static_map: BTreeMap<QCACellIndex, QCACell>,
    index_cells_read_map: BTreeMap<QCACellIndex, QCACell>,
    index_cells_write_map: BTreeMap<QCACellIndex, QCACell>,
    cell_input_map: BTreeMap<QCACellIndex, usize>,
    active_layer: i8,
    polarizations: [Vec<f64>; 2],
    neighborhood_map: BTreeMap<QCACellIndex, Vec<BistableNeighbor>>,
    model_settings: BistableModelSettings,
    clock_settings: BistableClockGeneratorSettings,
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone)]
pub struct BistableModelSettings {
    #[serde_inline_default(1000)]
    max_iterations: usize,

    #[serde_inline_default(1e-3)]
    convergence_tolerance: f64,

    #[serde_inline_default(65.0)]
    neighborhood_radius: f64,

    #[serde_inline_default(12.9)]
    relative_permitivity: f64,
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone)]
pub struct BistableClockGeneratorSettings {
    #[serde_inline_default(1)]
    num_cycles: usize,

    // QCADesigner's bistable clock low/high (3.8e-23 J and 9.8e-22 J) in meV.
    #[serde_inline_default(0.2372)]
    amplitude_min: f64,

    #[serde_inline_default(6.1167)]
    amplitude_max: f64,

    #[serde_inline_default(0)]
    extra_periods: usize,

    #[serde_inline_default(20)]
    samples_per_input: usize,
}

impl BistableModelSettings {
    pub fn new() -> Self {
        serde_json::from_str::<BistableModelSettings>("{}".into()).unwrap()
    }
}

impl SimulationModelSettingsTrait for BistableModelSettings {
    fn get_max_iterations(&self) -> usize {
        self.max_iterations
    }
    fn get_convergence_tolerance(&self) -> f64 {
        self.convergence_tolerance
    }
}

impl BistableClockGeneratorSettings {
    pub fn new() -> Self {
        serde_json::from_str::<BistableClockGeneratorSettings>("{}".into()).unwrap()
    }
}

impl ClockGeneratorSettingsTrait for BistableClockGeneratorSettings {
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

impl BistableModel {
    pub fn new() -> Self {
        BistableModel {
            clock_states: [0.0, 0.0, 0.0, 0.0],
            input_states: vec![],
            active_layer: 0,
            index_cells_static_map: BTreeMap::new(),
            index_cells_read_map: BTreeMap::new(),
            index_cells_write_map: BTreeMap::new(),
            cell_input_map: BTreeMap::new(),
            polarizations: [vec![], vec![]],
            neighborhood_map: BTreeMap::new(),
            model_settings: BistableModelSettings::new(),
            clock_settings: BistableClockGeneratorSettings::new(),
        }
    }

    fn cell_distance(cell_a: &QCACell, cell_b: &QCACell) -> f64 {
        ((cell_a.position[0] - cell_b.position[0]).powf(2.0)
            + (cell_a.position[1] - cell_b.position[1]).powf(2.0))
        .sqrt()
    }

    fn determine_kink_energy(cell_a: &QCACell, cell_b: &QCACell, permitivity: f64) -> f64 {
        const QCHARGE_SQUAR_OVER_FOUR: f64 = 6.41742353846709430467559076549e-39;
        const FOUR_PI_EPSILON: f64 = 1.11265005597565794635320037482e-10;

        const SAME_POLARIZATION: [[f64; 4]; 4] = [
            [
                QCHARGE_SQUAR_OVER_FOUR,
                -QCHARGE_SQUAR_OVER_FOUR,
                QCHARGE_SQUAR_OVER_FOUR,
                -QCHARGE_SQUAR_OVER_FOUR,
            ],
            [
                -QCHARGE_SQUAR_OVER_FOUR,
                QCHARGE_SQUAR_OVER_FOUR,
                -QCHARGE_SQUAR_OVER_FOUR,
                QCHARGE_SQUAR_OVER_FOUR,
            ],
            [
                QCHARGE_SQUAR_OVER_FOUR,
                -QCHARGE_SQUAR_OVER_FOUR,
                QCHARGE_SQUAR_OVER_FOUR,
                -QCHARGE_SQUAR_OVER_FOUR,
            ],
            [
                -QCHARGE_SQUAR_OVER_FOUR,
                QCHARGE_SQUAR_OVER_FOUR,
                -QCHARGE_SQUAR_OVER_FOUR,
                QCHARGE_SQUAR_OVER_FOUR,
            ],
        ];

        const DIFF_POLARIZATION: [[f64; 4]; 4] = [
            [
                -QCHARGE_SQUAR_OVER_FOUR,
                QCHARGE_SQUAR_OVER_FOUR,
                -QCHARGE_SQUAR_OVER_FOUR,
                QCHARGE_SQUAR_OVER_FOUR,
            ],
            [
                QCHARGE_SQUAR_OVER_FOUR,
                -QCHARGE_SQUAR_OVER_FOUR,
                QCHARGE_SQUAR_OVER_FOUR,
                -QCHARGE_SQUAR_OVER_FOUR,
            ],
            [
                -QCHARGE_SQUAR_OVER_FOUR,
                QCHARGE_SQUAR_OVER_FOUR,
                -QCHARGE_SQUAR_OVER_FOUR,
                QCHARGE_SQUAR_OVER_FOUR,
            ],
            [
                QCHARGE_SQUAR_OVER_FOUR,
                -QCHARGE_SQUAR_OVER_FOUR,
                QCHARGE_SQUAR_OVER_FOUR,
                -QCHARGE_SQUAR_OVER_FOUR,
            ],
        ];

        const DOT_OFFSET_X: [f64; 4] = [-4.5, 4.5, 4.5, -4.5];
        const DOT_OFFSET_Y: [f64; 4] = [-4.5, -4.5, 4.5, 4.5];

        let mut energy_same: f64 = 0.0;
        let mut energy_diff: f64 = 0.0;

        for i in 0..4 {
            for j in 0..4 {
                let x: f64 = f64::abs(
                    cell_a.position[0] + DOT_OFFSET_X[i] - (cell_b.position[0] + DOT_OFFSET_X[j]),
                );
                let y: f64 = f64::abs(
                    cell_a.position[1] + DOT_OFFSET_Y[i] - (cell_b.position[1] + DOT_OFFSET_Y[j]),
                );

                let dist = 1e-9 * f64::sqrt(x * x + y * y);

                energy_diff += DIFF_POLARIZATION[i][j] / dist;
                energy_same += SAME_POLARIZATION[i][j] / dist;
            }
        }

        const JOULES_PER_MEV: f64 = 1.602176634e-22;

        (1.0 / (FOUR_PI_EPSILON * permitivity)) * (energy_diff - energy_same) / JOULES_PER_MEV
    }
}

impl SimulationModelTrait for BistableModel {
    fn get_name(&self) -> String {
        "Bistable".into()
    }

    fn get_unique_id(&self) -> String {
        "bistable".into()
    }

    fn get_model_settings(&self) -> Box<dyn SimulationModelSettingsTrait> {
        Box::new(self.model_settings.clone()) as Box<dyn SimulationModelSettingsTrait>
    }

    fn get_clock_generator_settings(&self) -> Box<dyn ClockGeneratorSettingsTrait> {
        Box::new(self.clock_settings.clone()) as Box<dyn ClockGeneratorSettingsTrait>
    }

    fn get_model_options_list(&self) -> OptionsList {
        vec![
            OptionsEntry::Input {
                unique_id: "max_iterations".to_string(),
                name: "Maximum iterations".to_string(),
                description:
                    "The maximum number of iterations used for simulation convergence check"
                        .to_string(),
                descriptor: InputDescriptor::NumberInput {
                    min: Some(1.0),
                    max: None,
                    unit: None,
                    whole_num: true,
                    choices: None,
                },
            },
            OptionsEntry::Input {
                unique_id: "convergence_tolerance".to_string(),
                name: "Convergence tolerance".to_string(),
                description: "Tolerance for simulation convergence check".to_string(),
                descriptor: InputDescriptor::NumberInput {
                    min: Some(0.0),
                    max: Some(1.0),
                    unit: None,
                    whole_num: false,
                    choices: None,
                },
            },
            OptionsEntry::Input {
                unique_id: "neighborhood_radius".to_string(),
                name: "Radius of effect".to_string(),
                description: "Radius of effect for neighbouring cells".to_string(),
                descriptor: InputDescriptor::NumberInput {
                    min: Some(0.0),
                    max: None,
                    unit: Some("nm".into()),
                    whole_num: false,
                    choices: None,
                },
            },
            OptionsEntry::Input {
                unique_id: "relative_permitivity".to_string(),
                name: "Relative permitivity".to_string(),
                description: "Relative permitivity of the relative medium".to_string(),
                descriptor: InputDescriptor::NumberInput {
                    min: Some(0.0),
                    max: None,
                    unit: None,
                    whole_num: false,
                    choices: None,
                },
            },
        ]
    }

    fn get_clock_generator_options_list(&self) -> OptionsList {
        vec![
            OptionsEntry::Input {
                unique_id: "num_cycles".to_string(),
                name: "Number of cycles".to_string(),
                description: "The number of repeating clock cycles to run".to_string(),
                descriptor: InputDescriptor::NumberInput {
                    min: Some(1.0),
                    max: None,
                    unit: None,
                    whole_num: true,
                    choices: None,
                },
            },
            OptionsEntry::Input {
                unique_id: "amplitude_min".to_string(),
                name: "Minimum amplitude".to_string(),
                description: "Tunneling energy of the clock signal while cells hold their state (clock low)".to_string(),
                descriptor: InputDescriptor::NumberInput {
                    min: Some(0.0),
                    max: None,
                    unit: Some("meV".into()),
                    whole_num: false,
                    choices: None,
                },
            },
            OptionsEntry::Input {
                unique_id: "amplitude_max".to_string(),
                name: "Maximum amplitude".to_string(),
                description: "Tunneling energy of the clock signal while cells are released (clock high)".to_string(),
                descriptor: InputDescriptor::NumberInput {
                    min: Some(0.0),
                    max: None,
                    unit: Some("meV".into()),
                    whole_num: false,
                    choices: None,
                },
            },
            OptionsEntry::Input {
                unique_id: "extra_periods".to_string(),
                name: "Extra clock periods".to_string(),
                description: "Extra full clock cycles (phases 0/90/180/270) to run after the input sweep, giving cells with a large clock-phase delay - e.g. far down a propagating wire - time to settle before the simulation ends. Actual extra cycles simulated = this value x the cell architecture's polarization count (1 for two-state cells, 2 for tri/quad-state cells).".to_string(),
                descriptor: InputDescriptor::NumberInput {
                    min: Some(0.0),
                    max: None,
                    unit: None,
                    whole_num: true,
                    choices: None,
                },
            },
            OptionsEntry::Input {
                unique_id: "samples_per_input".to_string(),
                name: "Samples per input".to_string(),
                description: "Number of samples to be simulated for each input combination"
                    .to_string(),
                descriptor: InputDescriptor::NumberInput {
                    min: Some(1.0),
                    max: None,
                    unit: None,
                    whole_num: true,
                    choices: None,
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
        match serde_json::from_str::<BistableModelSettings>(settings_str) {
            Ok(res) => {
                self.model_settings = res;
                Ok(())
            }
            Err(err) => Err(err.to_string()),
        }
    }

    fn serialize_clock_generator_settings(&self) -> Result<String, String> {
        match serde_json::to_string(&self.clock_settings) {
            Ok(res) => Ok(res),
            Err(err) => Err(err.to_string()),
        }
    }

    fn deserialize_clock_generator_settings(
        &mut self,
        settings_str: &String,
    ) -> Result<(), String> {
        match serde_json::from_str::<BistableClockGeneratorSettings>(settings_str) {
            Ok(res) => {
                self.clock_settings = res;
                Ok(())
            }
            Err(err) => Err(err.to_string()),
        }
    }

    fn initiate(
        &mut self,
        layers: Box<Vec<QCALayer>>,
        _qca_architetures_map: HashMap<String, QCACellArchitecture>,
    ) {
        self.index_cells_static_map.clear();
        self.index_cells_read_map.clear();
        self.cell_input_map.clear();
        let mut input_count = 0;
        layers.iter().enumerate().for_each(|(i, layer)| {
            layer.cells.iter().enumerate().for_each(|(j, cell)| {
                let cell_index = QCACellIndex::new(i, j);
                match cell.typ {
                    CellType::Input | CellType::Fixed => {
                        self.index_cells_static_map
                            .insert(cell_index.clone(), cell.clone());
                        if cell.typ == CellType::Input {
                            self.cell_input_map.insert(cell_index.clone(), input_count);
                            input_count += 1;
                        }
                    }
                    CellType::Normal | CellType::Output => {
                        self.index_cells_read_map
                            .insert(cell_index.clone(), cell.clone());
                    }
                }
            })
        });
        self.index_cells_write_map = self.index_cells_read_map.clone();

        let all_cells_iter = self
            .index_cells_static_map
            .iter()
            .chain(self.index_cells_read_map.iter());

        let tmp_polarizations: Vec<f64> = all_cells_iter
            .clone()
            .map(|(_, c)| {
                (c.dot_probability_distribution[0] - c.dot_probability_distribution[1]
                    + c.dot_probability_distribution[2]
                    - c.dot_probability_distribution[3])
                    / c.dot_probability_distribution.iter().sum::<f64>()
            })
            .collect();

        self.active_layer = 0;
        self.polarizations = [tmp_polarizations.clone(), tmp_polarizations.clone()];

        let permitivity = self.model_settings.relative_permitivity;
        self.neighborhood_map.clear();
        all_cells_iter.clone().for_each(|(index_i, cell_i)| {
            all_cells_iter.clone().for_each(|(index_j, cell_j)| {
                if (index_i != index_j)
                    && BistableModel::cell_distance(cell_i, cell_j)
                        <= self.model_settings.neighborhood_radius
                {
                    if !self.neighborhood_map.contains_key(&index_i) {
                        self.neighborhood_map.insert(index_i.clone(), vec![]);
                    }

                    let kink_energy =
                        BistableModel::determine_kink_energy(cell_i, cell_j, permitivity);
                    self.neighborhood_map
                        .get_mut(index_i)
                        .unwrap()
                        .push(BistableNeighbor {
                            cell_index: index_j.clone(),
                            kink_energy,
                        });
                }
            })
        });
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
            .for_each(|(index, cell)| {
                if cell.typ == CellType::Input {
                    let input_index = self.cell_input_map.get(index).unwrap();
                    let polarization = input_states[*input_index];
                    cell.dot_probability_distribution =
                        polarization_to_dot_probability_distribution(&[polarization]);
                }
            });
    }

    fn calculate(&mut self, cell_ind: QCACellIndex) -> bool {
        let Some(cell) = self.index_cells_write_map.get(&cell_ind) else {
            return true;
        };

        let mut polar_math = self
            .neighborhood_map
            .get(&cell_ind)
            .map(|neighbours| neighbours.as_slice())
            .unwrap_or_default()
            .iter()
            .map(|neighbour| {
                let neighbour_cell = {
                    if let Some(neighbour_cell) =
                        self.index_cells_read_map.get(&neighbour.cell_index)
                    {
                        neighbour_cell
                    } else if let Some(neighbour_cell) =
                        self.index_cells_static_map.get(&neighbour.cell_index)
                    {
                        neighbour_cell
                    } else {
                        panic!("Unknown neighbour");
                    }
                };
                let neighbour_polarization = dot_probability_distribution_to_polarization(
                    &neighbour_cell.dot_probability_distribution,
                )[0];
                neighbour.kink_energy * neighbour_polarization
            })
            .sum::<f64>();

        let clock_index = cell.clock_zone();

        // The clock generator emits the tunneling energy as a negative value
        // (ICHA's Hamiltonian convention); the bistable approximation needs its
        // magnitude. A zero clock fully latches the cell instead of dividing by 0.
        let tunneling_energy = self.clock_states[clock_index].abs().max(f64::MIN_POSITIVE);
        polar_math /= 2.0 * tunneling_energy;

        let new_polarization = if polar_math > 1000.0 {
            1.0
        } else if polar_math < -1000.0 {
            -1.0
        } else if f64::abs(polar_math) < 0.001 {
            polar_math
        } else {
            polar_math / f64::sqrt(1.0 + polar_math * polar_math)
        };

        let new_dot_probability = polarization_to_dot_probability_distribution(&[new_polarization]);
        let mut stable = true;
        for i in 0..new_dot_probability.len() {
            if (new_dot_probability[i] - cell.dot_probability_distribution[i]).abs()
                > self.model_settings.convergence_tolerance
            {
                stable = false;
            }
        }
        self.index_cells_write_map
            .get_mut(&cell_ind)
            .unwrap()
            .dot_probability_distribution = new_dot_probability;

        stable
    }

    fn get_states(&self, cell_ind: &QCACellIndex) -> Vec<f64> {
        if let Some(c) = self.index_cells_write_map.get(cell_ind) {
            return c.dot_probability_distribution.clone();
        }
        if let Some(c) = self.index_cells_read_map.get(cell_ind) {
            return c.dot_probability_distribution.clone();
        }
        if let Some(c) = self.index_cells_static_map.get(cell_ind) {
            return c.dot_probability_distribution.clone();
        }
        panic!("Cell not found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::run_simulation;

    const ARCHITECTURE_ID: &str = "qcadesigner";

    fn cell(x: f64, typ: CellType, clock_phase_shift: f64) -> QCACell {
        QCACell {
            position: [x, 0.0],
            rotation: 0.0,
            typ,
            clock_phase_shift,
            dot_probability_distribution: polarization_to_dot_probability_distribution(&[0.0]),
            label: None,
        }
    }

    fn simulate(cells: Vec<QCACell>) -> crate::simulation::file::QCASimulationData {
        let mut layer = QCALayer::new("Main".into(), ARCHITECTURE_ID.into(), 0.0);
        layer.cells = cells;
        let architectures = HashMap::from([(
            ARCHITECTURE_ID.to_string(),
            QCACellArchitecture::new(18.0, 5.0, 4, 9.0),
        )]);
        run_simulation(
            Box::new(BistableModel::new()),
            vec![layer],
            architectures,
            None,
        )
    }

    /// Polarization of the stored cell `stored` while clock zone 0 holds the
    /// given input combination (each combination spans 20 samples by default).
    fn held_polarization(
        data: &crate::simulation::file::QCASimulationData,
        stored: usize,
        combination: usize,
    ) -> f64 {
        data.cells_data[stored].data[combination * 20 + 7]
    }

    #[test]
    fn wire_follows_its_input() {
        let mut cells = vec![cell(0.0, CellType::Input, 0.0)];
        cells.extend((1..4).map(|i| cell(20.0 * i as f64, CellType::Normal, 0.0)));
        cells.push(cell(80.0, CellType::Output, 0.0));

        let data = simulate(cells);

        // Stored cells are the input (0) and the output (1); combination 0
        // drives the input to -1 and combination 1 to +1.
        assert!(held_polarization(&data, 1, 0) < -0.9);
        assert!(held_polarization(&data, 1, 1) > 0.9);
    }

    #[test]
    fn isolated_cell_does_not_panic() {
        let data = simulate(vec![
            cell(0.0, CellType::Input, 0.0),
            cell(1000.0, CellType::Output, 0.0),
        ]);

        assert_eq!(held_polarization(&data, 1, 0), 0.0);
    }

    #[test]
    fn clock_phase_selects_the_clock_zone() {
        let mut cells = vec![cell(0.0, CellType::Input, 0.0)];
        cells.extend((1..4).map(|i| cell(20.0 * i as f64, CellType::Normal, 0.0)));
        // A phase that is not a multiple of 90 used to index past the clock array.
        cells.push(cell(80.0, CellType::Output, 45.0));

        let data = simulate(cells);

        assert!(held_polarization(&data, 1, 1) > 0.9);
    }
}
