mod agent;
mod objective;

use agent::agent::Agent;
use objective::trial::Trial;
use objective::Objective;

fn main() {
    let weights = vec![2.0, 3.0];

    let trial = Trial::new(&weights);
    let agent = Agent::new(weights, trial);

    agent.objective.evaluate();

    println!("Agent score: {}", agent.score);
}
