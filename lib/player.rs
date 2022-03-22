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
use crate::items::{HeldItem, ItemType};
use crate::map::{Coordinates, Direction, GridCell, Map};
use crate::npc::{ItemAlgorithm, MoveAlgorithm};
use bevy::ecs::component::Component;
use std::fmt::{Display, Formatter};
use std::slice::Iter;

#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PlayerType {
    LocalHuman,
    Computer(MoveAlgorithm, ItemAlgorithm),
}

impl Display for PlayerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerType::LocalHuman => write!(f, "Human"),
            PlayerType::Computer(mv, it) => write!(f, "Computer ({}, {})", mv, it),
        }
    }
}

#[derive(Component)]
pub struct Player {
    name: String,
    position: Coordinates,
    inventory: Vec<HeldItem>,
    die: WeightedDie,
    player_number: usize,
    ptype: PlayerType,
    moves: Vec<Direction>,
}

impl Player {
    pub fn spawn_at(
        position: Coordinates,
        name: String,
        player_number: usize,
        ptype: PlayerType,
    ) -> Self {
        Player {
            name,
            position,
            inventory: vec![],
            die: WeightedDie::fair_die(),
            player_number,
            ptype,
            moves: vec![],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> PlayerType {
        self.ptype
    }

    pub fn player_number(&self) -> usize {
        self.player_number
    }

    pub fn pick_up(&mut self, item: HeldItem) {
        self.inventory.push(item);
    }

    pub fn position(&self) -> Coordinates {
        self.position
    }

    pub fn step(&mut self, direction: Direction, map: &Map) -> bool {
        let mut current = self.position;
        match map.cell_at(current) {
            GridCell::Wall => panic!("Somehow the player is in a wall"),
            GridCell::Path(exits, _) => {
                if direction & exits == 0 {
                    return false;
                }
            }
            GridCell::Goal(_) => {}
        }
        if !current.step(direction, map.width(), map.height()) {
            return false;
        }
        match map.cell_at(current) {
            GridCell::Wall => panic!("Path allowed walking into a wall"),
            _ => {
                self.position = current;
                true
            }
        }
    }

    pub fn inventory_empty(&self) -> bool {
        self.inventory.is_empty()
    }

    pub fn items(&self) -> Iter<'_, HeldItem> {
        self.inventory.iter()
    }

    pub fn take_item(&mut self, index: usize) -> HeldItem {
        debug_assert!(index < self.inventory.len());
        self.inventory.remove(index)
    }

    pub fn use_item_on_die(&self, die: &mut WeightedDie, index: usize) {
        debug_assert!(index < self.inventory.len());
        self.inventory[index].use_item_on_die(die);
    }

    pub fn get_item_type(&self, index: usize) -> ItemType {
        debug_assert!(index < self.inventory.len());
        self.inventory[index].item_type()
    }

    pub fn transform_die(&mut self, transform: &WeightTransform) {
        self.die.apply_transformation(transform);
    }

    pub fn die(&self) -> &WeightedDie {
        &self.die
    }

    pub fn roll(&self) -> u32 {
        self.die.roll()
    }

    pub fn append_move(&mut self, direction: Direction) {
        self.moves.push(direction);
    }

    pub fn last_move(&self) -> Direction {
        *self.moves.last().unwrap_or(&0)
    }

    pub fn end_turn(&mut self) {
        self.moves.clear();
    }
}
