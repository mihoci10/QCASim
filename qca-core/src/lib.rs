pub mod sim;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use self::sim::{bistable::BistableModel, CellType, QCACell, Simulator};

    use super::*;
    
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn bistable_01() {
        let model = Box::new(BistableModel::new());
        let cells = (0..10).map(|i| {
            QCACell{
                pos_x: i as f64 * 20.0,
                pos_y: 0.0,
                clock: sim::CellClock::First,
                typ: if i == 0 {CellType::Fixed} else {CellType::Normal},
                polarization: if i == 0 {1.0} else {0.0}
            }
        }).collect();
        
        let mut simulator = Simulator::new(model, cells);
        simulator.run();
    }
}
