fn square_signal_function(x: f64, frequency: f64) -> f64 {
    let angle = (x * frequency) % 1.0;
    if angle < 0.25 || angle >= 0.75 {
        1.0
    } else {
        -1.0
    }
}

pub fn generate_cell_input_sample(
    num_states: usize,
    sample: usize,
    num_samples: usize,
    frequency: f64,
) -> Vec<f64> {
    let segment_size = num_samples / num_states;
    let segment_i = sample / segment_size;
    let x = sample as f64 / segment_size as f64;
    (0..num_states)
        .map(|i| {
            if i == segment_i {
                square_signal_function(x, frequency)
            } else {
                0.0
            }
        })
        .collect()
}
