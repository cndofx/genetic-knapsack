use crate::problem::Problem;

#[derive(Debug)]
pub struct Population<'p> {
    problem: &'p Problem,
    solutions: Vec<Vec<bool>>,
}

impl<'p> Population<'p> {
    pub fn new(problem: &'p Problem, size: usize) -> Self {
        Population {
            solutions: (0..size).map(|_| problem.random_solution()).collect(),
            problem,
        }
    }

    pub fn solutions(&self) -> &[Vec<bool>] {
        &self.solutions
    }
}
