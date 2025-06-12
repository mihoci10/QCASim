use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QCACellArchitecture {
    pub side_length: f64,
    pub dot_diameter: f64,
    pub dot_count: u8,
    pub dot_positions: Vec<[f64; 2]>,
    pub dot_tunnels: Vec<(u8, u8)>,
}

impl QCACellArchitecture {
    pub fn new(side_length: f64, dot_diameter: f64, dot_count: u8, dot_radius: f64) -> Self {
        QCACellArchitecture {
            side_length: side_length,
            dot_diameter: dot_diameter,
            dot_count: dot_count,
            dot_positions: (0..dot_count)
                .map(|i| {
                    let angle = (2.0 * PI / dot_count as f64) * i as f64;
                    [angle.cos() * dot_radius, angle.sin() * dot_radius]
                })
                .collect(),
            dot_tunnels: (0..dot_count)
                .map(|i| {
                    (
                        (i as i16 - 1).rem_euclid(dot_count as i16) as u8,
                        (i as i16 + 1).rem_euclid(dot_count as i16) as u8,
                    )
                })
                .collect(),
        }
    }
}
