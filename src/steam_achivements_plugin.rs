use std::collections::{HashSet};
use bevy::app::{App, Plugin, Startup};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_steamworks::{AppId, Client, SteamworksEvent, SteamworksPlugin};


/// Steam application ID of the Space War game. Replace this with the ID of your own game
const STEAM_APP_ID_TEST: AppId = AppId(480);

pub struct SteamIntegrationPlugin;

impl Plugin for SteamIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SteamworksPlugin::init_app(STEAM_APP_ID_TEST).unwrap());
        app.init_resource::<SteamIntegrationState>();
        app.add_systems(Startup, init_user_stats);
        app.add_systems(Update, steam_callback_handler);
        app.add_observer(on_unlock_achievement);
        app.add_observer(on_stat_progress);
    }
}

pub const STEAM_ACHIEVEMENT_WIN_ONE_GAME: &str = "ACH_WIN_ONE_GAME";
pub const STEAM_ACHIEVEMENT_WIN_100_GAMES: &str = "ACH_WIN_100_GAMES";
pub const STEAM_ACHIEVEMENT_TRAVEL_FAR_SINGLE: &str = "ACH_TRAVEL_FAR_SINGLE";
// The below is a stat-tracking achievement, unlocked by progressing the "FeetTraveled" stat
pub const STEAM_ACHIEVEMENT_TRAVEL_FAR_ACCUM: &str = "ACH_TRAVEL_FAR_ACCUM";

/// User stat for advancing the ACH_TRAVEL_FAR_ACCUM achievement
pub const STEAM_STAT_FEET_TRAVELED: &str = "FeetTraveled";

// All achievements in the Space War game. Replace with the achievements in your own game
const ALL_STEAM_ACHIEVEMENTS: [&str; 4] = [
    STEAM_ACHIEVEMENT_WIN_ONE_GAME,
    STEAM_ACHIEVEMENT_WIN_100_GAMES,
    STEAM_ACHIEVEMENT_TRAVEL_FAR_SINGLE,
    STEAM_ACHIEVEMENT_TRAVEL_FAR_ACCUM,
];

const ALL_STEAM_STATS: [&str; 1] = [
    STEAM_STAT_FEET_TRAVELED,
];

#[derive(Resource)]
pub struct SteamIntegrationState {
    user_stats_ready: bool,
    /// Set of all achievements that have been unlocked already
    unlocked: HashSet<String>,
    /// Map containing the value of all user stats
    stats_f32: HashMap<String, f32>
}

impl Default for SteamIntegrationState {
    fn default() -> Self {
        Self { user_stats_ready: false, unlocked: HashSet::new(), stats_f32: HashMap::new() }
    }
}

impl SteamIntegrationState {
    /// Returns true if the achievement is already unlocked
    pub fn already_unlocked(&self, achievement: &str) -> bool {
        self.unlocked.contains(achievement)
    }
}

#[derive(Event)]
struct UnlockSteamAchievementEvent {
    name: &'static str,
}

impl UnlockSteamAchievementEvent {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Event)]
struct ProgressSteamStatEvent {
    name: &'static str,
    add: f32,
}

