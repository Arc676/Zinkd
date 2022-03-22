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

use crate::map::{Coordinates, Direction, GridCell, Map, EAST, NORTH, SOUTH, WEST};
use crate::player::Player;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum MoveAlgorithm {
    ShortestPath,
}

#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ItemAlgorithm {
    HighestGain,
}

impl Display for MoveAlgorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveAlgorithm::ShortestPath => write!(f, "Shortest Path"),
        }
    }
}

impl Display for ItemAlgorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemAlgorithm::HighestGain => write!(f, "Highest gain"),
        }
    }
}

impl MoveAlgorithm {
    pub fn compute_move(&self, start: Coordinates, map: &Map) -> Direction {
        match self {
            MoveAlgorithm::ShortestPath => shortest_path(start, map),
        }
    }
}

impl ItemAlgorithm {
    pub fn choose_item(&self, user: &Player, players: &[Player]) -> Option<(usize, usize)> {
        match self {
            ItemAlgorithm::HighestGain => highest_self_benefit(user, players),
        }
    }
}

// Item computations
pub fn highest_self_benefit(user: &Player, _players: &[Player]) -> Option<(usize, usize)> {
    let mut best_item = None;
    let mut max_gain = 0.;
    for (i, item) in user.items().enumerate() {
        let benefit = item.item_benefit(user);
        if benefit > max_gain {
            max_gain = benefit;
            best_item = Some(i);
        }
    }
    match best_item {
        None => None,
        Some(idx) => Some((idx, user.player_number())),
    }
}

// Path computations
fn shortest_path(start: Coordinates, map: &Map) -> Direction {
    let mut min_distance = usize::MAX;
    let mut best_direction = 0;
    let exits = match map.cell_at(start) {
        GridCell::Wall => panic!("Cannot navigate from inside a wall"),
        GridCell::Path(directions, _) => *directions,
        GridCell::Goal(_) => 0,
    };
    for direction in [NORTH, EAST, SOUTH, WEST] {
        if exits & direction != 0 {
            let mut cell = start.clone();
            cell.step(direction, map.width(), map.height());
            let distance = map.distance_to_goal(cell).unwrap();
            if distance < min_distance {
                min_distance = distance;
                best_direction = direction;
            }
        }
    }
    best_direction
}
