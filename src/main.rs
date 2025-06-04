mod agent;
mod objective;
mod evolution_engine;

use agent::agent::Agent;
use objective::trial::Trial;
use evolution_engine::engine::Engine;

#[tokio::main]
async fn main() {
    let trial = Trial;
    let num_generations = 100;
    let num_agents = 100;
    let top_k = 10;
    let mutation_percent = 10.0;
    let engine = Engine::new(num_generations, num_agents, top_k);   
    engine.run(mutation_percent, trial).await;
    println!("Evolution completed."); 
}
