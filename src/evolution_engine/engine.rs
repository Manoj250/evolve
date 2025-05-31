use crate::agent::Agent;
use crate::objective::Objective;

pub struct Engine<T: Objective> {
    num_generations: i32,
    agents: Vec<Agent<T>>,
}

