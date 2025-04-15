use semver::{BuildMetadata, Prerelease, Version};

pub mod objects;
pub mod simulation;
pub mod design;

pub const QCA_CORE_VERSION: Version =
    Version{major: 1, minor: 0, patch: 0, pre: Prerelease::EMPTY, build: BuildMetadata::EMPTY};
pub fn get_qca_core_version() -> String{
    QCA_CORE_VERSION.to_string()
}

#[cfg(test)]
mod tests {

    use std::{f64::consts::PI, fs::File, io::Write};
    use std::collections::HashMap;
    use simulation::{full_basis::{FullBasisModel, QCACellInternal}, run_simulation};
    use crate::objects::architecture::QCACellArchitecture;
    use crate::objects::cell::{CellType, QCACell};
    use crate::objects::layer::QCALayer;
    use crate::simulation::model::SimulationModelTrait;
    use super::*;
    
    #[test]
    fn full_basis_cell() {
        let cell = QCACell{
            clock_phase_shift: 0.0, 
            dot_probability_distribution: vec![0.0; 8], 
            position: [0.0; 2], 
            rotation: 0.0, 
            typ: CellType::Normal
        };
        let cell_architecture = QCACellArchitecture::new(60.0, 10.0, 8, 20.0);
        let cell_architecture_id: String = "cell_arch_1".to_string();
        let layer = QCALayer::new("Layer 1".to_string(), cell_architecture_id, 0.0);

        let cell_internal = QCACellInternal::new(Box::new(cell), &layer, &cell_architecture, 10.0);
    }

    #[test]
    fn full_basis_line() {
        let mut sim_model: Box<dyn SimulationModelTrait> = Box::new(FullBasisModel::new());

        let cell_architecture = QCACellArchitecture::new(60.0, 10.0, 8, 20.0);
        let cell_architecture_id: String = "cell_arch_1".to_string();
        let mut layer = QCALayer::new("Layer 1".to_string(), cell_architecture_id.clone(), 0.0);
        let mut cell_architectures_map = HashMap::<String, QCACellArchitecture>::new();
        cell_architectures_map.insert(cell_architecture_id, cell_architecture);

        layer.cells = (0..3).map(|i| {
            QCACell{
                clock_phase_shift: 0.0, 
                dot_probability_distribution: vec![0.25; 8],
                position: [60.0 * i as f64, 0.0], 
                rotation: 0.0, 
                typ: ( if i == 0 { CellType::Fixed } else { CellType::Normal })
            }
        }).collect::<Vec<QCACell>>();
        

        layer.cells[0].dot_probability_distribution = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0];
        layer.cells.last_mut().unwrap().typ = CellType::Output;

        let file = Box::new(File::create("full_basis_line.qcs").unwrap()) as Box<dyn Write>;

        run_simulation(sim_model, vec![layer], cell_architectures_map);
    }

    #[test]
    fn full_basis_negation() {
        let mut sim_model: Box<dyn SimulationModelTrait> = Box::new(FullBasisModel::new());

        let cell_architecture = QCACellArchitecture::new(60.0, 10.0, 8, 20.0);
        let cell_architecture_id: String = "cell_arch_1".to_string();
        let mut layer = QCALayer::new("Layer 1".to_string(), cell_architecture_id.clone(), 0.0);
        let mut cell_architectures_map = HashMap::<String, QCACellArchitecture>::new();
        cell_architectures_map.insert(cell_architecture_id, cell_architecture);

        layer.cells = (0..2).map(|i| {
            QCACell{
                clock_phase_shift: 0.0, 
                dot_probability_distribution: vec![0.0; 8], 
                position: [60.0 * i as f64, 60.0 * i as f64], 
                rotation: 0.0, 
                typ: ( if i == 0 { CellType::Fixed } else { CellType::Normal })
            }
        }).collect::<Vec<QCACell>>();
        let architecture = QCACellArchitecture::new(60.0, 10.0, 8, 20.0);

        layer.cells[0].dot_probability_distribution = vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
        layer.cells.last_mut().unwrap().typ = CellType::Output;

        let file = Box::new(File::create("full_basis_negation.qcs").unwrap()) as Box<dyn Write>;

        run_simulation(sim_model, vec![layer], cell_architectures_map);
    }

    #[test]
    fn full_basis_line_clocked() {
        let mut sim_model: Box<dyn SimulationModelTrait> = Box::new(FullBasisModel::new());
        let r: f64 = (20.0 * 2.0 / 3.0)/(2.0 * (PI / 8.0).sin()) as f64;

        let cell_architecture = QCACellArchitecture::new(60.0, 10.0, 8, r);
        let cell_architecture_id: String = "cell_arch_1".to_string();
        let mut layer = QCALayer::new("Layer 1".to_string(), cell_architecture_id.clone(), 0.0);
        let mut cell_architectures_map = HashMap::<String, QCACellArchitecture>::new();
        cell_architectures_map.insert(cell_architecture_id, cell_architecture);

        layer.cells = (0..5).map(|i| {
            QCACell{
                clock_phase_shift: 90.0 * i as f64, 
                dot_probability_distribution: vec![0.0; 8], 
                position: [60.0 * i as f64, 0.0], 
                rotation: 0.0, 
                typ: ( if i == 0 { CellType::Fixed } else { CellType::Output })
            }
        }).collect::<Vec<QCACell>>();

        layer.cells[0].dot_probability_distribution = vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0];

        let file = Box::new(File::create("full_basis_line_clocked.qcs").unwrap()) as Box<dyn Write>;

        run_simulation(sim_model, vec![layer], cell_architectures_map);
    }
}
