use rand::Rng;

#[derive(Debug, Clone)]
pub struct Problem {
    pub items: Vec<Item>,
    pub weight_limit: u16,
}

impl Problem {
    pub fn random_solution(&self) -> Vec<bool> {
        let mut rng = rand::thread_rng();
        self.items.iter().map(|_| rng.gen()).collect()
    }

    pub fn score_solution(&self, solution: &[bool]) -> i32 {
        assert_eq!(self.items.len(), solution.len());

        let (weight, value) = self.items.iter().zip(solution.iter()).fold(
            (0, 0),
            |(acc_weight, acc_value), (item, taken)| {
                let taken = *taken as u16;
                let new_weight = acc_weight + item.weight * taken;
                let new_value = acc_value + item.value * taken;
                (new_weight, new_value)
            },
        );

        if weight > self.weight_limit {
            -1
        } else {
            value as i32
        }
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub weight: u16,
    pub value: u16,
}

impl Item {
    pub fn new(weight: u16, value: u16) -> Self {
        Item { weight, value }
    }
}
