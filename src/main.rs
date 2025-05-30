mod objective;

use objective::Objective;
use objective::trial::Trial;

fn main() {
    let trial = Trial::new(2.0, 3.0);
    let result = trial.evaluate();
    println!("The evaluation result is: {}", result);
}