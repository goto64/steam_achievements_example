use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::steam_achivements_plugin::{do_steam_achievement_unlock, do_steam_stat_progress, SteamIntegrationState, STEAM_ACHIEVEMENT_TRAVEL_FAR_ACCUM, STEAM_ACHIEVEMENT_TRAVEL_FAR_SINGLE, STEAM_ACHIEVEMENT_WIN_100_GAMES, STEAM_ACHIEVEMENT_WIN_ONE_GAME, STEAM_STAT_FEET_TRAVELED};

pub fn example_ui(
    mut commands: Commands,
    mut contexts: EguiContexts,
    steam_state: Res<SteamIntegrationState>,
) {
    egui::Window::new("Space War Achievements")
        .resizable(true)
        .collapsible(false)
        .default_pos(egui::Pos2{x: 400.0, y: 250.0})
        .default_width(300.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Unlock achievements with buttons");
            ui.separator();

            let win_game_btn = ui_btn("I won a game");
            if ui.add_enabled(!steam_state.already_unlocked(STEAM_ACHIEVEMENT_WIN_ONE_GAME), win_game_btn).clicked() {
                do_steam_achievement_unlock(&mut commands, STEAM_ACHIEVEMENT_WIN_ONE_GAME)
            }
            ui.add_space(10.0);

            let win_100_games_btn = ui_btn("I won 100 games");
            if ui.add_enabled(!steam_state.already_unlocked(STEAM_ACHIEVEMENT_WIN_100_GAMES), win_100_games_btn).clicked() {
                do_steam_achievement_unlock(&mut commands, STEAM_ACHIEVEMENT_WIN_100_GAMES)
            }
            ui.add_space(10.0);

            let travel_far_btn = ui_btn("I travelled 500 feet in one game");
            if ui.add_enabled(!steam_state.already_unlocked(STEAM_ACHIEVEMENT_TRAVEL_FAR_SINGLE), travel_far_btn).clicked() {
                do_steam_achievement_unlock(&mut commands, STEAM_ACHIEVEMENT_TRAVEL_FAR_SINGLE)
            }
            ui.add_space(10.0);

            let travel_accum_btn = ui_btn("I travelled another 1000 feet");
            if ui.add_enabled(!steam_state.already_unlocked(STEAM_ACHIEVEMENT_TRAVEL_FAR_ACCUM), travel_accum_btn).clicked() {
                do_steam_stat_progress(&mut commands, STEAM_STAT_FEET_TRAVELED, 1000.0);
            }
            ui.add_space(10.0);

        });
}

fn ui_btn(text: &str) -> egui::Button {
    egui::Button::new(egui::RichText::new(text).size(16.0))
}