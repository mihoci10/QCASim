use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum CellType {
    Normal,
    Input,
    Output,
    Fixed,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct QCACellIndex {
    pub layer: usize,
    pub cell: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct QCACell {
    pub position: [f64; 2],
    pub rotation: f64,
    pub typ: CellType,
    pub clock_phase_shift: f64,
    pub dot_probability_distribution: Vec<f64>,
    pub label: Option<String>,
}

impl QCACellIndex {
    pub fn new(layer: usize, cell: usize) -> Self {
        QCACellIndex {
            layer: layer,
            cell: cell,
        }
    }
}

impl Display for QCACellIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}-{}", self.layer, self.cell).as_str())
    }
}

impl PartialEq<Self> for QCACellIndex {
    fn eq(&self, other: &Self) -> bool {
        self.layer == other.layer && self.cell == other.cell
    }
}

impl PartialOrd<Self> for QCACellIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for QCACellIndex {}

impl Ord for QCACellIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.layer.cmp(&other.layer) {
            std::cmp::Ordering::Equal => self.cell.cmp(&other.cell),
            other_ordering => other_ordering,
        }
    }
}

impl FromStr for QCACellIndex {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();

        if parts.len() != 2 {
            return Err(format!("Invalid format '{}'. Expected 'layer-cell'", s));
        }

        let layer = parts[0]
            .parse::<usize>()
            .map_err(|_| format!("Invalid layer '{}'. Must be a positive integer", parts[0]))?;

        let cell = parts[1]
            .parse::<usize>()
            .map_err(|_| format!("Invalid cell '{}'. Must be a positive integer", parts[1]))?;

        Ok(QCACellIndex { layer, cell })
    }
}

pub fn dot_probability_distribution_to_polarization(
    dot_probability_distribution: &[f64],
) -> Vec<f64> {
    let arr = dot_probability_distribution;
    let sum = arr.iter().sum::<f64>();

    if (sum - 2.0).abs() > 1e-6 {
        panic!(
            "Dot probability distribution sum should always be 2.0 and not: {:?}",
            sum
        );
    }

    match arr.len() {
        4 => vec![((arr[0] + arr[2]) - (arr[1] + arr[3])) / sum],
        8 => vec![
            ((arr[0] + arr[4]) - (arr[2] + arr[6])) / sum,
            ((arr[1] + arr[5]) - (arr[3] + arr[7])) / sum,
        ],
        _ => panic!(
            "Unsupported dot probability distribution length: {}",
            arr.len()
        ),
    }
}

