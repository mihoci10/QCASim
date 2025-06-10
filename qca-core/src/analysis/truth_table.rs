use std::collections::HashMap;
use std::fmt;
use crate::design::file::QCADesign;
use crate::objects::cell::{QCACell, QCACellIndex};
use crate::simulation::file::QCASimulationData;

pub struct TruthTable{
    pub entries: Vec<(String, Vec<Option<u8>>)>
}

impl fmt::Display for TruthTable{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_len = self.entries.iter().map(|(_, x)| x.len()).max().unwrap();
        for entry in &self.entries{
            f.write_str(entry.0.as_str())?;
            f.write_str("\t")?;
        }
        for i in 0..max_len{
            f.write_str("\n")?;
            for entry in &self.entries{
                let mut value = None;
                if let Some(v) = entry.1.get(i){
                    value = *v;
                }
                match value {
                    None => {f.write_str("NaN")?;}
                    Some(value) => {f.write_str(value.to_string().as_str())?;}
                }
                f.write_str("\t")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct ClockRegion{
    start: usize,
    end: usize,
}

fn generate_clock_regions(clock_data: &[Vec<f64>; 4], clock_threshold: f64) -> [Vec<ClockRegion>; 4]{
    std::array::from_fn(|i| {
        let clock_data = &clock_data[i];

        let clock_high = clock_data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let clock_low = clock_data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let clock_high_threshold = clock_high - (clock_high - clock_low) * clock_threshold;

        let mut clock_regions = Vec::<ClockRegion>::new();
        let mut current_clock_region: Option<ClockRegion> = None;
        for (i, value) in clock_data.iter().enumerate(){
            if *value > clock_high_threshold{
                match current_clock_region {
                    None => current_clock_region = Some(ClockRegion{start: i, end: i}),
                    Some(ref mut region) => region.end = i
                }
            } else if let Some(region) = current_clock_region {
                clock_regions.push(region);
                current_clock_region = None;
            }
        }
        if let Some(region) = current_clock_region {
            clock_regions.push(region);
            current_clock_region = None;
        }

        clock_regions
    })
}

fn clean_clock_regions(clock_regions: &mut [Vec<ClockRegion>; 4]) {
    for i in (0..4).rev() {
        for j in (0..i).rev() {
            let (left, right) = clock_regions.split_at_mut(i);
            let current_clock = &mut right[0];
            let other_clock = &left[j];

            if !current_clock.is_empty() && !other_clock.is_empty()
                && current_clock[0].start < other_clock[0].start {
                current_clock.remove(0);
            }
        }
    }
}

fn generate_logical_value(cell_data: &[f64], clock_region: &ClockRegion, polarization_count: u8, logical_threshold: f64) -> Option<u8>{
    let polarization_high = 1f64 - (2f64 * logical_threshold);
    let polarization_low = - 1f64 + (2f64 * logical_threshold);

    let data_slice = &cell_data[
        clock_region.start * polarization_count as usize..clock_region.end * polarization_count as usize];

    (0..data_slice.len()).step_by(polarization_count as usize).into_iter().map(|i| {
        match polarization_count {
            1 => {
                let value = data_slice[i];
                if value > polarization_high { Some(1) }
                else if value < polarization_low { Some(0) }
                else { None }
            }
            2 => {
                let value_a = data_slice[i];
                let value_b = data_slice[i + 1];
                if value_a > polarization_high || value_a < polarization_low { Some(1) }
                else if value_b > polarization_high { Some(2) }
                else if value_b < polarization_low { Some(0) }
                else { None }
            }
            _ => panic!("Invalid polarization count")
        }
    }).fold(HashMap::new(), |mut acc, item| {
        *acc.entry(item).or_insert(0) += 1;
        acc
    }).into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(val, _)| val).unwrap()
}

pub fn generate_truth_table(design: &QCADesign, simulation: &QCASimulationData, cells: &Vec<QCACellIndex>, clock_threshold: f64, logical_threshold: f64) -> TruthTable{
    let mut clock_regions = generate_clock_regions(&simulation.clock_data, clock_threshold);
    clean_clock_regions(&mut clock_regions);

    let cells_logic_data = cells.iter().map(|cell| {
        let cell_data = simulation.cells_data.iter().find(|cell_data| cell_data.index.eq(cell)).unwrap();
        let clock_index = (design.layers[cell.layer].cells[cell.cell].clock_phase_shift % 90f64).round() as usize;
        let polarization_count = &design.cell_architectures[&design.layers[cell.layer].cell_architecture_id].dot_count / 4;
        let logical_data = clock_regions[clock_index].iter().map(|clock_region| {
            generate_logical_value(cell_data.data.as_slice(), clock_region, polarization_count, logical_threshold)
        }).collect::<Vec<_>>();
        logical_data
    }).collect::<Vec<_>>();

    TruthTable{
        entries: (0..cells.len()).map(|i| {
            (cells[i].to_string(), cells_logic_data[i].clone())
        }).collect()
    }
}