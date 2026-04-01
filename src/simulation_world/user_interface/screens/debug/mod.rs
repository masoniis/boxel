pub mod debug_screen;
pub mod elements;

pub use debug_screen::{diagnostic_ui_is_visible, toggle_debug_diagnostics_system};
pub use elements::active_gen_text::update_active_gen_text_system;
pub use elements::current_biome::update_current_biome_text_system;
pub use elements::fps_counter::update_fps_counter_screen_text_system;
pub use elements::mesh_counter::{MeshCounterResource, update_mesh_counter_screen_text_system};

// INFO: ----------------
//         plugin
// ----------------------

use crate::prelude::*;
use crate::simulation_world::input::ActionStateResource;
use crate::simulation_world::user_interface::screens::elements::memory_counter::SystemInfoResource;
use crate::simulation_world::user_interface::screens::elements::mesh_counter::{
    mesh_add_observer, mesh_remove_observer,
};
use crate::simulation_world::user_interface::screens::elements::{
    update_camera_chunk_coord_screen_text, update_camera_xyz_coord_screen_text,
    update_memory_counter_screen_text,
};
use bevy::app::{App, FixedUpdate, Plugin, PostUpdate};
use bevy::ecs::prelude::*;

pub struct DebugScreenPlugin;

impl Plugin for DebugScreenPlugin {
    fn build(&self, app: &mut App) {
        // mesh counting utils
        app.init_resource::<MeshCounterResource>()
            .init_resource::<SystemInfoResource>()
            .add_observer(mesh_add_observer)
            .add_observer(mesh_remove_observer);

        app.add_systems(
            PostUpdate,
            (
                toggle_debug_diagnostics_system.run_if(
                    (|action_state: Res<ActionStateResource>| {
                        action_state.just_happened(SimulationAction::ToggleDiagnostics)
                    })
                    .and(in_state(AppState::Running)),
                ),
                update_mesh_counter_screen_text_system
                    .run_if(resource_changed::<MeshCounterResource>),
            ),
        );

        app.add_systems(
            FixedUpdate,
            (
                update_camera_xyz_coord_screen_text,
                update_camera_chunk_coord_screen_text,
                update_current_biome_text_system,
                update_fps_counter_screen_text_system,
                update_memory_counter_screen_text,
                update_active_gen_text_system,
            )
                .run_if(diagnostic_ui_is_visible),
        );
    }
}
