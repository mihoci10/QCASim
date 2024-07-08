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

    use sim::{full_basis::QCACellInternal, QCACellArchitecture};

    use self::sim::{CellType, QCACell};

    use super::*;
    
    #[test]
    fn it_works() {
        let cell = QCACell{
            clock_phase_shift: 0.0, 
            dot_probability_distribution: vec![0.0; 8], 
            position: [0.0; 3], 
            rotation: 0.0, 
            typ: CellType::Normal
        };
        let architecture = QCACellArchitecture::new(18.0, 5.0, 8, 18.0 / 2.0);

        let cell_internal = QCACellInternal::new(Box::new(cell), Box::new(architecture), -0.03);
    }
}
