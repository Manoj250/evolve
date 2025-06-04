use crate::agent::Agent;
use crate::objective::Objective;

pub struct Engine<T: Objective> {
    pub num_generations: i32,
    pub num_agents: i32,
    pub top_k: i32,
}

impl<T: Objective + Send + Sync + 'static> Engine<T> {
    pub fn new(num_generations: i32, num_agents: i32, top_k: i32) -> Self {
        Self {
            num_generations,
            num_agents,
            top_k,
        }
    }

    fn generate_random_weights(length: usize) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        (0..length).map(|_| rng.gen_range(-1.0..1.0)).collect()
    }

    fn mutate(weights: &[f64], mutation_percent: f64) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        weights
            .iter()
            .map(|w| {
                let mutation = 1.0 + rng.gen_range(-mutation_percent..mutation_percent) / 100.0;
                w * mutation
            })
            .collect()
    }

    pub async fn run(&self, mutation_percent: f64, objective: T) {

        let weight_len = objective.get_weight_length();

        let mut population: Vec<Agent<T>> = (0..self.num_agents)
            .map(|_| Agent::new(generate_random_weights(weight_len), objective.clone()))
            .collect();

        for _ in 0..self.num_generations {
            // Run all agents concurrently
            let futures = population.iter_mut().map(|agent| agent.run());
            join_all(futures).await;

            // Sort by score DESCENDING
            population.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

            println!("Top Score: {}", population[0].score);

            // Keep top_k, mutate to make new population
            let elites = &population[0..self.top_k as usize];
            population = (0..self.num_agents)
                .map(|_| {
                    let parent = &elites[rand::thread_rng().gen_range(0..elites.len())];
                    let mutated_weights = mutate(&parent.weights, mutation_percent);
                    Agent::new(mutated_weights, objective.clone())
                })
                .collect();
        }
    }
}
