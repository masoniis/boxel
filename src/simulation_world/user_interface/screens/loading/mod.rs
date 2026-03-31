pub mod loading_screen;

pub use loading_screen::{despawn_loading_ui_system, spawn_loading_ui_system};

// INFO: ----------------
//         Plugin
// ----------------------

use crate::{ecs_core::state_machine::AppState, simulation_world::scheduling::StartupSet};
use bevy::app::{App, Plugin, Startup};
use bevy::ecs::prelude::*;
use bevy::state::state::OnExit;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_loading_ui_system.in_set(StartupSet::Tasks));

        app.add_systems(OnExit(AppState::StartingUp), despawn_loading_ui_system);
    }
}
