mod agent;
mod evolution_engine;
mod objective;

use evolution_engine::engine::Engine;
use objective::trial::Trial;

#[tokio::main]
async fn main() {
    let num_generations = 1000;
    let num_agents = 1000;
    let top_k = 100;
    let mutation_percent = 10.0;
    let trial = Box::new(Trial);
    let engine = Engine::new(num_generations, num_agents, top_k);
    engine.start_the_royal_rumble(mutation_percent, trial).await;
    println!("Evolution completed.");
}
