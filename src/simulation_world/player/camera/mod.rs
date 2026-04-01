pub mod component;
pub mod movement;
pub mod resource;

pub use component::*;
pub use movement::*;
pub use resource::*;

// INFO: -----------------------
//         Camera plugin
// -----------------------------

use crate::ecs_core::state_machine::{AppState, utils::in_state};
use bevy::app::{App, Plugin, Update};
use bevy::ecs::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveCamera>();

        app.add_systems(
            Update,
            (camera_movement_system, update_camera_chunk_chord_system)
                .chain()
                .run_if(in_state(AppState::Running)),
        );
    }
}
