use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use crate::example_ui::example_ui;
use crate::steam_achivements_plugin::SteamIntegrationPlugin;

mod steam_achivements_plugin;
mod example_ui;

/// This example uses the Steam demo game Space War.
/// Install Space War by putting this URL in a browser while the Steam desktop app is running:
/// steam://run/480

/// When building an executable, put "steam_api64.dll" AND "steam_api64.lib" in the same folder as the .exe
/// They are found in build folder: target\release\build\steamworks-sys-??????????\out

/// When running this example, make sure that the Steam desktop app is running.

/// In Steam console, reset a Space War achievement with for instance:
/// achievement_clear 480 ACH_WIN_ONE_GAME
/// To reset all Space War stats, in Steam console:
/// reset_all_stats 480


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { ..Default::default() }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugins(SteamIntegrationPlugin)
        .add_plugins(EguiPlugin)  // For the example UI
        .add_systems(Update, example_ui) // Example UI
        .run();
}
