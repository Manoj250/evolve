use async_trait::async_trait;

#[async_trait]
pub trait Objective {
    async fn evaluate(&self, weights: &[f64]) -> f64;
}