impl ProgressSteamStatEvent {
    pub fn new(name: &'static str, add: f32) -> Self {
        Self { name, add }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn add(&self) -> f32 {
        self.add
    }
}

pub fn do_steam_achievement_unlock(commands: &mut Commands, achievement: &'static str) {
    commands.trigger(UnlockSteamAchievementEvent::new(achievement));
}

pub fn do_steam_stat_progress(commands: &mut Commands, stat_name: &'static str, add: f32) {
    commands.trigger(ProgressSteamStatEvent::new(stat_name, add));
}


/// Request the user stats from Steam
fn init_user_stats(steam_client: Res<Client>) {
    steam_client.user_stats().request_current_stats();
}

/// Observer system that unlocks an achievement when triggered by an UnlockSteamAchievementEvent
fn on_unlock_achievement(
    trigger: Trigger<UnlockSteamAchievementEvent>,
    steam_client: Res<Client>,
    steam_state: ResMut<SteamIntegrationState>,
) {
    if !steam_state.user_stats_ready {
        // Not ready, user stats callback was not received yet
        return;
    }

    let already_unlocked = steam_state.unlocked.get(trigger.name());
    if already_unlocked.is_some() {
        return;  // Achievement already unlocked, nothing to do
    }

    let result = steam_client.user_stats().achievement(trigger.name()).set();

    if result.is_ok() {
        let result = steam_client.user_stats().store_stats();
        if result.is_err() {
            log::error!("Error storing Steam achievement: {}", trigger.name());
        } else {
            log::info!("Successfully unlocking and storing Steam achievement: {}", trigger.name());
        }
    } else {
        log::error!("Error unlocking Steam achievement: {}", trigger.name());
    }
}

fn on_stat_progress(
    trigger: Trigger<ProgressSteamStatEvent>,
    steam_client: Res<Client>,
    mut steam_state: ResMut<SteamIntegrationState>,
) {
    if !steam_state.user_stats_ready {
        // Not ready, user stats callback was not received yet
        return;
    }

    let cur_value = steam_state.stats_f32.get(&trigger.name().to_string());
    if cur_value.is_some() {
        let new_value = cur_value.unwrap() + trigger.add();

        let result = steam_client.user_stats().set_stat_f32(trigger.name(), new_value);

        if result.is_ok() {
            let result = steam_client.user_stats().store_stats();

            if result.is_err() {
                log::error!("Error storing Steam stat: {}", trigger.name());
            } else {
                steam_state.stats_f32.insert(trigger.name.to_string(), new_value);

                log::info!("Successfully updated Steam stat: {} to {}", trigger.name(), new_value);
            }
        } else {
            log::error!("Error updating Steam stat: {}", trigger.name());
        }
    }
}

/// Handle events that represent Steam SDK callbacks
fn steam_callback_handler(
    mut event: EventReader<SteamworksEvent>,
    mut steam_state: ResMut<SteamIntegrationState>,
    steam_client: Res<Client>
) {
    for ev in event.read() {
        match ev {
            SteamworksEvent::SteamServerConnectFailure(_) => {}
            SteamworksEvent::SteamServersConnected(_) => {}
            SteamworksEvent::SteamServersDisconnected(_) => {}
            SteamworksEvent::UserAchievementStored(achievement) => {
                steam_state.unlocked.insert(achievement.achievement_name.clone());

                log::info!("Achievement unlock stored: {}", achievement.achievement_name);
            }
            SteamworksEvent::UserStatsReceived(recv) => {
                log::info!("Steam user stats received, status ok: {}", recv.result.is_ok());

                read_achievement_status(&steam_client, &mut steam_state);

                steam_state.user_stats_ready = recv.result.is_ok();
            }
            SteamworksEvent::UserStatsStored(stored) => {
                log::info!("Steam user stats stored, status ok: {}", stored.result.is_ok());
            }
            _ => { }
        }
    }
}

/// Check the unlocked state of each achievement from Steam and store the state in the internal game state.
/// Also read the achievement related stats from Steam and store those in the internal game state.
fn read_achievement_status(
    steam_client: &Client,
    steam_state: &mut SteamIntegrationState
) {
    for ach in ALL_STEAM_ACHIEVEMENTS {
        let result = steam_client.user_stats().achievement(ach).get();
        if result.is_ok() && result.unwrap() {
            steam_state.unlocked.insert(ach.to_string());

            log::info!("Achievement is already unlocked: {}", ach);
        }
    }

    for stat in ALL_STEAM_STATS {
        let result = steam_client.user_stats().get_stat_f32(stat);
        if result.is_ok() {
            let value = result.unwrap();
            steam_state.stats_f32.insert(stat.to_string(), value);

            log::info!("Stat {}, value: {}", stat, value);
        }
    }
}
