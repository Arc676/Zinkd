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

pub type HeldItem = Box<dyn Item>;
pub type PossibleItem = Option<HeldItem>;

pub trait Item: Send + Sync {
    fn short_description(&self) -> &str;
    fn full_description(&self) -> &str;
    fn use_item(&self, player: &mut Player, die: &mut WeightedDie);
}

pub enum ItemType {
    WeightTransfer,
    DoubleWeightTransfer,
    WeightTransferPair,
}

pub struct WeightTransfer {
    transform: WeightTransform,
    short: String,
    full: String,
}

impl WeightTransfer {
    pub fn new(from: u32, to: u32, strength: f64) -> Self {
        WeightTransfer {
            transform: WeightTransform::superimpose_pair(to, from, strength),
            short: format!("Weight transfer {} > {}", from, to),
            full: format!(
                "Changes the weights on {1}, {2} to a weighted average favoring {2} at {0:0}%",
                strength * 100.,
                from,
                to
            ),
        }
    }
}

impl Item for WeightTransfer {
    fn short_description(&self) -> &str {
        &self.short
    }

    fn full_description(&self) -> &str {
        &self.full
    }

    fn use_item(&self, _player: &mut Player, die: &mut WeightedDie) {
        die.apply_transformation(&self.transform);
    }
}
