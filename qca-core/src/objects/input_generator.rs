use super::generator::{Generator, GeneratorConfig};

/// Configuration for cell input generator
#[derive(Clone)]
pub struct CellInputConfig {
    /// Number of cell states (dimensions of the output vector)
    pub num_states: usize,
    /// Frequency of the square signal
    pub frequency: f64,
}

impl GeneratorConfig for CellInputConfig {}

/// Generator for cell input values that produces vectors of values
pub struct CellInputGenerator {
    config: CellInputConfig,
    num_samples: usize,
}

impl Generator for CellInputGenerator {
    type Config = CellInputConfig;
    type Output = Vec<f64>;

    fn new(config: Self::Config, num_samples: usize) -> Self {
        Self {
            config,
            num_samples,
        }
    }

    fn generate(&self, sample: usize) -> Self::Output {
        let segment_size = self.num_samples / self.config.num_states;
        let segment_i = sample / segment_size;
        let x = sample as f64 / segment_size as f64;

        (0..self.config.num_states)
            .map(|i| {
                if i == segment_i {
                    square_signal_function(x, self.config.frequency)
                } else {
                    0.0
                }
            })
            .collect()
    }

    fn num_samples(&self) -> usize {
        self.num_samples
    }
}

/// Legacy function for backward compatibility
pub fn generate_cell_input_sample(
    num_states: usize,
    sample: usize,
    num_samples: usize,
    frequency: f64,
) -> Vec<f64> {
    let config = CellInputConfig {
        num_states,
        frequency,
    };
    let generator = CellInputGenerator::new(config, num_samples);
    generator.generate(sample)
}

fn square_signal_function(x: f64, frequency: f64) -> f64 {
    let angle = (x * frequency) % 1.0;
    if angle < 0.25 || angle >= 0.75 {
        1.0
    } else {
        -1.0
    }
}
