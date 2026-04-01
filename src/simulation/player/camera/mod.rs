pub mod component;
pub mod movement;
pub mod resource;

pub use component::*;
pub use movement::*;
pub use resource::*;

// INFO: -----------------------
//         camera plugin
// -----------------------------

use crate::ecs_core::AppState;
use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Update, in_state};

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
