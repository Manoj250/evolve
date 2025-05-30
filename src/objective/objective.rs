pub trait Objective {
    fn evaluate(&self) -> f64;
}