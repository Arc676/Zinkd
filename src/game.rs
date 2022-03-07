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
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::{ecs::component::Component, input::mouse::MouseWheel};
use bevy_egui::{egui, EguiContext};
use dicey_dungeons::dice::WeightedDie;
use dicey_dungeons::items::ItemType;
use dicey_dungeons::map::Direction;
use dicey_dungeons::map::*;
use dicey_dungeons::player::Player;
use std::f32::consts::{FRAC_PI_2, PI};

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct EntityTooltip(String);

enum GameAction {
    WaitForInput,
    UsingItem,
    Moving(Direction, u32),
    HasMoved,
}

impl Default for GameAction {
    fn default() -> Self {
        GameAction::WaitForInput
    }
}

enum ItemEffect {
    DieTransform(WeightedDie, WeightedDie),
    PlayerAction(String),
}

enum ItemAction {
    NoAction,
    UseItem,
    CancelItem,
}

#[derive(Default)]
struct ItemUsePreview {
    source_player: u32,
    item_index: usize,
    item_type: ItemType,
    target_player: u32,
    effect: Option<ItemEffect>,
}

#[derive(Default)]
pub struct GameState {
    player_count: u32,
    paused: bool,
    active_player: u32,
    player_names: Vec<String>,
    current_action: GameAction,
    hover_item: Option<String>,
    item_preview: ItemUsePreview,
    inventory_visible: bool,
    picked_up_item: Option<String>,
    rolled_value: Option<u32>,
    winners: Vec<u32>,
    winner_names: Vec<String>,
    game_over: bool,
    camera_follows_player: bool,
    camera_auto_zoom: bool,
}

impl GameState {
    fn get_player_name(&self, player: u32, active: u32) -> &str {
        if player == active {
            "yourself"
        } else {
            &self.player_names[player as usize]
        }
    }
}

enum Control {
    Roll,
    Inventory,
    Move(Direction),
    EndTurn,
}

pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    let map = Map::generate_random_map(
        settings.map_width(),
        settings.map_height(),
        settings.players(),
        settings.item_density(),
        settings.travel_distance(),
    );

    let tile_size = Vec2::splat(96.);
    let coords_to_vec =
        |x: usize, y: usize, z: f32| Vec2::new(x as f32 * 96., y as f32 * 96.).extend(z);

    let straight = asset_server.load("tiles/tile_straight.png");
    let dead_end = asset_server.load("tiles/tile_dead_end.png");
    let corner = asset_server.load("tiles/tile_corner.png");
    let t_intersection = asset_server.load("tiles/tile_cross1.png");
    let omnidirectional = asset_server.load("tiles/tile_cross2.png");
    let wall = asset_server.load("tiles/tile_wall.png");
    let goal = asset_server.load("sprites/goal.png");

    let item_sprite = asset_server.load("sprites/item_weight.png");

    let mut sprites = vec![];
    for (Coordinates(x, y), cell) in map.iter() {
        let mut rotation = Quat::IDENTITY;
        let texture = match cell {
            GridCell::Wall => wall.clone(),
            GridCell::Path(direction, item) => {
                if let Some(item) = item {
                    let translation = coords_to_vec(x, y, 0.5);
                    commands
                        .spawn_bundle(SpriteBundle {
                            texture: item_sprite.clone(),
                            transform: Transform {
                                translation,
                                ..Default::default()
                            },
                            sprite: Sprite {
                                custom_size: Some(tile_size),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(EntityTooltip(item.short_description().to_string()));
                }
                match *direction {
                    OMNIDIRECTIONAL => omnidirectional.clone(),
                    LONGITUDINAL | LATITUDINAL => {
                        if *direction == LATITUDINAL {
                            rotation = Quat::from_rotation_z(FRAC_PI_2);
                        }
                        straight.clone()
                    }
                    NORTH | EAST | SOUTH | WEST => {
                        match *direction {
                            NORTH => rotation = Quat::from_rotation_z(PI),
                            EAST => rotation = Quat::from_rotation_z(FRAC_PI_2),
                            WEST => rotation = Quat::from_rotation_z(-FRAC_PI_2),
                            _ => (),
                        }
                        dead_end.clone()
                    }
                    NOT_NORTH | NOT_EAST | NOT_SOUTH | NOT_WEST => {
                        match *direction {
                            NOT_NORTH => rotation = Quat::from_rotation_z(-FRAC_PI_2),
                            NOT_SOUTH => rotation = Quat::from_rotation_z(FRAC_PI_2),
                            NOT_EAST => rotation = Quat::from_rotation_z(PI),
                            _ => (),
                        }
                        t_intersection.clone()
                    }
                    NORTHEAST | NORTHWEST | SOUTHEAST | SOUTHWEST => {
                        match *direction {
                            NORTHEAST => rotation = Quat::from_rotation_z(PI),
                            NORTHWEST => rotation = Quat::from_rotation_z(-FRAC_PI_2),
                            SOUTHEAST => rotation = Quat::from_rotation_z(FRAC_PI_2),
                            _ => (),
                        }
                        corner.clone()
                    }
                    _ => {
                        if cfg!(debug_assertions) {
                            dbg!("Unknown direction {}", direction);
                            rotation = Quat::from_rotation_z(PI);
                            goal.clone()
                        } else {
                            wall.clone()
                        }
                    }
                }
            }
            GridCell::Goal => goal.clone(),
        };
        let translation = coords_to_vec(x, y, 0.);
        sprites.push(SpriteBundle {
            texture,
            transform: Transform {
                translation,
                rotation,
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

    let mut player_names = vec![];
    for (num, ((sprite, name), spawn_pos)) in settings
        .player_sprites_iter()
        .zip(settings.player_names_iter())
        .zip(map.starting_positions())
        .enumerate()
    {
        let Coordinates(x, y) = spawn_pos;
        player_names.push(name.clone());
        let player = Player::spawn_at(*spawn_pos, name.clone(), num as u32);

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
            .insert(EntityTooltip(name.clone()))
            .insert(player);
    }
    commands.insert_resource(map);

    let texture = asset_server.load("sprites/DieFaces.png");
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::splat(32.), 6, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);
    // let translation = Vec2::new(width as f32 / 2. - 20., height as f32 / 2. - 20.).extend(0.);
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas,
        // transform: Transform {
        //     translation,
        //     ..Default::default()
        // },
        visibility: Visibility { is_visible: false },
        ..Default::default()
    });

    commands.insert_resource(GameState {
        player_count: settings.players(),
        player_names,
        camera_follows_player: true,
        camera_auto_zoom: true,
        ..Default::default()
    });
}

fn get_control(keyboard: &Res<Input<KeyCode>>) -> Option<Control> {
    if keyboard.just_released(KeyCode::R) {
        return Some(Control::Roll);
    }
    if keyboard.just_released(KeyCode::E) {
        return Some(Control::Inventory);
    }
    if keyboard.just_released(KeyCode::W) {
        return Some(Control::Move(NORTH));
    }
    if keyboard.just_released(KeyCode::A) {
        return Some(Control::Move(WEST));
    }
    if keyboard.just_released(KeyCode::S) {
        return Some(Control::Move(SOUTH));
    }
    if keyboard.just_released(KeyCode::D) {
        return Some(Control::Move(EAST));
    }
    if keyboard.just_released(KeyCode::Return) {
        return Some(Control::EndTurn);
    }
    None
}

fn end_turn(game_state: &mut ResMut<GameState>) {
    game_state.rolled_value = None;
    game_state.inventory_visible = false;
    loop {
        game_state.active_player = (game_state.active_player + 1) % game_state.player_count;
        if !game_state.winners.contains(&game_state.active_player) {
            break;
        }
    }
    game_state.current_action = GameAction::WaitForInput;
    game_state.item_preview = ItemUsePreview::default();
    game_state.hover_item = None;
    game_state.picked_up_item = None;
}

pub fn update_die(
    game_state: Res<GameState>,
    mut query: Query<(&mut Visibility, &mut TextureAtlasSprite)>,
) {
    for (mut visibility, mut sprite) in query.iter_mut() {
        match game_state.rolled_value {
            None => visibility.is_visible = false,
            Some(value) => {
                visibility.is_visible = true;
                sprite.index = value as usize - 1;
            }
        }
    }
}

pub fn entity_tooltips(
    mut game_state: ResMut<GameState>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    item_query: Query<(&GlobalTransform, &EntityTooltip)>,
) {
    // https://bevy-cheatbook.github.io/cookbook/cursor2world.html
    let (camera, camera_transform) = camera_query.single();

    let threshold = 96.0 / 2.0f32.sqrt();

    let wnd = windows.get(camera.window).unwrap();

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();

        for (transform, EntityTooltip(description)) in item_query.iter() {
            if world_pos.distance(transform.translation.truncate()) < threshold {
                game_state.hover_item = Some(description.clone());
                return;
            }
        }
    }
    game_state.hover_item = None;
}

pub fn update_game(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    keyboard: Res<Input<KeyCode>>,
    mut map: ResMut<Map>,
    mut player_query: Query<(&mut Player, &mut Transform, &mut Sprite)>,
    item_query: Query<(Entity, &Transform, &EntityTooltip), Without<Player>>,
) {
    if keyboard.just_released(KeyCode::Escape) {
        game_state.paused = !game_state.paused;
    }
    for (mut player, mut transform, mut sprite) in player_query.iter_mut() {
        if game_state.active_player == player.player_number() {
            match game_state.current_action {
                GameAction::WaitForInput => {
                    if let Some(action) = get_control(&keyboard) {
                        match action {
                            Control::Roll => {
                                let rolled = player.roll();
                                game_state.rolled_value = Some(rolled);
                                game_state.current_action = GameAction::Moving(0, rolled);
                            }
                            Control::Inventory => {
                                game_state.inventory_visible = !game_state.inventory_visible
                            }
                            _ => (),
                        }
                    }
                }
                GameAction::UsingItem => {}
                GameAction::Moving(_, remaining) => {
                    if let Some(Control::Move(step)) = get_control(&keyboard) {
                        if player.step(step, &map) {
                            let position = player.position();
                            let Coordinates(x, y) = position;
                            transform.translation =
                                Vec2::new(x as f32 * 96., y as f32 * 96.).extend(1.);
                            sprite.flip_x = step == WEST;
                            match map.cell_at_mut(position) {
                                GridCell::Path(_, item) if item.is_some() => {
                                    let item = item.take().unwrap();
                                    game_state.picked_up_item =
                                        Some(item.short_description().to_string());
                                    player.pick_up(item);
                                    for (entity, item_transform, _) in item_query.iter() {
                                        if item_transform.translation.truncate()
                                            == transform.translation.truncate()
                                        {
                                            commands.entity(entity).despawn();
                                            break;
                                        }
                                    }
                                }
                                GridCell::Goal => {
                                    game_state.winners.push(player.player_number());
                                    game_state.winner_names.push(player.name().to_string());
                                    game_state.current_action = GameAction::HasMoved;
                                    break;
                                }
                                _ => (),
                            }
                            let mut step_count = remaining;
                            step_count -= 1;
                            if step_count == 0 {
                                game_state.current_action = GameAction::HasMoved;
                            } else {
                                game_state.current_action = GameAction::Moving(step, step_count);
                            }
                        }
                    }
                }
                GameAction::HasMoved => {
                    if let Some(action) = get_control(&keyboard) {
                        match action {
                            Control::Inventory => {
                                game_state.inventory_visible = !game_state.inventory_visible
                            }
                            Control::EndTurn => {
                                if game_state.winners.len() == game_state.player_count as usize - 1
                                {
                                    game_state.game_over = true;
                                } else {
                                    end_turn(&mut game_state)
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }
}

pub fn scroll_game(
    mut whl: EventReader<MouseWheel>,
    mut cam: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
    windows: Res<Windows>,
    input_mouse: Res<Input<MouseButton>>,
    mut prev: Local<Option<Vec2>>,
    mut game_state: ResMut<GameState>,
    player_query: Query<(&Transform, &Player), Without<MainCamera>>,
) {
    let mut tr = Vec2::ZERO;

    let delta_zoom: f32 = whl.iter().map(|e| e.y).sum();
    let (mut pos, mut cam) = cam.single_mut();
    let window = windows.get_primary().unwrap();
    let cursor_position = match window.cursor_position() {
        Some(x) => x,
        None => return,
    };

    if input_mouse.pressed(MouseButton::Left) && !input_mouse.just_pressed(MouseButton::Left) {
        tr = cursor_position - prev.unwrap_or(cursor_position);
    }

    if delta_zoom != 0. {
        let window_size = Vec2::new(window.width(), window.height());
        let mouse_normalized_screen_pos = (cursor_position / window_size) * 2. - Vec2::ONE;
        let mouse_world_pos = pos.translation.truncate()
            + mouse_normalized_screen_pos * Vec2::new(cam.right, cam.top) * cam.scale;

        cam.scale -= 0.05 * delta_zoom * cam.scale;
        cam.scale = cam.scale.clamp(0.05, 10.0);

        pos.translation = (mouse_world_pos
            - mouse_normalized_screen_pos * Vec2::new(cam.right, cam.top) * cam.scale)
            .extend(pos.translation.z);

        game_state.camera_auto_zoom = false;
    }
    if tr.length_squared() > 0.0 {
        let s = Vec2::new(
            window.width() / (cam.right - cam.left),
            window.height() / (cam.top - cam.bottom),
        ) * cam.scale;
        pos.translation -= (tr * s).extend(0.);
        game_state.camera_follows_player = false;
    }

    if game_state.camera_follows_player {
        for (transform, player) in player_query.iter() {
            if player.player_number() == game_state.active_player {
                pos.translation = Vec3::new(
                    transform.translation.x,
                    transform.translation.y,
                    pos.translation.z,
                );
                break;
            }
        }
    }
    if game_state.camera_auto_zoom {
        cam.scale = 0.7;
    }
    *prev = Some(cursor_position);
}

pub fn game_ui(mut game_state: ResMut<GameState>, mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Control Panel").show(egui_context.ctx_mut(), |ui| {
        if game_state.game_over {
            ui.heading("Game over!");
            ui.label("Leaderboard:");
            for (place, winner) in game_state.winner_names.iter().enumerate() {
                ui.label(format!("{}: {}", place + 1, winner));
            }
            return;
        }
        ui.heading(format!(
            "{}'s turn",
            game_state.player_names[game_state.active_player as usize]
        ));
        match game_state.current_action {
            GameAction::WaitForInput => {
                ui.label("Press R to roll");
                ui.label(
                    "Press E to view your inventory (note that you cannot use items at this time)",
                );
            }
            GameAction::UsingItem => {
                ui.label("Consult the item preview to see what the item will do.");
                ui.label("Click confirm to use the item.");
            }
            GameAction::Moving(_, remaining) => {
                ui.label("Use WASD to move");
                ui.label(format!("{} steps remaining", remaining));
                if let Some(description) = &game_state.picked_up_item {
                    ui.label(format!("You picked up an item: {}", description));
                }
            }
            GameAction::HasMoved => {
                if game_state.winners.contains(&game_state.active_player) {
                    ui.label("You have reached the goal!");
                } else {
                    if let Some(description) = &game_state.picked_up_item {
                        ui.label(format!("You picked up an item: {}", description));
                    }
                    ui.label("Press E to view your inventory (you may now use items)");
                }
                ui.label("Press Enter to end your turn");
            }
        }

        let sep = egui::Separator::default().spacing(12.).horizontal();
        ui.add(sep);

        if let Some(description) = &game_state.hover_item {
            ui.label(description);
        } else {
            ui.label("Hover over an item to see its description");
        }

        let sep = egui::Separator::default().spacing(12.).horizontal();
        ui.add(sep);

        ui.label("Drag to pan the camera");
        ui.checkbox(
            &mut game_state.camera_follows_player,
            "Camera follows current player",
        );
        ui.label("Scroll to zoom in or out");
        ui.checkbox(
            &mut game_state.camera_auto_zoom,
            "Automatically set camera zoom level",
        );
    });
}

fn get_player_with_number<'a>(number: u32, query: &'a mut Query<&mut Player>) -> Mut<'a, Player> {
    for player in query.iter_mut() {
        if player.player_number() == number {
            return player;
        }
    }
    panic!("No player with given number");
}

fn get_painter(ui: &mut egui::Ui) -> (egui::Painter, egui::emath::RectTransform) {
    use bevy_egui::egui::*;
    let (response, painter) = ui.allocate_painter(ui.available_size_before_wrap(), Sense::click());
    let to_screen = emath::RectTransform::from_to(
        Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
        response.rect,
    );
    (painter, to_screen)
}

fn die_weight_labels(painter: &egui::Painter, to_screen: egui::emath::RectTransform) {
    use bevy_egui::egui::*;
    for face in 1..=6 {
        painter.text(
            to_screen
                * Pos2 {
                    x: face as f32 / 7.,
                    y: 0.1,
                },
            Align2::CENTER_CENTER,
            face,
            TextStyle::Body,
            Color32::WHITE,
        );
    }
}

fn item_preview(
    egui_context: &mut ResMut<EguiContext>,
    query: &mut Query<&mut Player>,
    game_state: &mut ResMut<GameState>,
) -> ItemAction {
    let mut chosen_action = ItemAction::NoAction;
    let target_name = game_state
        .get_player_name(
            game_state.item_preview.target_player,
            game_state.item_preview.source_player,
        )
        .to_string();
    let item_preview = &mut game_state.item_preview;
    if item_preview.effect.is_none() {
        match item_preview.item_type {
            _ => {
                let (die_before, mut die_after) = {
                    let target_player = get_player_with_number(item_preview.target_player, query);
                    let die_before = target_player.die().clone();
                    let die_after = die_before.clone();
                    (die_before, die_after)
                };
                let user = get_player_with_number(item_preview.source_player, query);
                user.use_item_on_die(&mut die_after, item_preview.item_index);
                item_preview.effect = Some(ItemEffect::DieTransform(die_before, die_after));
            }
        }
    }
    egui::Window::new("Item Effect").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label(format!(
                "Use {} item on {}?",
                item_preview.item_type, target_name
            ));
            if ui.button("Confirm").clicked() {
                let item = {
                    let mut user = get_player_with_number(item_preview.source_player, query);
                    user.take_item(item_preview.item_index)
                };
                let mut target = get_player_with_number(item_preview.target_player, query);
                item.use_item(&mut target);
                chosen_action = ItemAction::UseItem;
            }
            if ui.button("Cancel").clicked() {
                chosen_action = ItemAction::CancelItem;
            }
        });
        match item_preview.effect.as_ref().unwrap() {
            ItemEffect::DieTransform(before, after) => {
                ui.label("Lost weight in red. Gained weight in green. Yellow sections unchanged.");
                let (painter, to_screen) = get_painter(ui);
                die_weight_labels(&painter, to_screen);
                before.visualize_weights(
                    &painter,
                    to_screen,
                    egui::Color32::from_rgba_unmultiplied(255, 0, 0, 128),
                );
                after.visualize_weights(
                    &painter,
                    to_screen,
                    egui::Color32::from_rgba_unmultiplied(0, 255, 0, 128),
                );
            }
            ItemEffect::PlayerAction(effect) => {
                ui.label(effect);
            }
        }
    });
    chosen_action
}

fn die_inspector(
    egui_context: &mut ResMut<EguiContext>,
    query: &mut Query<&mut Player>,
    game_state: &mut ResMut<GameState>,
) {
    let player = get_player_with_number(game_state.active_player, query);
    egui::Window::new("Die Inspector").show(egui_context.ctx_mut(), |ui| {
        ui.heading(format!("Die weights for {}", player.name()));
        let (painter, to_screen) = get_painter(ui);
        die_weight_labels(&painter, to_screen);
        player
            .die()
            .visualize_weights(&painter, to_screen, egui::Color32::BLUE);
    });
}

fn inventory_window(
    egui_context: &mut ResMut<EguiContext>,
    query: &mut Query<&mut Player>,
    game_state: &mut ResMut<GameState>,
) {
    let player = get_player_with_number(game_state.active_player, query);
    egui::Window::new("Inventory").show(egui_context.ctx_mut(), |ui| {
        ui.heading(format!("{}'s inventory", player.name()));
        if player.inventory_empty() {
            ui.label("No items");
            return;
        }
        let mut used = None;
        for (i, item) in player.items().enumerate() {
            ui.horizontal(|ui| {
                ui.collapsing(format!("{}: {}", i, item.short_description()), |ui| {
                    ui.label(item.full_description());
                    ui.horizontal(|ui| {
                        ui.label("Use this on");
                        egui::ComboBox::from_id_source(format!("target_picker_{}", i))
                            .selected_text(game_state.get_player_name(
                                game_state.item_preview.target_player,
                                player.player_number(),
                            ))
                            .show_ui(ui, |ui| {
                                for num in 0..game_state.player_count {
                                    let name = game_state
                                        .get_player_name(num, player.player_number())
                                        .to_string();
                                    ui.selectable_value(
                                        &mut game_state.item_preview.target_player,
                                        num,
                                        name,
                                    );
                                }
                            });
                    });
                    if let GameAction::HasMoved = game_state.current_action {
                        ui.horizontal(|ui| {
                            if ui.button("Use item...").clicked() {
                                used = Some(i);
                            }
                        });
                    }
                });
            });
        }
        if let Some(item_index) = used {
            game_state.item_preview = ItemUsePreview {
                source_player: player.player_number(),
                item_type: player.get_item_type(item_index),
                item_index,
                target_player: game_state.item_preview.target_player,
                effect: None,
            };
            game_state.current_action = GameAction::UsingItem;
        }
    });
}

pub fn player_hud(
    mut egui_context: ResMut<EguiContext>,
    mut query: Query<&mut Player>,
    mut game_state: ResMut<GameState>,
) {
    die_inspector(&mut egui_context, &mut query, &mut game_state);
    if let GameAction::UsingItem = game_state.current_action {
        match item_preview(&mut egui_context, &mut query, &mut game_state) {
            ItemAction::NoAction => {}
            ItemAction::UseItem => end_turn(&mut game_state),
            ItemAction::CancelItem => game_state.current_action = GameAction::HasMoved,
        }
    }
    if game_state.inventory_visible {
        inventory_window(&mut egui_context, &mut query, &mut game_state);
    }
}

pub fn pause_menu(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    game_state: Res<GameState>,
) {
    if game_state.paused || game_state.game_over {
        egui::Window::new("Pause").show(egui_context.ctx_mut(), |ui| {
            if ui.button("Back to Main").clicked() {
                state.set(AppState::MainMenu).unwrap();
            }
        });
    }
}

pub fn cleanup_game(mut commands: Commands, query: Query<Entity, With<Transform>>) {
    commands.remove_resource::<Map>();
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
