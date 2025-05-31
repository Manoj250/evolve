mod agent;
mod objective;

use agent::agent::Agent;
use objective::trial::Trial;

fn main() {
    let weights = vec![2.0, 3.0];
    let trial = Trial;
    let mut agent = Agent::new(weights, trial);
    agent.run();
    println!("Agent score: {}", agent.score);
}
