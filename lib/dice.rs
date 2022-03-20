// MIT/Apache 2.0 dual license
// Apache 2.0
// Copyright 2022 Arc676/Alessandro Vinciguerra <alesvinciguerra@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// MIT
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use bevy_egui::egui::emath::RectTransform;
use bevy_egui::egui::*;
use num_complex::Complex64 as c64;
use num_traits::identities::{One, Zero};
use rand::Rng;

type Weights = [c64; 6];
pub struct WeightedDie {
    weights: Weights,
}

type Matrix = [[c64; 6]; 6];
pub struct WeightTransform {
    matrix: Matrix,
}

impl Clone for WeightedDie {
    fn clone(&self) -> Self {
        WeightedDie {
            weights: self.weights,
        }
    }
}

impl WeightedDie {
    pub fn fair_die() -> Self {
        WeightedDie {
            weights: [c64::from((1f64 / 6.).sqrt()); 6],
        }
    }

    pub fn with_weights(weights: Weights) -> Self {
        if cfg!(debug_assertions) {
            let mut total = 0.;
            for w in weights {
                total += w.norm_sqr();
            }
            debug_assert!((total - 1.).abs() < 1e-12);
        }
        WeightedDie { weights }
    }

    pub fn weights(&self) -> Weights {
        self.weights
    }

    pub fn roll(&self) -> u32 {
        let mut roll: f64 = rand::thread_rng().gen_range(0.0..1.0);
        for (value, weight) in self.weights.iter().enumerate() {
            if roll < weight.norm_sqr() {
                return value as u32 + 1;
            }
            roll -= weight.norm_sqr();
        }
        panic!("Failed to roll a number");
    }

    pub fn apply_transformation(&mut self, transform: &WeightTransform) {
        self.weights = transform.apply(self.weights);
    }

    pub fn visualize_weights(&self, painter: &Painter, to_screen: RectTransform, color: Color32) {
        for (i, weight) in self.weights.iter().enumerate() {
            let face = i + 1;
            painter.rect_filled(
                Rect::from([
                    to_screen
                        * Pos2 {
                            x: face as f32 / 7. - 1. / 28.,
                            y: 0.9 - weight.norm_sqr() as f32,
                        },
                    to_screen
                        * Pos2 {
                            x: face as f32 / 7. + 1. / 28.,
                            y: 0.9,
                        },
                ]),
                0.,
                color,
            );
        }
    }
}

impl WeightTransform {
    pub fn identity() -> Self {
        let mut matrix = [[c64::zero(); 6]; 6];
        #[allow(clippy::needless_range_loop)]
        for i in 0..6 {
            matrix[i][i] = c64::one();
        }
        WeightTransform { matrix }
    }

    #[allow(clippy::needless_range_loop)]
    pub fn matrix_product(a: &Matrix, b: &Matrix) -> Matrix {
        let mut combined = [[c64::zero(); 6]; 6];
        for i in 0..6 {
            for j in 0..6 {
                for k in 0..6 {
                    combined[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        combined
    }

    pub fn combined_with(&self, other: &WeightTransform) -> Self {
        let matrix = WeightTransform::matrix_product(&self.matrix, &other.matrix);
        #[cfg(debug_assertions)]
        debug_assert!(WeightTransform::is_unitary(&matrix));
        WeightTransform { matrix }
    }

    #[cfg(debug_assertions)]
    #[allow(clippy::needless_range_loop)]
    fn is_unitary(matrix: &Matrix) -> bool {
        let mut cc = [[c64::zero(); 6]; 6];
        for i in 0..6 {
            for j in 0..6 {
                cc[i][j] = matrix[j][i].conj();
            }
        }
        let product = WeightTransform::matrix_product(matrix, &cc);
        for i in 0..6 {
            for j in 0..6 {
                let term = product[i][j];
                if i == j {
                    if (term - c64::one()).norm() > 1e-12 {
                        return false;
                    }
                } else if term.norm() > 1e-12 {
                    return false;
                }
            }
        }
        true
    }

    pub fn with_matrix(matrix: Matrix) -> Self {
        #[cfg(debug_assertions)]
        debug_assert!(WeightTransform::is_unitary(&matrix));
        WeightTransform { matrix }
    }

    pub fn superimpose_pair(v1: u32, v2: u32, transfer: f64) -> Self {
        debug_assert!(transfer <= 1.);
        debug_assert!(transfer >= 0.);

        let (v1, v2) = (v1 as usize - 1, v2 as usize - 1);
        let mut transform = WeightTransform::identity();
        let a = c64::from((transfer / 2.).sqrt());
        let b = c64::from(((2. - transfer) / 2.).sqrt());

        transform.matrix[v1][v1] = a;
        transform.matrix[v2][v2] = a;
        transform.matrix[v1][v2] = b;
        transform.matrix[v2][v1] = -b;

        #[cfg(debug_assertions)]
        debug_assert!(WeightTransform::is_unitary(&transform.matrix));

        transform
    }

    #[allow(clippy::needless_range_loop)]
    pub fn apply(&self, rhs: Weights) -> Weights {
        let mut res = [c64::zero(); 6];
        for i in 0..6 {
            for j in 0..6 {
                res[i] += self.matrix[i][j] * rhs[j];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::dice::{WeightTransform, WeightedDie};
    use num_complex::Complex64 as c64;

    fn generate_rolls(die: &WeightedDie, count: u32) -> [i32; 6] {
        let mut results = [0; 6];
        for _ in 0..count {
            let roll = die.roll();
            results[roll as usize - 1] += 1;
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
        let die = WeightedDie::with_weights([
            c64::from((1f64 / 21.).sqrt()),
            c64::from((2f64 / 21.).sqrt()),
            c64::from((3f64 / 21.).sqrt()),
            c64::from((4f64 / 21.).sqrt()),
            c64::from((5f64 / 21.).sqrt()),
            c64::from((6f64 / 21.).sqrt()),
        ]);
        let results = generate_rolls(&die, 1000);
        dbg!(results.map(|x| x as f64 / results[0] as f64));
    }

    #[test]
    fn superposition() {
        let mut die = WeightedDie::fair_die();
        // Transfer all weight from 2 to 1
        let transform = WeightTransform::superimpose_pair(1, 2, 1.);
        die.apply_transformation(&transform);
        dbg!(die.weights);
    }

    #[test]
    fn multiple_transformations() {
        let m1 = WeightTransform::superimpose_pair(1, 3, 1.);
        assert!(WeightTransform::is_unitary(&m1.matrix));
        let m2 = WeightTransform::superimpose_pair(2, 4, 1.);
        assert!(WeightTransform::is_unitary(&m2.matrix));
        let m3 = m1.combined_with(&m2);
        assert!(WeightTransform::is_unitary(&m3.matrix));
    }
}
