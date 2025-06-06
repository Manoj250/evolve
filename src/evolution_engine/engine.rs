use crate::agent::agent::Agent;
use crate::objective::Objective;
use futures::future::join_all;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::task::JoinHandle;

pub struct Engine {
    pub num_generations: i32,
    pub num_agents: i32,
    pub top_k: i32,
    pub random_population_percent: f64,
    pub mutation_percent: f64,
}

impl Engine {
    pub fn new(
        num_generations: i32,
        num_agents: i32,
        top_k: i32,
        random_population_percent: f64,
        mutation_percent: f64,
    ) -> Self {
        Self {
            num_generations,
            num_agents,
            top_k,
            random_population_percent,
            mutation_percent,
        }
    }

    fn generate_random_weights(length: usize) -> Vec<f64> {
        // Generate random weights using a normal distribution
        // with mean 0 and standard deviation 1
        let normal = Normal::new(0.0, 1.0).unwrap();
        let mut rng = rand::thread_rng();
        (0..length).map(|_| normal.sample(&mut rng)).collect()
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

    async fn write_to_file(elites: &[Agent]) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("top_performers.csv")
            .await
            .expect("Can't open file");

        for agent in elites {
            let weight_str = agent
                .weights
                .iter()
                .map(|w| w.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            let line = format!("{},{}\n", agent.score, weight_str);
            file.write_all(line.as_bytes()).await.unwrap();
        }
    }

    fn crossover(dad: &Agent, mom: &Agent) -> Vec<f64> {
        dad.weights
            .iter()
            .zip(&mom.weights)
            .map(|(w1, w2)| if rand::random() { *w1 } else { *w2 })
            .collect()
    }

    pub async fn start_the_royal_rumble(&self, objective: Box<dyn Objective + Send + Sync>) {
        let weight_len = objective.get_weight_length();
        let mut write_tasks: Vec<JoinHandle<()>> = Vec::new();

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
            // Spawn async write, push JoinHandle to vector
            let elites_clone = elites.to_vec(); // clone for move into async block
            let handle = tokio::spawn(async move {
                Engine::write_to_file(&elites_clone).await;
            });
            write_tasks.push(handle);

            let num_randoms = ((self.random_population_percent / 100.0) * self.num_agents as f64)
                .round() as usize;
            let num_offspring = self.num_agents as usize - num_randoms;
            let males = elites;
            let females = &population[self.top_k as usize..];
            // Generate offspring via crossover
            let mut new_population: Vec<Agent> = (0..num_offspring)
                .map(|_| {
                    let dad = &males[rand::thread_rng().gen_range(0..males.len())];
                    let mom = &females[rand::thread_rng().gen_range(0..females.len())];

                    let mixed_weights = Self::crossover(dad, mom);
                    let mutated_weights = Self::mutate(&mixed_weights, self.mutation_percent);
                    Agent::new(mutated_weights, objective.clone_box())
                })
                .collect();

            // Add new randoms to spice up the gene pool
            new_population.extend((0..num_randoms).map(|_| {
                Agent::new(
                    Self::generate_random_weights(weight_len),
                    objective.clone_box(),
                )
            }));

            population = new_population;
        }
        join_all(write_tasks).await;
        println!("Evolution complete. Top performers written to file.");
    }
}