pub fn polarization_to_dot_probability_distribution(polarization: &[f64]) -> Vec<f64> {
    let sum = polarization.iter().map(|x| x.abs()).sum::<f64>();
    if sum > 1.0 {
        panic!(
            "Polarization sum abs value cannot be larger than 1.0: {:?}",
            polarization
        );
    }

    match polarization.len() {
        1 => {
            let offset = (1.0 - sum) / 2.0;
            let p1 = 0.0f64.max(polarization[0]) + offset;
            let p_neg1 = 0.0f64.max(-polarization[0]) + offset;

            vec![p1, p_neg1, p1, p_neg1]
        }
        2 => {
            let offset = (1.0 - sum) / 4.0;
            let p1 = 0.0f64.max(polarization[0]) + offset;
            let p_neg1 = 0.0f64.max(-polarization[0]) + offset;
            let p2 = 0.0f64.max(polarization[1]) + offset;
            let p_neg2 = 0.0f64.max(-polarization[1]) + offset;

            vec![p1, p2, p_neg1, p_neg2, p1, p2, p_neg1, p_neg2]
        }
        _ => panic!("Unsupported polarization length: {}", polarization.len()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_probability_distribution_to_polarization_valid_cases() {
        // Tests for 4-element distribution
        let distribution = vec![0.5; 4];
        let polarization = dot_probability_distribution_to_polarization(&distribution);
        assert_eq!(polarization, vec![0.0]);

        let distribution = vec![1.0, 0.0, 1.0, 0.0];
        let polarization = dot_probability_distribution_to_polarization(&distribution);
        assert_eq!(polarization, vec![1.0]);

        let distribution = vec![0.0, 1.0, 0.0, 1.0];
        let polarization = dot_probability_distribution_to_polarization(&distribution);
        assert_eq!(polarization, vec![-1.0]);

        // Tests for 8-element distribution
        let distribution = vec![0.25; 8];
        let polarization = dot_probability_distribution_to_polarization(&distribution);
        assert_eq!(polarization, vec![0.0, 0.0]);

        let distribution = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0];
        let polarization = dot_probability_distribution_to_polarization(&distribution);
        assert_eq!(polarization, vec![1.0, 0.0]);

        let distribution = vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
        let polarization = dot_probability_distribution_to_polarization(&distribution);
        assert_eq!(polarization, vec![0.0, 1.0]);

        let distribution = vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let polarization = dot_probability_distribution_to_polarization(&distribution);
        assert_eq!(polarization, vec![-1.0, 0.0]);

        let distribution = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let polarization = dot_probability_distribution_to_polarization(&distribution);
        assert_eq!(polarization, vec![0.0, -1.0]);
    }

    #[test]
    #[should_panic(expected = "Dot probability distribution sum should always be 2.0")]
    fn test_dot_probability_distribution_to_polarization_invalid_sum() {
        // Distribution sum is not 2.0
        let distribution = vec![0.5, 0.5, 0.5, 0.4];
        dot_probability_distribution_to_polarization(&distribution);
    }

    #[test]
    #[should_panic(expected = "Dot probability distribution sum should always be 2.0")]
    fn test_dot_probability_distribution_to_polarization_invalid_length() {
        // Unsupported length
        let distribution = vec![0.5, 0.5, 0.5]; // Length = 3
        dot_probability_distribution_to_polarization(&distribution);
    }

    #[test]
    fn test_polarization_to_dot_probability_distribution_valid_cases() {
        // Tests for 1-element polarization
        let polarization = vec![0.0];
        let distribution = polarization_to_dot_probability_distribution(&polarization);
        assert_eq!(distribution, vec![0.5; 4]);

        let polarization = vec![1.0];
        let distribution = polarization_to_dot_probability_distribution(&polarization);
        assert_eq!(distribution, vec![1.0, 0.0, 1.0, 0.0]);

        let polarization = vec![-1.0];
        let distribution = polarization_to_dot_probability_distribution(&polarization);
        assert_eq!(distribution, vec![0.0, 1.0, 0.0, 1.0]);

        // Tests for 2-element polarization
        let polarization = vec![0.0, 0.0];
        let distribution = polarization_to_dot_probability_distribution(&polarization);
        assert_eq!(distribution, vec![0.25; 8]);

        let polarization = vec![1.0, 0.0];
        let distribution = polarization_to_dot_probability_distribution(&polarization);
        assert_eq!(distribution, vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]);

        let polarization = vec![0.0, 1.0];
        let distribution = polarization_to_dot_probability_distribution(&polarization);
        assert_eq!(distribution, vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0]);

        let polarization = vec![-1.0, 0.0];
        let distribution = polarization_to_dot_probability_distribution(&polarization);
        assert_eq!(distribution, vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);

        let polarization = vec![0.0, -1.0];
        let distribution = polarization_to_dot_probability_distribution(&polarization);
        assert_eq!(distribution, vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    #[should_panic(expected = "Polarization sum abs value cannot be larger than 1.0")]
    fn test_polarization_to_dot_probability_distribution_invalid_sum() {
        // Polarization sum is not 1.0
        let polarization = vec![-1.1];
        polarization_to_dot_probability_distribution(&polarization);
    }

    #[test]
    #[should_panic(expected = "Unsupported polarization length")]
    fn test_polarization_to_dot_probability_distribution_invalid_length() {
        // Unsupported length
        let polarization = vec![0.5, 0.4, 0.1]; // Length = 3
        polarization_to_dot_probability_distribution(&polarization);
    }
}
