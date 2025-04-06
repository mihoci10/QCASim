use std::{f64::consts::{self, PI}, io::Write, ops::Rem};
use std::collections::HashMap;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use serde::{Serialize, Deserialize};
use tokio::sync::oneshot;
use crate::sim::architecture::QCACellArchitecture;
use crate::sim::cell::{dot_probability_distribution_to_polarization, CellType, QCACellIndex};
use crate::sim::clock::get_clock_values;
use crate::sim::input_generator::{generate_cell_input, generate_cell_input_sample};
use crate::sim::layer::QCALayer;
use crate::sim::model::SimulationModelTrait;

pub mod settings;
pub mod cell;
pub mod layer;
pub mod architecture;
pub mod model;

//pub mod bistable;
pub mod full_basis;
pub mod clock;
pub mod input_generator;

pub enum SimulationProgress{
    Initializing,
    Running { current_sample: usize, total_samples: usize},
    Deinitializng
}
pub enum SimulationCancelRequest{}

fn send_progress(progress: SimulationProgress, tx: &Option<Sender<SimulationProgress>>){
    if let Some(tx) = &tx{
        let _ = tx.send(progress);
    }
}

fn check_cancelled(rx: &mut Option<oneshot::Receiver<SimulationCancelRequest>>) -> bool {
    match rx {
        Some(ref mut rrx) => rrx.try_recv().is_ok(),
        None => false
    }
}

fn run_simulation_internal(
    mut sim_model: Box<dyn SimulationModelTrait>,
    layers: Vec<QCALayer>,
    architectures: HashMap<String, QCACellArchitecture>,
    mut stream: Option<Box<dyn Write + Send>>,
    progress_tx: Option<Sender<SimulationProgress>>,
    cancel_rx: &mut Option<oneshot::Receiver<SimulationCancelRequest>>
)
{
    send_progress(SimulationProgress::Initializing, &progress_tx);
    let cell_architecture: QCACellArchitecture = architectures.get(&layers[0].cell_architecture_id).unwrap().clone();
    //TODO: ugly workaround
    let n: usize = cell_architecture.dot_count as usize;
    let num_inputs: usize = layers.iter().map(|layer| layer.cells.iter().filter(|c| c.typ == CellType::Input).count()).sum();
    let num_outputs: usize = layers.iter().map(|layer| layer.cells.iter().filter(|c| c.typ == CellType::Output).count()).sum();
    let model_settings = sim_model.get_settings();

    sim_model.initiate( Box::new(layers.clone()), architectures.clone());
    
    if let Some(s) = &mut stream {
        let _ = s.write(&(num_inputs + num_outputs).to_le_bytes());
        let _ = s.write(&(n / 4).to_le_bytes());
    }

    for i in 0..model_settings.get_num_samples() {
        if check_cancelled(cancel_rx){
            break;
        }
        send_progress(
            SimulationProgress::Running {
                current_sample: i,
                total_samples: model_settings.get_num_samples()
            }, &progress_tx);

        let clock_states = get_clock_values(
            model_settings.get_num_samples(),
            i * 4 * 2,
            num_inputs,
            model_settings.get_clock_ampl_min(),
            model_settings.get_clock_ampl_max(),
            model_settings.get_clock_ampl_fac()
        );

        let input_states = (0..num_inputs).map(|j| {
            generate_cell_input_sample(n/4, i, model_settings.get_num_samples(), f64::powi(2.0, j as i32))
        }).flatten().collect::<Vec<f64>>();

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
                            let polarization = dot_probability_distribution_to_polarization(&distribution);
                            for p in 0..polarization.len() {
                                let _ = s.write(&polarization[p].to_le_bytes());
                            }
                        }
                    }
                }
            }
        }
    };
    send_progress(SimulationProgress::Deinitializng, &progress_tx);
}

pub fn run_simulation(
    sim_model: Box<dyn SimulationModelTrait>,
    layers: Vec<QCALayer>,
    architectures: HashMap<String, QCACellArchitecture>,
    stream: Option<Box<dyn Write + Send>>){
    run_simulation_internal(sim_model, layers, architectures, stream, None, &mut None);
}

pub fn run_simulation_async(
    sim_model: Box<dyn SimulationModelTrait>,
    layers: Vec<QCALayer>,
    architectures: HashMap<String, QCACellArchitecture>,
    stream: Option<Box<dyn Write + Send>>
) -> (JoinHandle<()>, Receiver<SimulationProgress>, oneshot::Sender<SimulationCancelRequest>) {

    let (progress_tx, progress_rx) = mpsc::channel::<SimulationProgress>();
    let (cancel_tx, mut cancel_rx) = oneshot::channel::<SimulationCancelRequest>();
    let thread_handler = std::thread::spawn(move || {
        run_simulation_internal(sim_model, layers, architectures, stream, Some(progress_tx), &mut Some(cancel_rx));
    });

    (thread_handler, progress_rx, cancel_tx)
}