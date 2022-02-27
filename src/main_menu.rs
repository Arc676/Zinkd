use bevy::prelude::*;
use crate::AppState;

struct MainMenu {
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    todo!()
}

pub fn main_menu(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    todo!()
}

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MainMenu>) {
    todo!()
}
