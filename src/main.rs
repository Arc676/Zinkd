use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use dicey_dungeons::*;

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
        .add_system_set(SystemSet::on_exit(AppState::Game).with_system(game::cleanup_game))
        .add_system_set(SystemSet::on_update(AppState::Settings).with_system(settings::settings_ui))
        .run();
}
