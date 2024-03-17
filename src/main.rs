use population::Population;
use problem::{Item, Problem};

mod population;
mod problem;

fn main() {
    let problem = Problem {
        items: vec![
            Item::new(7, 5),
            Item::new(2, 4),
            Item::new(1, 7),
            Item::new(9, 2),
        ],
        weight_limit: 15,
    };

    let population = Population::new(&problem, 10);

    for solution in population.solutions() {
        dbg!(solution, problem.score_solution(solution));
        println!();
    }
}
