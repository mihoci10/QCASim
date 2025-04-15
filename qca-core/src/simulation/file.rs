use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use chrono::{DateTime, Duration, Local, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use serde_json::Value;
use tar::{Archive, Builder, Header, HeaderMode};
use crate::design::file::QCADesign;
use crate::objects::layer::QCALayer;
use crate::objects::architecture::QCACellArchitecture;
use crate::{get_qca_core_version, QCA_CORE_VERSION};
use crate::objects::cell::QCACellIndex;

pub const SIMULATION_FILE_EXTENSION: &str = "qcs";

const DESIGN_ENTRY_NAME: &str = "DESIGN.json";
const SIM_METADATA_ENTRY_NAME: &str = "METADATA.json";
const SIM_DATA_ENTRY_NAME: &str = "DATA.bin";

#[derive(Serialize, Deserialize, Debug)]
#[serde_inline_default]
pub struct QCASimulationMetadata{
    #[serde_inline_default("unknown".to_string())]
    pub qca_core_version: String,

    pub start_time: DateTime<Local>,
    pub duration: TimeDelta,

    #[serde_inline_default(Vec::new())]
    pub stored_cells: Vec<QCACellIndex>
}

pub struct QCACellData{
    pub index: QCACellIndex,
    pub data: Vec<f64>
}

pub struct QCASimulationData {
    pub metadata: QCASimulationMetadata,
    pub clock_data: [Vec<f64>; 4],
    pub cells_data: Vec<QCACellData>
}


impl QCACellData{
    pub fn new(index: QCACellIndex, data_capacity: usize) -> QCACellData{
        QCACellData{
            index,
            data: Vec::with_capacity(data_capacity)
        }
    }
}

impl QCASimulationMetadata{
    pub fn new() -> QCASimulationMetadata{
        QCASimulationMetadata{
            qca_core_version: get_qca_core_version(),
            start_time: Local::now(),
            duration: TimeDelta::zero(),
            stored_cells: Vec::new()
        }
    }
}

impl QCASimulationData{
    pub fn new() -> QCASimulationData{
        QCASimulationData{
            clock_data: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            cells_data: Vec::new(),
            metadata: QCASimulationMetadata::new()
        }
    }
}

fn get_sim_data_raw(sim_data: &QCASimulationData) -> Vec<u8>{
    let capacity = sim_data.clock_data.iter().map(|clock| {
            clock.len() * size_of::<f64>()
        }).sum::<usize>() +
        sim_data.cells_data.iter().map(|cell_data| {
            cell_data.data.len() * size_of::<f64>()
    }).sum::<usize>();

    let mut output = Vec::with_capacity(capacity);

    for clock_data in &sim_data.clock_data {
        for value in clock_data {
            let byte_repr = value.to_ne_bytes();
            output.extend_from_slice(&byte_repr);
        }
    }

    for cell_data in &sim_data.cells_data{
        for value in &cell_data.data{
            let byte_repr = value.to_ne_bytes();
            output.extend_from_slice(&byte_repr);
        }
    }
    output
}

fn write_slice(builder: &mut Builder<File>, entry_name: &str, data: Vec<u8>) -> Result<(), String>{
    let mut header = Header::new_gnu();
    header.set_size(data.len() as u64);
    header.set_mode(0o644);
    header.set_mtime(Utc::now().timestamp() as u64);
    header.set_cksum();

    builder.append_data(&mut header, entry_name, data.as_slice())
        .map_err(|error| error.to_string())?;

    Ok(())
}

pub fn write_to_file(filename: &str, design: &QCADesign, simulation_data: &QCASimulationData) -> Result<(), String> {
    let file = File::create(filename)
        .map_err(|error| error.to_string())?;

    let mut builder = Builder::new(file);
    builder.mode(HeaderMode::Deterministic);

    let design_raw = serde_json::to_vec_pretty(design)
        .map_err(|error| error.to_string())?;
    write_slice(&mut builder, DESIGN_ENTRY_NAME, design_raw)?;

    let sim_metadata_raw = serde_json::to_vec_pretty(&simulation_data.metadata)
        .map_err(|error| error.to_string())?;
    write_slice(&mut builder, SIM_METADATA_ENTRY_NAME, sim_metadata_raw)?;

    let sim_data_raw = get_sim_data_raw(simulation_data);
    write_slice(&mut builder, SIM_DATA_ENTRY_NAME, sim_data_raw)?;

    builder.into_inner()
        .map_err(|error| error.to_string())?;

    Ok(())
}

pub fn read_from_file(filename: &str) -> Result<(QCADesign, QCASimulationData), String>{
    let file = File::open(filename)
        .map_err(|error| error.to_string())?;

    let mut archive = Archive::new(file);
    let mut entries = archive.entries()
        .map_err(|error| error.to_string())?;

    let mut design: Option<QCADesign> = None;
    let mut metadata: Option<QCASimulationMetadata> = None;
    let mut sim_data: Option<Vec<u8>> = None;

    for entry in entries{
        let mut entry = entry
            .map_err(|error| error.to_string())?;

        match entry.path().unwrap().to_string_lossy().as_ref() {
            DESIGN_ENTRY_NAME => {
                let mut contents = String::new();
                let _ = entry.read_to_string(&mut contents)
                    .map_err(|error| error.to_string())?;
                design = Some(serde_json::from_str::<QCADesign>(contents.as_str())
                        .map_err(|error| error.to_string())?
                );
            },
            SIM_METADATA_ENTRY_NAME => {
                let mut contents = String::new();
                let _ = entry.read_to_string(&mut contents)
                    .map_err(|error| error.to_string())?;
                metadata = Some(serde_json::from_str::<QCASimulationMetadata>(contents.as_str())
                    .map_err(|error| error.to_string())?
                );},
            SIM_DATA_ENTRY_NAME => {
                let mut contents: Vec<u8> = Vec::new();
                let _ = entry.read_to_end(&mut contents)
                    .map_err(|error| error.to_string())?;
                sim_data = Some(contents);
            },
            _ => {}
        }
    }

    if let Some(design) = design{
        if let Some(metadata) = metadata{
            if let Some(sim_data) = sim_data{
                let mut simulation = QCASimulationData::new();
                simulation.metadata = metadata;

                return Ok((design, simulation));
            }
            else{ return Err(format!("Missing {} entry in file!", SIM_DATA_ENTRY_NAME)) }
        }
        else{ return Err(format!("Missing {} entry in file!", SIM_METADATA_ENTRY_NAME)) }
    }
    else{ return Err(format!("Missing {} entry in file!", DESIGN_ENTRY_NAME)) }
}