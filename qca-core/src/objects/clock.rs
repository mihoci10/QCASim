use std::convert::TryInto;
use std::f64::consts::PI;

pub fn get_clock_values(
    num_samples: usize,
    cur_sample: usize,
    _num_inputs: usize,
    ampl_min: f64,
    ampl_max: f64,
    _ampl_fac: f64,
) -> [f64; 4] {
    // let prefactor = (ampl_max - ampl_min) * ampl_fac;
    // let repetitions = f64::powi(2.0, num_inputs as i32);
    // let clock_shift = ampl_max - ampl_min;

    // (0..4).map(|i| {
    //     (prefactor * (repetitions * (cur_sample as f64) * ((2.0 * consts::PI) / num_samples as f64) - (consts::PI * (i as f64) / 2.0) - consts::PI).cos() + clock_shift)
    //     .clamp(ampl_min, ampl_max)
    // }).collect::<Vec<f64>>().try_into().unwrap()

    (0..4)
        .map(|i| {
            let mut clock = (cur_sample as f64 / num_samples as f64) - (i as f64 * 0.25);
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
        .unwrap()
}
