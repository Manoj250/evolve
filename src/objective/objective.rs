pub trait Objective {
    fn evaluate(&self, weights: &[f64]) -> f64;
}