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
    /// When set, step through this explicit, ordered list of input vectors
    /// (repeats allowed) instead of exhaustively enumerating every
    /// combination. Each vector holds one state index per input, in the
    /// same encoding `get_combination` produces.
    pub custom_sequence: Option<Vec<Vec<usize>>>,
}

impl GeneratorConfig for CellInputConfig {}

impl CellInputConfig {
    /// Number of samples generated for `num_combinations` input combinations,
    /// including the trailing extra clock periods.
    pub fn num_samples(&self, num_combinations: usize) -> usize {
        self.num_samples_per_combination * num_combinations + self.extra_samples()
    }

    fn extra_samples(&self) -> usize {
        self.extra_clock_periods * self.num_samples_per_combination
    }
}

/// Number of input combinations a simulation steps through: the length of
/// the custom sequence when one is given (repeats included), otherwise every
/// combination, `(2 * num_polarization)^num_inputs`.
pub fn num_combinations(
    num_polarization: usize,
    num_inputs: usize,
    custom: Option<&[Vec<usize>]>,
) -> usize {
    match custom {
        Some(sequence) => sequence.len(),
        None => (num_polarization * 2).pow(num_inputs as u32),
    }
}

/// Generator for cell input values that produces vectors of values
/// Output dimension: num_inputs * num_polarization
/// Total combinations: (2 * num_polarization)^num_inputs
pub struct CellInputGenerator {
    config: CellInputConfig,
    num_samples: usize,
    extra_samples: usize,
}

impl Generator for CellInputGenerator {
    type Config = CellInputConfig;
    type Output = Vec<f64>;

    fn new(config: Self::Config) -> Self {
        let input_combinations = num_combinations(
            config.num_polarization,
            config.num_inputs,
            config.custom_sequence.as_deref(),
        );
        let extra_samples = config.extra_samples();
        let num_samples = config.num_samples(input_combinations);
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
        let combination = match &self.config.custom_sequence {
            Some(sequence) => sequence[combination_index].clone(),
            None => self.get_combination(combination_index),
        };

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
    /// Each input can have values from 0 to (2 * num_polarization - 1)
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
    ) -> f64 {
        // Get the state for this input from the combination
        let input_state = combination[input_idx];

        // Determine the base signal value and polarization
        let polarization_value = input_state / 2; // 0 to num_polarization-1
        let offset = input_state % 2;
        // Combination indices must enumerate states in literature order
        // (A, B, C, D, ...): the first polarization's states run
        // negative-then-positive (A, B), matching the truth table's
        // A = -1 / B = +1 convention, while every following
        // polarization's states run positive-then-negative (C, D, ...)
        // to match its A/B-independent labeling.
        let signal_polarity = if polarization_value == 0 {
            if offset == 0 {
                -1.0
            } else {
                1.0
            }
        } else {
            if offset == 0 {
                1.0
            } else {
                -1.0
            }
        };

        // Only generate signal for the corresponding polarization
        if polarization_value == pol_idx {
            // Generate a square wave signal that varies with progress
            signal_polarity
        } else {
            0.0
        }
    }
}
