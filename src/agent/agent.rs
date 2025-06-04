use crate::objective::Objective;

pub struct Agent {
    pub weights: Vec<f64>,
    pub score: f64,
    pub objective: Box<dyn Objective + Send + Sync>,
}

impl Agent {
    pub fn new(weights: Vec<f64>, objective: Box<dyn Objective + Send + Sync>) -> Self {
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
