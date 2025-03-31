use std::f64::consts::PI;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum CellType{
    Normal, Input, Output, Fixed
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct QCACellIndex{
    pub layer: usize,
    pub cell: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct QCACell{
    pub position: [f64; 2],
    pub rotation: f64,
    pub typ: CellType,
    pub clock_phase_shift: f64,
    pub dot_probability_distribution: Vec<f64>,
}

impl QCACellIndex{
    pub fn new(layer: usize, cell: usize) -> Self{
        QCACellIndex{
            layer: layer,
            cell: cell
        }
    }
}