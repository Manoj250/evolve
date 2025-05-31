use crate::objective::Objective;

pub struct Trial;

impl Objective for Trial {
    fn evaluate(&self, weights: &[f64]) -> f64 {
        let x = weights[0];
        let y = weights[1];
        let result = (x - 3.0).powi(2) + (y + 1.0).powi(2) + (3.0 * x).sin() * (5.0 * y).cos();
        -result
    }
}
