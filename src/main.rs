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

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod main_menu;
mod game;
mod settings;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Game,
    Settings,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_state(AppState::MainMenu)
        .insert_resource(settings::GameSettings::default())
        .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(main_menu::setup_menu))
        .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(main_menu::main_menu))
        .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(main_menu::cleanup_menu))
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(game::setup_game))
        .add_system_set(SystemSet::on_update(AppState::Game).with_system(game::update_game))
        .add_system_set(SystemSet::on_update(AppState::Game).with_system(game::update_die))
        .add_system_set(SystemSet::on_update(AppState::Game).with_system(game::pause_menu))
        .add_system_set(SystemSet::on_exit(AppState::Game).with_system(game::cleanup_game))
        .add_system_set(SystemSet::on_update(AppState::Settings).with_system(settings::settings_ui))
        .run();
}
