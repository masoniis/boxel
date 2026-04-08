use bevy::prelude::*;
use crate::state::ClientAppState;

pub mod systems;

// INFO: -------------------
//         ui plugin
// -------------------------

pub struct VantablockUiPlugin;

impl Plugin for VantablockUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ClientAppState::Running),
            systems::spawning::spawn_ui_system,
        )
        .add_systems(
            OnExit(ClientAppState::Running),
            systems::spawning::despawn_ui_system,
        );
    }
}
