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

use crate::AppState;
use bevy::prelude::*;
use bevy_egui::egui::emath::Numeric;
use bevy_egui::egui::{Separator, Slider, Ui};
use bevy_egui::{egui, EguiContext};
use std::fmt::Formatter;
use std::slice::Iter;

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerSprite {
    Ferris,
    Darryl,
}

impl std::fmt::Display for PlayerSprite {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PlayerSprite::Ferris => "Ferris",
                PlayerSprite::Darryl => "Darryl",
            }
        )
    }
}

impl PlayerSprite {
    pub fn path(&self) -> &str {
        match self {
            PlayerSprite::Ferris => "sprites/p1.png",
            PlayerSprite::Darryl => "sprites/p2.png",
        }
    }
}

pub struct GameSettings {
    players: u32,
    player_sprites: Vec<PlayerSprite>,
    map_width: usize,
    map_height: usize,
    item_density: f64,
    initial_travel_distance: usize,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            players: 2,
            player_sprites: vec![PlayerSprite::Ferris, PlayerSprite::Darryl],
            map_width: 20,
            map_height: 20,
            item_density: 0.1,
            initial_travel_distance: 10,
        }
    }
}

impl GameSettings {
    pub fn reset_settings(&mut self) {
        *self = GameSettings::default();
    }

    pub fn players(&self) -> u32 {
        self.players
    }

    pub fn player_sprites_iter(&self) -> Iter<'_, PlayerSprite> {
        self.player_sprites.iter()
    }

    pub fn map_width(&self) -> usize {
        self.map_width
    }

    pub fn map_height(&self) -> usize {
        self.map_height
    }

    pub fn item_density(&self) -> f64 {
        self.item_density
    }

    pub fn travel_distance(&self) -> usize {
        self.initial_travel_distance
    }
}

fn number_setting<T>(ui: &mut Ui, num: &mut T, min: T, max: T, lbl: &str)
where
    T: Numeric,
{
    ui.label(lbl);
    let slider = Slider::new(num, min..=max);
    ui.add(slider);
}

pub fn settings_ui(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    mut settings: ResMut<GameSettings>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.heading("Dicey Dungeons: Settings");

        number_setting(ui, &mut settings.players, 2, 6, "Number of players");
        let size = settings.players as usize;
        if size > settings.player_sprites.len() {
            settings.player_sprites.resize(size, PlayerSprite::Ferris);
        }

        for i in 0..settings.players {
            let sprite = &mut settings.player_sprites[i as usize];
            egui::ComboBox::from_label(format!("Player {} sprite", i + 1))
                .selected_text(sprite.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        sprite,
                        PlayerSprite::Ferris,
                        PlayerSprite::Ferris.to_string(),
                    );
                    ui.selectable_value(
                        sprite,
                        PlayerSprite::Darryl,
                        PlayerSprite::Darryl.to_string(),
                    );
                });
        }

        number_setting(ui, &mut settings.map_width, 5, 60, "Map width");
        number_setting(ui, &mut settings.map_height, 5, 60, "Map height");

        let sep = Separator::default().spacing(12.).horizontal();
        ui.add(sep);

        ui.label(
            "All players' starting positions will be connected to the goal by a path of \
         a fixed length before additional paths are generated. This initial distance can be \
         freely chosen.",
        );
        let max_dist = settings.map_height.min(settings.map_width) / 2;
        number_setting(
            ui,
            &mut settings.initial_travel_distance,
            1,
            max_dist,
            "Initial travel distance",
        );

        number_setting(ui, &mut settings.item_density, 0., 0.8, "Item density");

        let sep = Separator::default().spacing(12.).horizontal();
        ui.add(sep);

        if ui.button("Revert to default settings").clicked() {
            settings.reset_settings();
        }

        let sep = Separator::default().spacing(12.).horizontal();
        ui.add(sep);

        if ui.button("Back to Main").clicked() {
            state.set(AppState::MainMenu).unwrap();
        }
    });
}
