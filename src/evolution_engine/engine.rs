use crate::agent::agent::Agent;
use crate::objective::Objective;
use futures::future::join_all;
use rand::Rng;
use std::fs::OpenOptions;
use std::io::Write;

pub struct Engine {
    pub num_generations: i32,
    pub num_agents: i32,
    pub top_k: i32,
}

impl Engine {
    pub fn new(num_generations: i32, num_agents: i32, top_k: i32) -> Self {
        Self {
            num_generations,
            num_agents,
            top_k,
        }
    }

    fn generate_random_weights(length: usize) -> Vec<f64> {
        let mut rng = rand::rng();
        (0..length).map(|_| rng.random_range(-1.0..1.0)).collect()
    }

    fn mutate(weights: &[f64], mutation_percent: f64) -> Vec<f64> {
        let mut rng = rand::rng();
        weights
            .iter()
            .map(|w| {
                let mutation = 1.0 + rng.random_range(-mutation_percent..mutation_percent) / 100.0;
                w * mutation
            })
            .collect()
    }

    fn write_to_file(elites: &[Agent]) {
        let mut file = OpenOptions::new()
            .create(true) // create if not exists
            .append(true) // append, don’t erase
            .open("top_performers.csv")
            .expect("Can't open file");

        for agent in elites {
            let weight_str = agent
                .weights
                .iter()
                .map(|w| w.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            writeln!(file, "{},{}", agent.score, weight_str).unwrap();
        }
    }

    pub async fn start_the_royal_rumble(
        &self,
        mutation_percent: f64,
        objective: Box<dyn Objective + Send + Sync>,
    ) {
        let weight_len = objective.get_weight_length();

        let mut population: Vec<Agent> = (0..self.num_agents)
            .map(|_| {
                Agent::new(
                    Self::generate_random_weights(weight_len),
                    objective.clone_box(),
                )
            })
            .collect();

        for _ in 0..self.num_generations {
            let futures = population.iter_mut().map(|agent| agent.run());
            join_all(futures).await;

            population.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

            println!("Top Score: {}", population[0].score);

            let elites = &population[0..self.top_k as usize];
            Self::write_to_file(elites);

            population = (0..self.num_agents)
                .map(|_| {
                    let parent = &elites[rand::rng().random_range(0..elites.len())];
                    let mutated_weights = Self::mutate(&parent.weights, mutation_percent);
                    Agent::new(mutated_weights, objective.clone_box())
                })
                .collect();
        }
    }
}
