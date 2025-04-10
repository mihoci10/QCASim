use std::collections::HashMap;
use std::fs::File;
use chrono::{DateTime, Duration, Local, TimeDelta};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use serde_json::Value;
use tar::{Archive, Builder, Header};
use crate::design::file::QCADesign;
use crate::objects::layer::QCALayer;
use crate::objects::architecture::QCACellArchitecture;

pub const SIMULATION_FILE_EXTENSION: &str = "qcs";

#[derive(Serialize, Deserialize, Debug)]
#[serde_inline_default]
pub struct QCASimulationMetadata{
    #[serde_inline_default("unknown".to_string())]
    pub qca_core_version: String,

    pub start_time: DateTime<Local>,
    pub duration: TimeDelta,
}

pub struct QCASimulationData {
    pub cells_data: Vec<>
}

pub fn write_to_file(filename: &str, design: &QCADesign) -> Result<(), String> {
    let file = File::create_new(filename)
        .map_err(|error| error.to_string())?;

    let mut builder = Builder::new(file);

    {
        let design_raw = serde_json::to_vec_pretty(design)
            .map_err(|error| error.to_string())?;

        let mut header = Header::new_gnu();
        header.set_size(design_raw.len() as u64);

        builder.append_data(&mut header, "DESIGN", design_raw.as_slice())
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}