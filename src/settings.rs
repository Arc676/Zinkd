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
use directories_next::ProjectDirs;
use ron;
use serde;
use std::fmt::Formatter;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::slice::Iter;

#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct GameSettings {
    players: u32,
    player_sprites: Vec<PlayerSprite>,
    player_names: Vec<String>,
    map_width: usize,
    map_height: usize,
    item_density: f64,
    initial_travel_distance: usize,
    default_zoom_level: f32,
    walking_speed: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            players: 2,
            player_sprites: vec![PlayerSprite::Ferris, PlayerSprite::Darryl],
            player_names: vec!["Ferris".to_string(), "Darryl".to_string()],
            map_width: 60,
            map_height: 60,
            item_density: 0.1,
            initial_travel_distance: 40,
            default_zoom_level: 0.7,
            walking_speed: 2.,
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

    pub fn player_names_iter(&self) -> Iter<'_, String> {
        self.player_names.iter()
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

    pub fn default_zoom_level(&self) -> f32 {
        self.default_zoom_level
    }

    pub fn walking_speed(&self) -> f32 {
        self.walking_speed
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
        ui.heading("Zink'd: Settings");

        number_setting(ui, &mut settings.players, 2, 6, "Number of players");
        let size = settings.players as usize;
        if size > settings.player_sprites.len() {
            settings.player_sprites.resize(size, PlayerSprite::Ferris);
            settings.player_names.resize(size, "New Player".to_string());
        }

        ui.label("Choose player names and sprites");
        for i in 0..settings.players {
            ui.horizontal(|ui| {
                ui.label(format!("Player {}:", i + 1));
                ui.text_edit_singleline(&mut settings.player_names[i as usize]);
                let sprite = &mut settings.player_sprites[i as usize];
                egui::ComboBox::from_id_source(format!("sprite_picker_{}", i))
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
            });
        }

        let sep = Separator::default().spacing(12.).horizontal();
        ui.add(sep);

        number_setting(ui, &mut settings.map_width, 20, 120, "Map width");
        number_setting(ui, &mut settings.map_height, 20, 120, "Map height");

        ui.label(
            "All players' starting positions will be connected to the goal by a path of \
         a fixed length before additional paths are generated. This initial distance can be \
         freely chosen.",
        );
        let max_dist = settings.map_height.min(settings.map_width) * 3 / 4;
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

        number_setting(
            ui,
            &mut settings.walking_speed,
            1.,
            10.,
            "Walking speed (tiles per second)",
        );

        number_setting(
            ui,
            &mut settings.default_zoom_level,
            0.05,
            5.,
            "Default camera zoom level (higher is more zoomed out)",
        );

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

pub fn load_settings(mut settings: ResMut<GameSettings>) {
    #[cfg(feature = "serde")]
    if let Some(dir) = ProjectDirs::from("", "", "Zink'd") {
        let mut file = dir.config_dir().to_path_buf();
        file.push("settings.ron");
        let file = File::open(file);
        if let Ok(mut file) = file {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read settings file");
            if let Ok(existing) = ron::from_str(contents.as_str()) {
                *settings = existing;
            }
        }
    }
}

pub fn save_settings(settings: Res<GameSettings>) {
    #[cfg(feature = "serde")]
    if let Some(dir) = ProjectDirs::from("", "", "Zink'd") {
        let mut file = dir.config_dir().to_path_buf();
        create_dir_all(&file).expect("Failed to create config directory");
        file.push("settings.ron");
        let mut file = File::create(file).expect("Failed to create settings file");
        file.write(ron::to_string(&*settings).unwrap().as_ref())
            .expect("Failed to write settings to disk");
    }
}
