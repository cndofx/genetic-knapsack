use rand::prelude::*;

use crate::problem::Problem;

#[derive(Debug)]
pub struct Population<'p> {
    problem: &'p Problem,
    solutions: Vec<Vec<bool>>,
    crossover_rate: f64,
    mutation_rate: f64,
}

impl<'p> Population<'p> {
    /// panics if size is not a multiple of 2
    pub fn new(problem: &'p Problem, size: usize, crossover_rate: f64, mutation_rate: f64) -> Self {
        assert!(size % 2 == 0);
        Population {
            solutions: (0..size).map(|_| problem.random_solution()).collect(),
            problem,
            crossover_rate,
            mutation_rate,
        }
    }

    pub fn next_generation(&mut self) {
        let mut rng = rand::thread_rng();

        let parents = self.select_parents(&mut rng);
        let mut children = self.crossover(&parents, &mut rng);
        self.mutate(&mut children, &mut rng);

        self.solutions = children;
    }

    pub fn solutions(&self) -> &[Vec<bool>] {
        &self.solutions
    }

    pub fn best_solution(&self) -> &[bool] {
        self.solutions
            .iter()
            .max_by_key(|s| self.problem.score_solution(s))
            .unwrap()
    }

    fn select_parents(&self, rng: &mut ThreadRng) -> Vec<Vec<bool>> {
        let len = self.solutions.len();
        let mut parents = Vec::with_capacity(len);

        loop {
            let mut choose = self.solutions.choose_multiple(rng, 2);
            let solution1 = choose.next().unwrap();
            let solution2 = choose.next().unwrap();
            let score1 = self.problem.score_solution(solution1);
            let score2 = self.problem.score_solution(solution2);

            if score1 >= score2 {
                parents.push(solution1.clone());
            } else {
                parents.push(solution2.clone());
            }

            if parents.len() == len {
                break;
            }
        }

        parents
    }

    fn crossover(&self, parents: &[Vec<bool>], rng: &mut ThreadRng) -> Vec<Vec<bool>> {
        let mut children = Vec::with_capacity(parents.len());

        for parents in parents.chunks_exact(2) {
            let parent1 = &parents[0];
            let parent2 = &parents[1];
            assert_eq!(parent1.len(), parent2.len());
            let len = parent1.len();

            if rng.gen_bool(self.crossover_rate) {
                let point = rng.gen_range(0..len);
                let p1_1 = &parent1[..point];
                let p1_2 = &parent1[point..];
                let p2_1 = &parent2[..point];
                let p2_2 = &parent2[point..];

                let child1 = [p1_1, p2_2].into_iter().flatten().copied().collect();
                let child2 = [p2_1, p1_2].into_iter().flatten().copied().collect();

                children.push(child1);
                children.push(child2);
            } else {
                children.push(parent1.clone());
                children.push(parent2.clone());
            }
        }

        children
    }

    fn mutate(&self, children: &mut Vec<Vec<bool>>, rng: &mut ThreadRng) {
        for child in children.iter_mut() {
            if rng.gen_bool(self.mutation_rate) {
                let b = child.choose_mut(rng).unwrap();
                *b = !*b;
            }
        }
    }
}
