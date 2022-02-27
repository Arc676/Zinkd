use bevy::app::AppExit;
use bevy::prelude::*;
use crate::AppState;

pub struct MainMenu {
    play_btn: Entity,
    quit_btn: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

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
            color: NORMAL_BUTTON.into(),
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
    let quit_btn = commands
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
                    "Quit",
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
    commands.insert_resource(MainMenu { play_btn, quit_btn });
}

pub fn main_menu(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    menu: Res<MainMenu>,
) {
    for (entity, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                if entity == menu.play_btn {
                    state.set(AppState::Game).unwrap();
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
    commands.entity(menu.quit_btn).despawn_recursive();
}
