use rand::Rng;

type Weights = [u32; 6];

pub struct WeightedDie {
    weights: Weights,
    total: u32
}

pub struct WeightTransform {
    matrix: [[f64; 6]; 6]
}

fn sum_values(vec: Weights) -> u32 {
    vec.iter().fold(0, |mut acc, x| {
        acc += x;
        acc
    })
}

impl WeightedDie {
    pub fn fair_die() -> Self {
        WeightedDie { weights: [1; 6], total: 6 }
    }

    pub fn with_weights(weights: Weights) -> Self {
        let total = sum_values(weights);
        WeightedDie { weights, total }
    }

    pub fn weights(&self) -> Weights {
        self.weights
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

    pub fn apply_transformation(&mut self, transform: WeightTransform) {
        self.weights = transform.apply(self.weights);
        self.total = sum_values(self.weights);
    }
}

impl WeightTransform {
    pub fn identity() -> Self {
        let mut matrix = [[0.; 6]; 6];
        for i in 0..6 {
            matrix[i][i] = 1.;
        }
        WeightTransform { matrix }
    }

    pub fn boost_values(factor: f64, values: &[u32]) -> Self {
        debug_assert!(!factor.is_sign_negative());
        let mut boost = WeightTransform::identity();
        for v in values {
            let i = (*v - 1) as usize;
            boost.matrix[i][i] *= factor;
        }
        boost
    }

    pub fn with_matrix(matrix: [[f64; 6]; 6]) -> Self {
        WeightTransform { matrix }
    }

    pub fn rescale(&self, rhs: f64) -> WeightTransform {
        debug_assert!(!rhs.is_sign_negative());
        let mut matrix = self.matrix;
        for i in 0..6 {
            for j in 0..6 {
                matrix[i][j] *= rhs;
            }
        }
        WeightTransform { matrix }
    }

    pub fn apply(&self, rhs: Weights) -> Weights {
        let mut res = [0; 6];
        for i in 0..6 {
            for j in 0..6 {
                let term = self.matrix[i][j] * rhs[j] as f64;
                res[i] += term.round() as u32;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::dice::{WeightedDie, WeightTransform};

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

    #[test]
    fn transform() {
        let mut die = WeightedDie::fair_die();
        let boost = WeightTransform::boost_values(2., &[1u32, 6]);
        die.apply_transformation(boost);
        assert_eq!(die.weights, [2, 1, 1, 1, 1, 2]);
    }

    #[test]
    fn rescaling() {
        let identity = WeightTransform::identity();
        let double = identity.rescale(2.);
        for i in 0..6 {
            for j in 0..6 {
                if i == j {
                    assert_eq!(identity.matrix[i][i], 1.);
                    assert_eq!(double.matrix[i][i], 2.);
                } else {
                    assert_eq!(identity.matrix[i][j], 0.);
                    assert_eq!(double.matrix[i][j], 0.);
                }
            }
        }
    }
}
