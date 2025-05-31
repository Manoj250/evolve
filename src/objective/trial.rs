use crate::objective::Objective;

pub struct Trial {
    pub x: f64,
    pub y: f64,
}

impl Trial {
    pub fn new(weights: &[f64]) -> Self {
        Self {
            x: weights[0],
            y: weights[1],
        }
    }
}

impl Objective for Trial {
    fn evaluate(&self) -> f64 {
        (self.x - 3.0).powi(2)
            + (self.y + 1.0).powi(2)
            + (3.0 * self.x).sin() * (5.0 * self.y).cos()
    }
}
