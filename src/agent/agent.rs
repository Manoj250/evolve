use crate::objective::Objective;

pub struct Agent<T: Objective> {
    weights: Vec<f64>,
    pub score: f32,
    pub objective: T,
}

impl<T: Objective> Agent<T> {
    pub fn new(weights: Vec<f64>, objective: T) -> Self {
        Self {
            weights,
            score: 0.0,
            objective,
        }
    }
}
