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
use bevy::app::AppExit;
use bevy::prelude::*;

pub struct MainMenu {
    play_btn: Entity,
    settings_btn: Entity,
    quit_btn: Entity,
    about_btn: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

macro_rules! button_with_text {
    ($commands:ident, $assets:ident, $text:tt) => {
        $commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: NORMAL_BUTTON.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        $text,
                        TextStyle {
                            color: Color::BLACK,
                            font_size: 40.,
                            font: $assets.load("fonts/FiraSans-Bold.ttf"),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
            })
            .id()
    };
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    let play_btn = button_with_text!(commands, asset_server, "Play");
    let settings_btn = button_with_text!(commands, asset_server, "Settings");
    let about_btn = button_with_text!(commands, asset_server, "About");
    let quit_btn = button_with_text!(commands, asset_server, "Quit");
    commands.insert_resource(MainMenu {
        play_btn,
        settings_btn,
        quit_btn,
        about_btn,
    });
}

type ColoredButton<'a> = (Entity, &'a Interaction, &'a mut UiColor);
type ButtonFilter = (Changed<Interaction>, With<Button>);
pub fn main_menu(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<ColoredButton, ButtonFilter>,
    mut app_exit_events: EventWriter<AppExit>,
    menu: Res<MainMenu>,
) {
    for (entity, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                if entity == menu.play_btn {
                    state.set(AppState::Game).unwrap();
                } else if entity == menu.settings_btn {
                    state.set(AppState::Settings).unwrap();
                } else if entity == menu.about_btn {
                    state.set(AppState::About).unwrap();
                } else {
                    app_exit_events.send(AppExit {});
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, menu: Res<MainMenu>) {
    commands.entity(menu.play_btn).despawn_recursive();
    commands.entity(menu.settings_btn).despawn_recursive();
    commands.entity(menu.quit_btn).despawn_recursive();
    commands.entity(menu.about_btn).despawn_recursive();
}
