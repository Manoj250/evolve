use crate::objective::Objective;


pub struct Agent<T: Objective> {
    weights: Vec<f64>,
    pub score: f64,
    pub objective: T,
}

impl<T: Objective + Send + Sync> Agent<T> {
    pub fn new(weights: Vec<f64>, objective: T) -> Self {
        Self {
            weights,
            score: 0.0,
            objective,
        }
    }

    pub async fn run(&mut self) {
        self.score = self.objective.evaluate(&self.weights).await;
    }
}
