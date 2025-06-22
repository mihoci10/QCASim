use crate::objects::generator::{Generator, GeneratorConfig};
use serde::{Deserialize, Serialize};

/// Configuration for cell input generator
#[derive(Clone, Serialize, Deserialize)]
pub struct CellInputConfig {
    /// Number of cell inputs
    pub num_inputs: usize,
    /// Number of polarization states
    pub num_polarization: usize,
    /// Extend last cycle to the end of the simulation
    pub extend_last_cycle: bool,
}

impl GeneratorConfig for CellInputConfig {}

/// Generator for cell input values that produces vectors of values
/// Output dimension: num_inputs * num_polarization
/// Total combinations: (num_polarization * 2)^num_inputs
pub struct CellInputGenerator {
    config: CellInputConfig,
    num_samples: usize,
    total_combinations: usize,
}

impl Generator for CellInputGenerator {
    type Config = CellInputConfig;
    type Output = Vec<f64>;

    fn new(config: Self::Config, num_samples: usize) -> Self {
        let total_combinations = (config.num_polarization * 2).pow(config.num_inputs as u32);
        Self {
            config,
            num_samples,
            total_combinations,
        }
    }

    fn generate(&self, sample: usize) -> Self::Output {
        // Calculate which combination we're in
        let samples_per_combination = self.num_samples / self.total_combinations;
        let combination_index = sample / samples_per_combination;

        // Progress within the current combination (0.0 to 1.0)
        let progress = (sample % samples_per_combination) as f64 / samples_per_combination as f64;

        // Generate the combination pattern
        let combination = self.get_combination(combination_index);

        // Generate output vector with dimension num_inputs * num_polarization
        let mut output = Vec::with_capacity(self.config.num_inputs * self.config.num_polarization);

        for input_idx in 0..self.config.num_inputs {
            for pol_idx in 0..self.config.num_polarization {
                let value = self.generate_signal_value(input_idx, pol_idx, &combination, progress);
                output.push(value);
            }
        }

        output
    }

    fn num_samples(&self) -> usize {
        self.num_samples
    }
}

impl CellInputGenerator {
    /// Get the combination pattern for a given combination index
    /// Each input can have values from 0 to (num_polarization * 2 - 1)
    fn get_combination(&self, combination_index: usize) -> Vec<usize> {
        let base = self.config.num_polarization * 2;
        let mut combination = Vec::with_capacity(self.config.num_inputs);
        let mut index = combination_index;

        for _ in 0..self.config.num_inputs {
            combination.push(index % base);
            index /= base;
        }

        combination
    }

    /// Generate signal value for a specific input and polarization
    fn generate_signal_value(
        &self,
        input_idx: usize,
        pol_idx: usize,
        combination: &[usize],
        progress: f64,
    ) -> f64 {
        // Get the state for this input from the combination
        let input_state = combination[input_idx];

        // Determine the base signal value and polarization
        let polarization_value = input_state / 2; // 0 to num_polarization-1
        let signal_polarity = if input_state % 2 == 0 { 1.0 } else { -1.0 };

        // Only generate signal for the corresponding polarization
        if polarization_value == pol_idx {
            // Generate a square wave signal that varies with progress
            let frequency = 1.0 + input_idx as f64; // Different frequency per input
            signal_polarity
        } else {
            0.0
        }
    }
}
