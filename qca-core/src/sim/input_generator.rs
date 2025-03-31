use std::f64::consts;

pub fn get_input_values(num_samples: usize, cur_sample: usize, num_inputs: usize) -> Vec<f64>{
    (0..num_inputs).map(|i| {
        (-1.0 * (f64::powi(2.0, i as i32) * cur_sample as f64 * ((2.0 * consts::PI) / num_samples as f64)).sin()).signum()
    }).collect()
}