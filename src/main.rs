use bevy::prelude::*;
use dicey_dungeons::*;

mod main_menu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::MainMenu)
        .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(main_menu::setup_menu))
        .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(main_menu::main_menu))
        .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(main_menu::cleanup_menu))
        .run();
}
