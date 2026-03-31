pub mod core;
pub mod ui_pass;
pub mod world;

// INFO: ---------------------------
//         plugin definition
// ---------------------------------

use crate::render_world::{
    passes::{
        core::execute_render_graph_system, ui_pass::UiRenderPassPlugin,
        world::WorldRenderPassesPlugin,
    },
    scheduling::{RenderSchedule, RenderSet},
};
use bevy::app::{App, Plugin};
use bevy::ecs::schedule::IntoScheduleConfigs;

/// A plugin that sets up all the necessary resources and render
/// passes used in the rendering pipeline.
pub struct RenderPassManagerPlugin;

impl Plugin for RenderPassManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WorldRenderPassesPlugin, UiRenderPassPlugin));

        // INFO: ----------------
        //         render
        // ----------------------

        app.add_systems(
            RenderSchedule::Main,
            execute_render_graph_system.in_set(RenderSet::Render),
        );
    }
}
