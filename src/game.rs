use bevy::prelude::*;
use dicey_dungeons::map::Map;
use crate::AppState;

pub struct DungeonGame {
    map: Map,
}

pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
}

pub fn update_game(
    mut state: ResMut<State<AppState>>,
) {
}

pub fn cleanup_game(mut commands: Commands, menu: Res<DungeonGame>) {
}
