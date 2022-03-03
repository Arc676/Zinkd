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

use crate::settings::GameSettings;
use crate::AppState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use dicey_dungeons::map::*;
use dicey_dungeons::player::Player;
use std::cmp::min;

#[derive(Default)]
pub struct GameState {
    paused: bool,
    active_player: u32,
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

    let offset = Vec2::new(
        settings.map_width() as f32 / 2. - 0.5,
        settings.map_height() as f32 / 2. - 0.5,
    ) * tile_size;
    let coords_to_vec =
        |x: usize, y: usize, z: f32| (Vec2::new(x as f32, y as f32) * tile_size - offset).extend(z);

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
        let translation = coords_to_vec(x, y, 0.);
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

    for (num, (sprite, spawn_pos)) in settings
        .player_sprites_iter()
        .zip(map.starting_positions())
        .enumerate()
    {
        let Coordinates(x, y) = spawn_pos;
        let player = Player::spawn_at(*spawn_pos, num as u32);

        let texture = asset_server.load(sprite.path());
        let translation = coords_to_vec(*x, *y, 1.);

        commands
            .spawn_bundle(SpriteBundle {
                texture,
                transform: Transform {
                    translation,
                    ..Default::default()
                },
                sprite: Sprite {
                    custom_size: Some(tile_size / 2.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(player);
    }
    commands.insert_resource(map);
    commands.insert_resource(GameState::default());
}

pub fn update_game(mut game_state: ResMut<GameState>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_released(KeyCode::Escape) {
        game_state.paused = !game_state.paused;
    }
}

pub fn pause_menu(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    game_state: Res<GameState>,
) {
    if !game_state.paused {
        return;
    }
    egui::Window::new("Pause").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Back to Main").clicked() {
            state.set(AppState::MainMenu).unwrap();
        }
    });
}

pub fn cleanup_game(mut commands: Commands, sprite_query: Query<Entity, With<Sprite>>) {
    commands.remove_resource::<Map>();
    for sprite in sprite_query.iter() {
        commands.entity(sprite).despawn();
    }
}
