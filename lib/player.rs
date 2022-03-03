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
use crate::items::HeldItem;
use crate::map::{Coordinates, Direction, GridCell, Map};
use bevy::ecs::component::Component;
use std::slice::Iter;

#[derive(Component)]
pub struct Player {
    position: Coordinates,
    inventory: Vec<HeldItem>,
    die: WeightedDie,
    player_number: u32,
}

impl Player {
    pub fn spawn_at(position: Coordinates, player_number: u32) -> Self {
        Player {
            position,
            inventory: vec![],
            die: WeightedDie::fair_die(),
            player_number,
        }
    }

    pub fn player_number(&self) -> u32 {
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
        if !current.step(direction, map.width(), map.height()) {
            return false;
        }
        match map.cell_at(current) {
            GridCell::Wall => false,
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

    pub fn use_item(&mut self, index: usize) {
        debug_assert!(index < self.inventory.len());
        let item = self.inventory.remove(index);
        item.use_item(self);
    }

    pub fn transform_die(&mut self, transform: &WeightTransform) {
        self.die.apply_transformation(transform);
    }

    pub fn roll(&self) -> u32 {
        self.die.roll()
    }
}
