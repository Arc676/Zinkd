use crate::AppState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_egui::egui::{Separator, Slider, Ui};

pub struct GameSettings {
    players: u32,
    map_width: u32,
    map_height: u32,
    initial_travel_distance: u32,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            players: 2,
            map_width: 10,
            map_height: 10,
            initial_travel_distance: 5,
        }
    }
}

impl GameSettings {
    pub fn reset_settings(&mut self) {
        *self = GameSettings::default();
    }
}

fn number_setting(ui: &mut Ui, num: &mut u32, min: u32, max: u32, lbl: &str) {
    ui.label(lbl);
    let slider = Slider::new(num, min..=max);
    ui.add(slider);
}

pub fn settings_ui(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    mut settings: ResMut<GameSettings>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.heading("Dicey Dungeons: Settings");

        number_setting(ui, &mut settings.players, 2, 6, "Number of players");
        number_setting(ui, &mut settings.map_width, 5, 20, "Map width");
        number_setting(ui, &mut settings.map_height, 5, 20, "Map height");

        let sep = Separator::default().spacing(12.).horizontal();
        ui.add(sep);

        ui.label("All players' starting positions will be connected to the goal by a path of \
         a fixed length before additional paths are generated. This initial distance can be \
         freely chosen.");
        number_setting(ui, &mut settings.initial_travel_distance, 2, 20, "Initial travel distance");

        let sep = Separator::default().spacing(12.).horizontal();
        ui.add(sep);

        if ui.button("Revert to default settings").clicked() {
            settings.reset_settings();
        }

        let sep = Separator::default().spacing(12.).horizontal();
        ui.add(sep);

        if ui.button("Back to Main").clicked() {
            state.set(AppState::MainMenu).unwrap();
        }
    });
}
