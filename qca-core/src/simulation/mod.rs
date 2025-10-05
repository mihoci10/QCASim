use crate::objects::architecture::QCACellArchitecture;
use crate::objects::cell::{dot_probability_distribution_to_polarization, CellType, QCACellIndex};
use crate::objects::generator::Generator;
use crate::objects::layer::QCALayer;
use crate::simulation::clock_generator::{ClockConfig, ClockGenerator};
use crate::simulation::file::{QCACellData, QCASimulationData};
use crate::simulation::input_generator::{CellInputConfig, CellInputGenerator};
use crate::simulation::model::SimulationModelTrait;
use chrono::Local;
use log::{debug, info, trace, warn};
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use tokio::sync::oneshot;

pub mod model;
pub mod settings;

//pub mod bistable;
pub mod clock_generator;
pub mod file;
pub mod icha;
pub mod input_generator;

#[derive(Debug)]
pub enum SimulationProgress {
    Initializing,
    Running {
        current_sample: usize,
        total_samples: usize,
    },
    Deinitializng,
}
#[derive(Debug)]
pub struct SimulationCancelRequest {}

fn send_progress(progress: SimulationProgress, tx: &Option<Sender<SimulationProgress>>) {
    if let Some(tx) = &tx {
        let _ = tx.send(progress);
    }
}

fn check_cancelled(rx: &mut Option<oneshot::Receiver<SimulationCancelRequest>>) -> bool {
    match rx {
        Some(ref mut rrx) => rrx.try_recv().is_ok(),
        None => false,
    }
}

fn run_simulation_internal(
    mut sim_model: Box<dyn SimulationModelTrait>,
    layers: Vec<QCALayer>,
    architectures: HashMap<String, QCACellArchitecture>,
    progress_tx: Option<Sender<SimulationProgress>>,
    cancel_rx: &mut Option<oneshot::Receiver<SimulationCancelRequest>>,
) -> QCASimulationData {
    info!("Starting simulation");
    send_progress(SimulationProgress::Initializing, &progress_tx);
    let mut simulation_data = QCASimulationData::new();
    let architecture = architectures.get(&layers[0].cell_architecture_id).unwrap();
    let polarization_n = architecture.dot_count / 4;
    //TODO: ugly workaround
    let num_inputs = get_num_inputs(&layers);

    let model_settings = sim_model.get_model_settings();
    let clock_generator_settings = sim_model.get_clock_generator_settings();

    let input_generator = CellInputGenerator::new(CellInputConfig {
        num_inputs,
        num_samples_per_combination: clock_generator_settings.get_samples_per_input(),
        num_polarization: polarization_n as usize,
        extra_clock_periods: clock_generator_settings.get_extra_periods() * polarization_n as usize,
    });
    let mut input_iter = input_generator.iter();
    let num_samples = input_generator.num_samples();
    let clock_generator = ClockGenerator::new(ClockConfig {
        num_samples,
        num_cycles: (polarization_n as usize + 1).pow(num_inputs as u32)
            * clock_generator_settings.get_num_cycles()
            + (polarization_n as usize * clock_generator_settings.get_extra_periods()),
        amplitude_max: clock_generator_settings.get_amplitude_max(),
        amplitude_min: clock_generator_settings.get_amplitude_min(),
    });
    let mut clock_iter = clock_generator.iter();

    for i in 0..layers.len() {
        for j in 0..layers[i].cells.len() {
            let cell = &layers[i].cells[j];
            if matches!(cell.typ, CellType::Input | CellType::Output) {
                let cell_index = QCACellIndex::new(i, j);
                simulation_data
                    .cells_data
                    .push(QCACellData::new(cell_index.clone(), num_samples));
                simulation_data
                    .metadata
                    .stored_cells
                    .push(cell_index.clone());
            }
        }
    }

    debug!("Simulation model: {:?}", sim_model.get_name());
    debug!("Total iterations: {:?}", num_samples);

    sim_model.initiate(Box::new(layers.clone()), architectures.clone());

    let mut simulated_samples: usize = 0;
    for i in 0..num_samples {
        if check_cancelled(cancel_rx) {
            debug!("Simulation canceled");
            break;
        }
        debug!("Simulation iteration {:?}", simulated_samples);
        send_progress(
            SimulationProgress::Running {
                current_sample: i,
                total_samples: num_samples,
            },
            &progress_tx,
        );

        let clock_states = clock_iter.next().unwrap();
        let input_states = input_iter.next().unwrap();

        trace!("Clock states: {:?}", clock_states);
        trace!("Input states: {:?}", input_states);

        let mut stable = false;
        let mut j = 0;
        while !stable && j < model_settings.get_max_iterations() {
            stable = true;

            sim_model.pre_calculate(&clock_states, &input_states);

            for l in 0..layers.len() {
                for c in 0..layers[l].cells.len() {
                    stable &= sim_model.calculate(QCACellIndex::new(l, c));
                }
            }

            j += 1;
        }

        if !stable {
            warn!("Unstable simulation loop detected")
        }

        simulation_data
            .clock_data
            .iter_mut()
            .enumerate()
            .for_each(|(i, clock_data)| {
                clock_data.push(clock_states[i]);
            });

        simulation_data.cells_data.iter_mut().for_each(|cell_data| {
            let distribution = sim_model.get_states(&cell_data.index);
            let polarization = dot_probability_distribution_to_polarization(&distribution);
            for p in polarization {
                cell_data.data.push(p);
            }
        });
        simulated_samples += 1;
    }
    send_progress(SimulationProgress::Deinitializng, &progress_tx);
    simulation_data.metadata.duration = Local::now() - simulation_data.metadata.start_time;
    simulation_data.metadata.num_samples = simulated_samples;

    info!("Simulation complete");
    debug!("Duration: {:?}", simulation_data.metadata.duration);
    debug!(
        "Number of samples: {}",
        simulation_data.metadata.num_samples
    );

    simulation_data
}

