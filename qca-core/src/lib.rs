use semver::{BuildMetadata, Prerelease, Version};

pub mod sim;
pub mod datafile;

pub const QCA_CORE_VERSION: Version = 
    Version{major: 1, minor: 0, patch: 0, pre: Prerelease::EMPTY, build: BuildMetadata::EMPTY};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    use std::{f64::consts::PI, fs::File, io::Write};

    use sim::{full_basis::{FullBasisModel, QCACellInternal}, run_simulation, QCACellArchitecture, SimulationModelTrait};

    use self::sim::{CellType, QCACell};

    use super::*;
    
    #[test]
    fn full_basis_cell() {
        let cell = QCACell{
            clock_phase_shift: 0.0, 
            dot_probability_distribution: vec![0.0; 8], 
            position: [0.0; 3], 
            rotation: 0.0, 
            typ: CellType::Normal
        };
        let architecture = Box::new(QCACellArchitecture::new(60.0, 10.0, 8, 20.0));

        let cell_internal = QCACellInternal::new(Box::new(cell), &architecture, 10.0);
    }

    #[test]
    fn full_basis_line() {
        let mut sim_model: Box<dyn SimulationModelTrait> = Box::new(FullBasisModel::new());

        let mut cells = (0..3).map(|i| {
            QCACell{
                clock_phase_shift: 0.0, 
                dot_probability_distribution: vec![0.0; 8], 
                position: [60.0 * i as f64, 0.0, 0.0], 
                rotation: 0.0, 
                typ: ( if i == 0 { CellType::Fixed } else { CellType::Normal })
            }
        }).collect::<Vec<QCACell>>();
        let architecture = QCACellArchitecture::new(60.0, 10.0, 8, 20.0);

        cells[0].dot_probability_distribution = vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
        cells.last_mut().unwrap().typ = CellType::Output;

        let file = Box::new(File::create("full_basis_line.bin").unwrap()) as Box<dyn Write>;

        run_simulation(&mut sim_model, cells, architecture, Some(file));
    }

    #[test]
    fn full_basis_negation() {
        let mut sim_model: Box<dyn SimulationModelTrait> = Box::new(FullBasisModel::new());

        let mut cells = (0..2).map(|i| {
            QCACell{
                clock_phase_shift: 0.0, 
                dot_probability_distribution: vec![0.0; 8], 
                position: [60.0 * i as f64, 60.0 * i as f64, 0.0], 
                rotation: 0.0, 
                typ: ( if i == 0 { CellType::Fixed } else { CellType::Normal })
            }
        }).collect::<Vec<QCACell>>();
        let architecture = QCACellArchitecture::new(60.0, 10.0, 8, 20.0);

        cells[0].dot_probability_distribution = vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
        cells.last_mut().unwrap().typ = CellType::Output;

        let file = Box::new(File::create("full_basis_negation.bin").unwrap()) as Box<dyn Write>;

        run_simulation(&mut sim_model, cells, architecture, Some(file));
    }

    #[test]
    fn full_basis_line_clocked() {
        let mut sim_model: Box<dyn SimulationModelTrait> = Box::new(FullBasisModel::new());

        let mut cells = (0..5).map(|i| {
            QCACell{
                clock_phase_shift: 90.0 * i as f64, 
                dot_probability_distribution: vec![0.0; 8], 
                position: [60.0 * i as f64, 0.0, 0.0], 
                rotation: 0.0, 
                typ: ( if i == 0 { CellType::Fixed } else { CellType::Output })
            }
        }).collect::<Vec<QCACell>>();
        let r: f64 = (20.0 * 2.0 / 3.0)/(2.0 * (PI / 8.0).sin()) as f64;
        let architecture = QCACellArchitecture::new(60.0, 10.0, 8, r);

        cells[0].dot_probability_distribution = vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0];

        let file = Box::new(File::create("full_basis_line_clocked.bin").unwrap()) as Box<dyn Write>;

        run_simulation(&mut sim_model, cells, architecture, Some(file));
    }
}
