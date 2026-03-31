pub mod extract;
pub mod prepare;
pub mod queue;
pub mod render;
pub mod startup;

pub use render::TransparentPassRenderNode;
use startup::TransparentPipeline;

// INFO: ---------------------------
//         plugin definition
// ---------------------------------

use crate::render_world::{
    RenderSchedule, RenderSet,
    global_extract::ExtractComponentPlugin,
    passes::world::main_passes::opaque_pass::prepare::delete_gpu_buffers_system,
    passes::world::main_passes::transparent_pass::{
        prepare::prepare_transparent_meshes_system,
        queue::{Transparent3dRenderPhase, queue_and_prepare_transparent_system},
    },
};
use crate::simulation_world::chunk::mesh::TransparentMeshComponent;
use bevy::app::{App, Plugin};
use bevy::ecs::schedule::IntoScheduleConfigs;

pub struct TransparentRenderPassPlugin;

impl Plugin for TransparentRenderPassPlugin {
    fn build(&self, app: &mut App) {
        // INFO: -----------------
        //         Startup
        // -----------------------

        app.init_resource::<TransparentPipeline>();

        // INFO: -----------------
        //         Extract
        // -----------------------

        app.add_plugins(ExtractComponentPlugin::<TransparentMeshComponent>::default());

        // INFO: -----------------
        //         Prepare
        // -----------------------

        app.add_systems(
            RenderSchedule::Main,
            (
                delete_gpu_buffers_system.before(prepare_transparent_meshes_system),
                prepare_transparent_meshes_system,
            )
                .in_set(RenderSet::Prepare),
        );

        // INFO: ---------------
        //         Queue
        // ---------------------

        app
            // resources
            .init_resource::<Transparent3dRenderPhase>()
            // systems
            .add_systems(
                RenderSchedule::Main,
                queue_and_prepare_transparent_system.in_set(RenderSet::Queue),
            );
    }
}