pub fn run_simulation(
    sim_model: Box<dyn SimulationModelTrait>,
    layers: Vec<QCALayer>,
    architectures: HashMap<String, QCACellArchitecture>,
) -> QCASimulationData {
    run_simulation_internal(sim_model, layers, architectures, None, &mut None)
}

pub fn run_simulation_async(
    sim_model: Box<dyn SimulationModelTrait>,
    layers: Vec<QCALayer>,
    architectures: HashMap<String, QCACellArchitecture>,
) -> (
    JoinHandle<QCASimulationData>,
    Receiver<SimulationProgress>,
    oneshot::Sender<SimulationCancelRequest>,
) {
    let (progress_tx, progress_rx) = mpsc::channel::<SimulationProgress>();
    let (cancel_tx, cancel_rx) = oneshot::channel::<SimulationCancelRequest>();
    let thread_handler = std::thread::spawn(move || {
        return run_simulation_internal(
            sim_model,
            layers,
            architectures,
            Some(progress_tx),
            &mut Some(cancel_rx),
        );
    });

    (thread_handler, progress_rx, cancel_tx)
}

pub fn get_num_inputs(layers: &Vec<QCALayer>) -> usize {
    layers
        .iter()
        .map(|layer| {
            layer
                .cells
                .iter()
                .filter(|c| c.typ == CellType::Input)
                .count()
        })
        .sum()
}

pub fn get_num_samples(
    sim_model: &Box<dyn SimulationModelTrait>,
    layers: &Vec<QCALayer>,
    architectures: &HashMap<String, QCACellArchitecture>,
) -> usize {
    let architecture = architectures.get(&layers[0].cell_architecture_id).unwrap();
    let polarization_n = architecture.dot_count / 4;
    let clock_generator_settings = sim_model.get_clock_generator_settings();

    let num_inputs = get_num_inputs(layers);

    let input_generator = CellInputGenerator::new(CellInputConfig {
        num_inputs,
        num_samples_per_combination: clock_generator_settings.get_samples_per_input(),
        num_polarization: polarization_n as usize,
        extra_clock_periods: clock_generator_settings.get_extra_periods(),
    });

    input_generator.num_samples()
}
