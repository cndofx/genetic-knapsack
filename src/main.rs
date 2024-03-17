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
            Item::new(9, 3),
            Item::new(5, 1),
            Item::new(3, 7),
            Item::new(7, 2),
            Item::new(5, 5),
            Item::new(1, 2),
            Item::new(11, 9),
        ],
        weight_limit: 20,
    };

    let mut population = Population::new(&problem, 100, 0.6, 0.1, 4);

    for i in 0.. {
        let best = population.best_solution();
        let score = problem.score_solution(best);
        println!("gen {i} best: {best:?}, score: {score}");
        population.next_generation();

        // std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
