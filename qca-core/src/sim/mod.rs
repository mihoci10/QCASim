#[derive(Clone, Copy)]
pub enum CellType{
    Normal, Input, Output, Fixed
}
#[derive(Clone, Copy)]
pub enum CellClock{
    First, Second, Third, Fourth 
}

#[derive(Clone, Copy)]
pub struct QCACell{
    pub pos_x: f64,
    pub pos_y: f64,
    pub typ: CellType,
    pub clock: CellClock,
    pub polarization: f64,
}

pub trait SimulationModelTrait{
    fn initiate(&mut self, cells: Box<Vec<QCACell>>);
    fn pre_calculate(&mut self, clock_states: [f64; 4]);
    fn calculate(&mut self, cell_ind: usize) -> bool;
}

pub mod bistable;

pub struct Simulator{
    sim_model: Box<dyn SimulationModelTrait>,
    cells: Vec<QCACell>,
}

impl Simulator{

    pub fn new(sim_model: Box<dyn SimulationModelTrait>, cells: Vec<QCACell>) -> Simulator{
        Simulator{sim_model: sim_model, cells: cells}
    }

    pub fn run(&mut self){
        self.sim_model.initiate(Box::new(self.cells.clone()));
        for _ in 0..10 {
            self.sim_model.pre_calculate([0.0, 0.0, 0.0, 0.0]);

            let mut stable = false;
			while !stable {
				stable = true;

				for i in 0..self.cells.len() { 
					stable &= self.sim_model.calculate(i)
                }
			}
        }
    }

}