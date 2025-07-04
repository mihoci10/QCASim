use crate::objects::generator::{Generator, GeneratorConfig};
use std::convert::TryInto;
use std::f64::consts::PI;

/// Configuration for clock generator
#[derive(Clone)]
pub struct ClockConfig {
    /// Number of samples
    pub num_samples: usize,
    /// Number of clock cycles
    pub num_cycles: usize,
    /// Minimum amplitude value
    pub amplitude_min: f64,
    /// Maximum amplitude value
    pub amplitude_max: f64,
    /// Extend last cycle to the end of the simulation
    pub extend_last_cycle: bool,
}

impl GeneratorConfig for ClockConfig {}

/// Generator for clock values that produces arrays of 4 clock values
pub struct ClockGenerator {
    config: ClockConfig,
}

impl Generator for ClockGenerator {
    type Config = ClockConfig;
    type Output = [f64; 4];

    fn new(config: Self::Config) -> Self {
        Self { config }
    }

    fn generate(&self, sample: usize) -> Option<Self::Output> {
        if sample >= self.config.num_samples {
            return None;
        }

        let clock_cycles = self.config.num_cycles;
        let samples_per_cycle = self.config.num_samples / clock_cycles;
        let ampl_min = self.config.amplitude_min;
        let ampl_max = self.config.amplitude_max;

        // let clock_extension = if self.config.extend_last_cycle {
        //     1.0 / 4.0
        // } else {
        //     0.0
        // };

        let sample_fac = sample.rem_euclid(samples_per_cycle) as f64 / samples_per_cycle as f64;
        Some(
            (0..4)
                .map(|i| {
                    let mut clock = sample_fac - (i as f64 * 0.25);
                    clock = clock.rem_euclid(1.0);
                    if clock < 0.25 {
                        (1.0 + ((1.0 - clock / 0.25) * PI).cos()) / 2.0
                    } else if clock < 0.5 {
                        1.0
                    } else if clock < 0.75 {
                        (1.0 + (PI * (clock - 0.5) / 0.25).cos()) / 2.0
                    } else {
                        0.0
                    }
                })
                .map(|v| -((ampl_max - ampl_min) * (1.0 - v) + ampl_min))
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        )
    }

    fn num_samples(&self) -> usize {
        self.config.num_samples
    }
}
