use crate::objects::architecture::QCACellArchitecture;
use crate::objects::cell::{dot_probability_distribution_to_polarization, CellType, QCACellIndex};
use crate::objects::generator::Generator;
use crate::objects::layer::QCALayer;
use crate::simulation::clock_generator::{ClockConfig, ClockGenerator};
use crate::simulation::file::{QCACellData, QCASimulationData};
use crate::simulation::input_generator::{CellInputConfig, CellInputGenerator};
use crate::simulation::model::SimulationModelTrait;
use chrono::Local;
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
pub mod full_basis;
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
    send_progress(SimulationProgress::Initializing, &progress_tx);
    let mut simulation_data = QCASimulationData::new();
    //TODO: ugly workaround
    let num_inputs: usize = layers
        .iter()
        .map(|layer| {
            layer
                .cells
                .iter()
                .filter(|c| c.typ == CellType::Input)
                .count()
        })
        .sum();
    let model_settings = sim_model.get_settings();

    for i in 0..layers.len() {
        for j in 0..layers[i].cells.len() {
            let cell = &layers[i].cells[j];
            if matches!(cell.typ, CellType::Input | CellType::Output) {
                let cell_index = QCACellIndex::new(i, j);
                simulation_data.cells_data.push(QCACellData::new(
                    cell_index.clone(),
                    model_settings.get_num_samples(),
                ));
                simulation_data
                    .metadata
                    .stored_cells
                    .push(cell_index.clone());
            }
        }
    }

    sim_model.initiate(Box::new(layers.clone()), architectures.clone());

    let clock_generator = ClockGenerator::new(
        ClockConfig {
            ampl_max: model_settings.get_clock_ampl_max(),
            ampl_min: model_settings.get_clock_ampl_min(),
        },
        model_settings.get_num_samples(),
    );
    let mut clock_iter = clock_generator.iter();
    let input_generator = CellInputGenerator::new(
        CellInputConfig {
            num_states: num_inputs,
            frequency: 2.0,
        },
        model_settings.get_num_samples(),
    );
    let mut input_iter = input_generator.iter();

    let mut simulated_samples: usize = 0;
    for i in 0..model_settings.get_num_samples() {
        if check_cancelled(cancel_rx) {
            break;
        }
        send_progress(
            SimulationProgress::Running {
                current_sample: i,
                total_samples: model_settings.get_num_samples(),
            },
            &progress_tx,
        );

        let clock_states = clock_iter.next().unwrap();
        let input_states = input_iter.next().unwrap();

        let mut stable = false;
        let mut j = 0;
        while !stable && j < model_settings.get_max_iter() {
            stable = true;

            sim_model.pre_calculate(&clock_states, &input_states);

            for l in 0..layers.len() {
                for c in 0..layers[l].cells.len() {
                    stable &= sim_model.calculate(QCACellIndex::new(l, c));
                }
            }

            j += 1;
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

    return simulation_data;
}

pub fn run_simulation(
    sim_model: Box<dyn SimulationModelTrait>,
    layers: Vec<QCALayer>,
    architectures: HashMap<String, QCACellArchitecture>,
) -> QCASimulationData {
    return run_simulation_internal(sim_model, layers, architectures, None, &mut None);
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
