use crate::objects::generator::{Generator, GeneratorConfig};
use serde::{Deserialize, Serialize};

/// Configuration for cell input generator
#[derive(Clone, Serialize, Deserialize)]
pub struct CellInputConfig {
    /// Number of samples per combination
    pub num_samples_per_combination: usize,
    /// Number of cell inputs
    pub num_inputs: usize,
    /// Number of polarization states
    pub num_polarization: usize,
    /// Number of extra clock periods to generate
    pub extra_clock_periods: usize,
}

impl GeneratorConfig for CellInputConfig {}

/// Generator for cell input values that produces vectors of values
/// Output dimension: num_inputs * num_polarization
/// Total combinations: (num_polarization + 1)^num_inputs
pub struct CellInputGenerator {
    config: CellInputConfig,
    num_samples: usize,
    extra_samples: usize,
}

impl Generator for CellInputGenerator {
    type Config = CellInputConfig;
    type Output = Vec<f64>;

    fn new(config: Self::Config) -> Self {
        let input_combinations = (config.num_polarization + 1).pow(config.num_inputs as u32);
        let extra_samples = config.extra_clock_periods * config.num_samples_per_combination;
        let num_samples = config.num_samples_per_combination * input_combinations + extra_samples;
        Self {
            config,
            num_samples,
            extra_samples,
        }
    }

    fn generate(&self, sample: usize) -> Option<Self::Output> {
        if sample >= self.num_samples {
            return None;
        }

        if sample >= self.num_samples - self.extra_samples {
            return Some(vec![
                0.0;
                self.config.num_inputs * self.config.num_polarization
            ]);
        }

        // Calculate which combination we're in
        let samples_per_combination = self.config.num_samples_per_combination;
        let combination_index = sample / samples_per_combination;

        // Generate the combination pattern
        let combination = self.get_combination(combination_index);

        // Generate output vector with dimension num_inputs * num_polarization
        let mut output = Vec::with_capacity(self.config.num_inputs * self.config.num_polarization);

        for input_idx in 0..self.config.num_inputs {
            for pol_idx in 0..self.config.num_polarization {
                let value = self.generate_signal_value(input_idx, pol_idx, &combination);
                output.push(value);
            }
        }

        Some(output)
    }

    fn num_samples(&self) -> usize {
        self.num_samples
    }
}

impl CellInputGenerator {
    /// Get the combination pattern for a given combination index
    /// Each input can have values from 0 to (num_polarization)
    fn get_combination(&self, combination_index: usize) -> Vec<usize> {
        let base = self.config.num_polarization + 1;
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
    ) -> f64 {
        // Get the state for this input from the combination
        let input_state = combination[input_idx];

        // Determine the base signal value and polarization
        let polarization_value = input_state / 2; // 0 to num_polarization-1
        let signal_polarity = if input_state % 2 == 0 { 1.0 } else { -1.0 };

        // Only generate signal for the corresponding polarization
        if polarization_value == pol_idx {
            // Generate a square wave signal that varies with progress
            signal_polarity
        } else {
            0.0
        }
    }
}
