use serde::{Deserialize, Serialize};
use crate::objects::cell::QCACell;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QCALayer{
    pub name: String,
    pub cell_architecture_id: String,
    pub cells: Vec<QCACell>,
    pub z_position: f64,
}

impl QCALayer{
    pub fn new(name: String, cell_architecture_id: String, z_position: f64) -> Self {
        QCALayer {
            name,
            cell_architecture_id,
            cells: Vec::new(),
            z_position,
        }
    }
}