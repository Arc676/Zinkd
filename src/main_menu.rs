use bevy::prelude::*;
use crate::AppState;

pub struct MainMenu {
    play_btn: Entity,
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    let play_btn = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Play",
                    TextStyle {
                        color: Color::BLACK,
                        font_size: 40.,
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        ..Default::default()
                    },
                    Default::default()
                ),
                ..Default::default()
            });
        })
        .id();
    commands.insert_resource(MainMenu { play_btn });
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
