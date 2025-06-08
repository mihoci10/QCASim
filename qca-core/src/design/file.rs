use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use serde_json::Value;
use crate::objects::layer::QCALayer;
use crate::objects::architecture::QCACellArchitecture;

pub const DESIGN_FILE_EXTENSION: &str = "qcd";

#[derive(Serialize, Deserialize, Debug)]
#[serde_inline_default]
pub struct QCADesign {
    #[serde_inline_default("unknown".to_string())]
    pub qca_core_version: String,

    #[serde_inline_default(Vec::new())]
    pub layers: Vec<QCALayer>,

    #[serde_inline_default(HashMap::new())]
    pub simulation_model_settings: HashMap<String, Value>,

    #[serde_inline_default(None)]
    pub selected_simulation_model_id: Option<String>,

    #[serde_inline_default(HashMap::<String, QCACellArchitecture>::new())]
    pub cell_architectures: HashMap<String, QCACellArchitecture>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QCADesignFile{
    pub design: QCADesign,
}