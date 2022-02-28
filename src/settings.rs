use crate::AppState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

pub fn settings_ui(mut egui_context: ResMut<EguiContext>, mut state: ResMut<State<AppState>>) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.heading("Dicey Dungeons: Settings");

        if ui.button("Back to Main").clicked() {
            state.set(AppState::MainMenu).unwrap();
        }
    });
}
