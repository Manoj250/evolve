use async_trait::async_trait;

#[async_trait]
pub trait Objective: Send + Sync {
    async fn evaluate(&self, weights: &[f64]) -> f64;
    fn get_weight_length(&self) -> usize;
    fn clone_box(&self) -> Box<dyn Objective + Send + Sync>;
}

impl Clone for Box<dyn Objective + Send + Sync> {
    fn clone(&self) -> Box<dyn Objective + Send + Sync> {
        self.clone_box()
    }
}
