mod agent;
mod evolution_engine;
mod objective;

use evolution_engine::engine::Engine;
use objective::trial::Trial;

#[tokio::main]
async fn main() {
    let num_generations = 1000;
    let num_agents = 10000;
    let top_k = 100;
    let mutation_percent = 20.0;
    let random_population_percent = 30.0;
    let trial = Box::new(Trial);
    let engine = Engine::new(
        num_generations,
        num_agents,
        top_k,
        random_population_percent,
        mutation_percent,
    );
    engine.start_the_royal_rumble(trial).await;
    println!("Evolution completed.");
}
