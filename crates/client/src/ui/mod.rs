use bevy::prelude::*;
use shared::ecs_core::AppState;

pub mod systems;

// INFO: -------------------
//         ui plugin
// -------------------------

pub struct VantablockUiPlugin;

impl Plugin for VantablockUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Running),
            systems::spawning::spawn_ui_system,
        )
        .add_systems(
            OnExit(AppState::Running),
            systems::spawning::despawn_ui_system,
        );
    }
}
