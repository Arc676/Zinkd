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
use bevy_egui::{egui, EguiContext};

pub fn about_ui(mut egui_context: ResMut<EguiContext>, mut state: ResMut<State<AppState>>) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.heading("About Dicey Dungeons");

        ui.label("Project by Arc676/Alessandro Vinciguerra and Fatcat590. Created for the first Bevy Jam");

        ui.label("This project is available under the terms of the MIT license or the Apache 2.0 license, at your option. You should have received copies of the licenses with this game. If not, you can find them in the repositories.");
        ui.horizontal(|ui| {
            ui.hyperlink_to("GitHub repository", "https://github.com/Arc676/Dicey-Dungeons");
            ui.hyperlink_to("GitLab repository", "https://gitlab.com/Arc676/dicey-dungeons");
            ui.hyperlink_to("Bevy Jam", "https://itch.io/jam/bevy-jam-1");
        });

        ui.add(egui::Separator::default().horizontal());

        ui.label(include_str!("../licenses/CREDITS"));

        ui.add(egui::Separator::default().horizontal());

        if ui.button("Back to Main").clicked() {
            state.set(AppState::MainMenu).unwrap();
        }
    });
}
