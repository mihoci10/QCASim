use std::collections::HashMap;

use crate::objects::architecture::QCACellArchitecture;
use crate::objects::layer::QCALayer;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use serde_json::Value;

pub const DESIGN_FILE_EXTENSION: &str = "qcd";

#[derive(Serialize, Deserialize, Debug)]
pub struct SimulationModelSettings {
    pub model_settings: Value,
    pub clock_generator_settings: Value,
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug)]
pub struct SimulationSettings {
    #[serde_inline_default(None)]
    pub selected_simulation_model_id: Option<String>,

    #[serde_inline_default(HashMap::new())]
    pub simulation_model_settings: HashMap<String, SimulationModelSettings>,

    /// When true, the simulation steps through `custom_input_sequence` in
    /// order (repeats allowed) instead of exhaustively enumerating every
    /// input combination. Kept separate from `custom_input_sequence` itself
    /// so toggling back to exhaustive doesn't discard a sequence the user
    /// built.
    #[serde_inline_default(false)]
    pub use_custom_input_sequence: bool,

    /// An ordered list of input vectors to simulate when
    /// `use_custom_input_sequence` is set. Each entry is one vector - a
    /// per-input state index (0..2*polarization_n, the same encoding
    /// CellInputGenerator's exhaustive sweep uses) in input order. The same
    /// vector may appear more than once: since the simulation is inherently
    /// sequential, a cell holding state between samples (e.g. a flip-flop)
    /// can still produce a different output the second time.
    #[serde_inline_default(Vec::new())]
    pub custom_input_sequence: Vec<Vec<usize>>,
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug)]
pub struct QCADesign {
    #[serde_inline_default("unknown".to_string())]
    pub qca_core_version: String,

    #[serde_inline_default(Vec::new())]
    pub layers: Vec<QCALayer>,

    #[serde_inline_default(HashMap::<String, QCACellArchitecture>::new())]
    pub cell_architectures: HashMap<String, QCACellArchitecture>,

    #[serde_inline_default(SimulationSettings::new())]
    pub simulation_settings: SimulationSettings,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QCADesignFile {
    pub design: QCADesign,
}

impl SimulationSettings {
    pub fn new() -> Self {
        Self {
            selected_simulation_model_id: None,
            simulation_model_settings: HashMap::new(),
            use_custom_input_sequence: false,
            custom_input_sequence: Vec::new(),
        }
    }
}
