use rand::Rng;

pub struct WeightedDie {
    weights: [u32; 6],
    total: u32
}

impl WeightedDie {
    pub fn fair_die() -> Self {
        WeightedDie { weights: [1; 6], total: 6 }
    }

    pub fn with_weights(weights: [u32; 6]) -> Self {
        let mut total = 0;
        for w in weights {
            total += w;
        }
        WeightedDie { weights, total }
    }

    pub fn roll(&self) -> u32 {
        let mut roll = rand::thread_rng().gen_range(0..self.total);
        for (value, weight) in self.weights.iter().enumerate() {
            if roll < *weight {
                return value as u32;
            }
            roll -= weight;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::dice::WeightedDie;

    fn generate_rolls(die: &WeightedDie, count: u32) -> [i32; 6] {
        let mut results = [0; 6];
        for _ in 0..count {
            let roll = die.roll();
            results[roll as usize] += 1;
        }
        results
    }

    #[test]
    fn fair_rolls() {
        let die = WeightedDie::fair_die();
        let results = generate_rolls(&die, 1000);
        dbg!(results);
    }

    #[test]
    fn unfair_rolls() {
        let die = WeightedDie::with_weights([1, 2, 3, 4, 5, 6]);
        let results = generate_rolls(&die, 1000);
        dbg!(results);
        dbg!(results.map(|x| x as f64 / results[0] as f64));
    }
}
