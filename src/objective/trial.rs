use crate::objective::Objective;
use async_trait::async_trait;

#[derive(Clone)]
pub struct Trial;

#[async_trait]
impl Objective for Trial {
    async fn evaluate(&self, weights: &[f64]) -> f64 {
        let x = weights[0];
        let y = weights[1];
        let result = (x - 3.0).powi(2) + (y + 1.0).powi(2) + (3.0 * x).sin() * (5.0 * y).cos();
        -result
    }

    fn get_weight_length(&self) -> usize {
        2
    }

    fn clone_box(&self) -> Box<dyn Objective + Send + Sync> {
        Box::new(self.clone())
    }
}
