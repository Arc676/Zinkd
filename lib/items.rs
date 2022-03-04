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

use crate::dice::{WeightTransform, WeightedDie};
use crate::player::Player;
use bevy::ecs::component::Component;
use rand::Rng;
use std::fmt::{Display, Formatter};

pub type HeldItem = Box<dyn Item>;
pub type PossibleItem = Option<HeldItem>;

pub trait Item: Send + Sync {
    fn short_description(&self) -> &str;
    fn full_description(&self) -> &str;
    fn use_item(&self, player: &mut Player);
    fn use_item_on_die(&self, die: &mut WeightedDie);
    fn item_type(&self) -> ItemType;

    fn create_tooltip(&self) -> ItemTooltip {
        ItemTooltip(self.short_description().to_string())
    }
}

#[derive(Component)]
pub struct ItemTooltip(pub String);

const ITEM_TYPES: u32 = 3;
#[derive(Copy, Clone)]
pub enum ItemType {
    WeightTransfer,
    DoubleWeightTransfer,
    WeightTransferPair,
}

impl Display for ItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemType::WeightTransfer => write!(f, "Weight Transfer"),
            ItemType::DoubleWeightTransfer => write!(f, "2x Weight Transfer"),
            ItemType::WeightTransferPair => write!(f, "Pair of Weight Transfers"),
        }
    }
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::WeightTransfer
    }
}

pub fn random_item() -> HeldItem {
    let mut rng = rand::thread_rng();
    Box::new(match rng.gen_range(0..ITEM_TYPES) {
        0 => WeightTransfer::random_single(),
        1 => WeightTransfer::random_double(),
        2 => WeightTransfer::random_pair(),
        _ => panic!("Unknown item type"),
    })
}

pub struct WeightTransfer {
    item_type: ItemType,
    transform: WeightTransform,
    short: String,
    full: String,
}

fn random_transfer_parameters(count: u32) -> (u32, Vec<u32>, Vec<f64>) {
    let mut rng = rand::thread_rng();
    let mut faces = Vec::with_capacity(count as usize);
    let mut strengths = Vec::with_capacity(count as usize);
    let dest = rng.gen_range(1..=6);
    for _ in 0..count {
        let mut next = rng.gen_range(1..=6);
        while faces.contains(&next) || dest == next {
            next = rng.gen_range(1..=6);
        }
        faces.push(next);
        strengths.push(rng.gen_range(0.5..=1.0));
    }
    (dest, faces, strengths)
}

impl WeightTransfer {
    fn new_single(from: u32, to: u32, strength: f64) -> Self {
        WeightTransfer {
            item_type: ItemType::WeightTransfer,
            transform: WeightTransform::superimpose_pair(to, from, strength),
            short: format!("Weight transfer {} > {}", from, to),
            full: format!(
                "Changes the weights on {1} and {2} to a weighted average favoring {2} at {0:.0}%",
                strength * 100.,
                from,
                to
            ),
        }
    }

    fn random_single() -> Self {
        let (to, mut from, mut strength) = random_transfer_parameters(1);
        let from = from.pop().unwrap();
        let strength = strength.pop().unwrap();
        WeightTransfer::new_single(from, to, strength)
    }

    fn new_double(from1: u32, strength1: f64, from2: u32, strength2: f64, to: u32) -> Self {
        WeightTransfer {
            item_type: ItemType::DoubleWeightTransfer,
            transform: WeightTransform::superimpose_pair(to, from1, strength1)
                .combined_with(&WeightTransform::superimpose_pair(to, from2, strength2)),
            short: format!("Weight transfer {}, {} > {}", from1, from2, to),
            full: format!(
                "Sets the weight on {0} to a weighted average with the weight \
                on {1} and then the weight on {2}, favoring {0} at {3:.0}% and then {4:.0}%",
                to,
                from2,
                from1,
                strength2 * 100.,
                strength1 * 100.
            ),
        }
    }

    fn random_double() -> Self {
        let (to, mut froms, mut strengths) = random_transfer_parameters(2);
        let from1 = froms.pop().unwrap();
        let from2 = froms.pop().unwrap();
        let strength1 = strengths.pop().unwrap();
        let strength2 = strengths.pop().unwrap();
        WeightTransfer::new_double(from1, strength1, from2, strength2, to)
    }

    fn new_pair(
        from1: u32,
        strength1: f64,
        to1: u32,
        from2: u32,
        strength2: f64,
        to2: u32,
    ) -> Self {
        WeightTransfer {
            item_type: ItemType::WeightTransferPair,
            transform: WeightTransform::superimpose_pair(to1, from1, strength1)
                .combined_with(&WeightTransform::superimpose_pair(to2, from2, strength2)),
            short: format!(
                "Weight transfers {} > {} and then {} > {}",
                from2, to2, from1, to1
            ),
            full: format!(
                "Performs two weighed averages: {1} and {2}, \
            favoring {1} at {0:.0}%; then {4} and {5}, favoring {4} at {3:.0}%",
                strength2 * 100.,
                to2,
                from2,
                strength1 * 100.,
                to1,
                from1
            ),
        }
    }

    fn random_pair() -> Self {
        let (to1, mut from1, mut strength1) = random_transfer_parameters(1);
        let from1 = from1.pop().unwrap();
        let strength1 = strength1.pop().unwrap();
        let (to2, mut from2, mut strength2) = random_transfer_parameters(1);
        let from2 = from2.pop().unwrap();
        let strength2 = strength2.pop().unwrap();
        WeightTransfer::new_pair(from1, strength1, to1, from2, strength2, to2)
    }
}

impl Item for WeightTransfer {
    fn short_description(&self) -> &str {
        &self.short
    }

    fn full_description(&self) -> &str {
        &self.full
    }

    fn use_item(&self, player: &mut Player) {
        player.transform_die(&self.transform);
    }

    fn use_item_on_die(&self, die: &mut WeightedDie) {
        die.apply_transformation(&self.transform);
    }

    fn item_type(&self) -> ItemType {
        self.item_type
    }
}
