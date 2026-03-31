pub mod extract;
pub mod prepare;
pub mod queue;
pub mod render;
pub mod startup;

pub use render::OpaquePassRenderNode;
use startup::OpaquePipelines;

// INFO: ---------------------------
//         Plugin definition
// ---------------------------------

use crate::{
    render_world::{
        global_extract::{ExtractComponentPlugin, extract_resource_system},
        passes::world::main_passes::opaque_pass::{
            extract::OpaqueRenderModeExtractor, queue::Opaque3dRenderPhase,
        },
        scheduling::{RenderSchedule, RenderSet},
    },
    simulation_world::chunk::OpaqueMeshComponent,
};
use bevy::app::{App, Plugin};
use bevy::ecs::prelude::*;

pub struct OpaqueRenderPassPlugin;

impl Plugin for OpaqueRenderPassPlugin {
    fn build(&self, app: &mut App) {
        // INFO: -----------------
        //         Startup
        // -----------------------

        app.init_resource::<OpaquePipelines>();

        // INFO: -----------------
        //         Extract
        // -----------------------

        app.add_plugins(ExtractComponentPlugin::<OpaqueMeshComponent>::default())
            .add_systems(
                RenderSchedule::Extract,
                extract_resource_system::<OpaqueRenderModeExtractor>,
            );

        // INFO: -----------------
        //         Prepare
        // -----------------------

        app.add_systems(
            RenderSchedule::Main,
            (
                prepare::delete_gpu_buffers_system.before(prepare::prepare_opaque_meshes_system),
                prepare::prepare_opaque_meshes_system,
            )
                .in_set(RenderSet::Prepare),
        );

        // INFO: ---------------
        //         Queue
        // ---------------------

        app
            // resources
            .init_resource::<Opaque3dRenderPhase>()
            // systems
            .add_systems(
                RenderSchedule::Main,
                queue::queue_opaque_system.in_set(RenderSet::Queue),
            );
    }
}
