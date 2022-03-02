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

use std::cmp::min;
use crate::settings::GameSettings;
use crate::AppState;
use bevy::prelude::*;
use dicey_dungeons::map::*;

pub struct DungeonGame {
    map: Map,
}

pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
    window: Res<Windows>,
) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
    let map = Map::generate_random_map(
        settings.map_width(),
        settings.map_height(),
        settings.players(),
        settings.travel_distance(),
    );

    let window = window.get_primary().unwrap();
    let width = window.width() as i32;
    let tile_width = width / settings.map_width() as i32;
    let height = window.height() as i32;
    let tile_height = height / settings.map_height() as i32;

    let tile_size = (min(tile_width, tile_height) / 24) * 24;
    let tile_size = Vec2::splat(tile_size as f32);

    let offset = Vec2::new(settings.map_width() as f32 / 2. - 0.5, settings.map_height() as f32 / 2. - 0.5) * tile_size;

    let longitudinal = asset_server.load("tiles/tile_straight.png");
    let latitudinal = asset_server.load("tiles/tile_straight_h.png");
    let omnidirectional = asset_server.load("tiles/tile_cross2.png");
    let wall = asset_server.load("tiles/tile_wall.png");
    let goal = asset_server.load("sprites/goal.png");

    let mut sprites = vec![];
    for (Coordinates(x, y), cell) in map.iter() {
        let texture = match cell {
            GridCell::Wall => wall.clone(),
            GridCell::Path(direction, _) => match *direction {
                OMNIDIRECTIONAL => omnidirectional.clone(),
                LONGITUDINAL => longitudinal.clone(),
                LATITUDINAL => latitudinal.clone(),
                _ => panic!("Unknown direction"),
            },
            GridCell::Goal => goal.clone(),
        };
        let translation = (Vec2::new(x as f32, y as f32) * tile_size - offset).extend(0.);
        sprites.push(SpriteBundle {
            texture,
            transform: Transform {
                translation,
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(tile_size),
                ..Default::default()
            },
            ..Default::default()
        });
    }
    commands.spawn_batch(sprites);
    let game = DungeonGame { map };
    commands.insert_resource(game);
}

pub fn update_game(mut state: ResMut<State<AppState>>) {
}

pub fn cleanup_game(mut commands: Commands) {
    commands.remove_resource::<DungeonGame>()
}
