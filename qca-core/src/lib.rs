pub mod sim;
pub mod datafile;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use self::sim::{bistable::BistableModel, run_simulation, settings::*, CellType, QCACell, SimulationModelTrait};

    use super::*;
    
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn bistable_01() {
        let mut model: Box<dyn SimulationModelTrait> = Box::new(BistableModel::new());
        let cells = (0..10).map(|i| {
            QCACell{
                pos_x: i as f64 * 20.0,
                pos_y: 0.0,
                z_index: 0,
                clock_phase_shift: 0.0,
                typ: if i == 0 {CellType::Fixed} else {CellType::Normal},
                polarization: if i == 0 {1.0} else {0.0}
            }
        }).collect();
        
        run_simulation(&mut model, cells);
    }

    #[test]
    fn bistable_02() {
        let mut model: Box<dyn SimulationModelTrait> = Box::new(BistableModel::new());
        let cells = (0..2).map(|i| {
            QCACell{
                pos_x: i as f64 * 20.0,
                pos_y: i as f64 * 20.0,
                z_index: 0,
                clock_phase_shift: 0.0,
                typ: if i == 0 {CellType::Fixed} else {CellType::Normal},
                polarization: if i == 0 {1.0} else {0.0}
            }
        }).collect();
        
        run_simulation(&mut model, cells);
    }

    #[test]
    fn serialize_01() {
        let cells: Vec<QCACell> = (0..10).map(|i| {
            QCACell{
                pos_x: i as f64 * 20.0,
                pos_y: 0.0,
                z_index: 0,
                clock_phase_shift: 0.0,
                typ: if i == 0 {CellType::Fixed} else {CellType::Normal},
                polarization: if i == 0 {1.0} else {0.0}
            }
        }).collect();

        println!("{}", serde_json::to_string(&cells).unwrap());
    }

    #[test]
    fn serialize_02() {
        let settings: OptionsList = vec![
            OptionsEntry::Header { label: "Cell structure".to_string() },
            OptionsEntry::Break,
            OptionsEntry::Input { 
                unique_id: "cell_size".to_string(), 
                name: "Size".to_string(), 
                description: "Side dimension of the cell in nm".to_string(), 
                descriptor: InputDescriptor::NumberInput {min: Some(1.0), max: Some(2.0), unit: Some("unit".into()), whole_num: true} }
        ]; 

        println!("{}", serde_json::to_string(&settings).unwrap());
    }
}
