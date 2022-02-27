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
