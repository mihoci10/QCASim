use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

use crate::sim::QCACell;

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug)]
pub struct QCADesign {
    #[serde_inline_default("unknown".to_string())]
    qca_core_version: String,

    #[serde_inline_default(Vec::new())]
    cells: Vec<QCACell>,

    #[serde_inline_default(HashMap::new())]
    simulation_model_settings: HashMap<String, String>,

    #[serde_inline_default(None)]
    selected_simulation_model_id: Option<String>,
}