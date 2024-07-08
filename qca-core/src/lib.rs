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

        cells[0].dot_probability_distribution = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0];

        run_simulation(&mut sim_model, cells, architecture, None);
    }
}
