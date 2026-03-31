pub mod extract;
pub mod gpu_resources;
pub mod queue;
pub mod render;

pub use render::BoundingBoxNode;

// INFO: ---------------------------
//         plugin definition
// ---------------------------------

use crate::render_world::{
    RenderSchedule, RenderSet,
    global_extract::{clone_resource_system, extract_resource_system},
    passes::world::main_passes::bounding_box_pass::{
        extract::WireframeToggleExtractor, queue::queue_wireframe_system,
    },
};
use crate::simulation_world::block::TargetedBlock;
use bevy::app::{App, Plugin};
use bevy::ecs::schedule::IntoScheduleConfigs;
use gpu_resources::{
    UnitCubeMesh, WireframeObjectBuffer, WireframePipeline,
    object_binding::WireframeObjectBindGroupLayout,
};

pub struct WireframeRenderPassPlugin;

impl Plugin for WireframeRenderPassPlugin {
    fn build(&self, app: &mut App) {
        // INFO: -----------------
        //         startup
        // -----------------------

        app.init_resource::<WireframeObjectBindGroupLayout>()
            .init_resource::<WireframeObjectBuffer>()
            .init_resource::<WireframePipeline>()
            .init_resource::<UnitCubeMesh>();

        // INFO: -----------------
        //         extract
        // -----------------------

        app.add_systems(
            RenderSchedule::Extract,
            (
                extract_resource_system::<WireframeToggleExtractor>,
                clone_resource_system::<TargetedBlock>,
            ),
        );

        // INFO: ---------------
        //         queue
        // ---------------------

        app.add_systems(
            RenderSchedule::Main,
            queue_wireframe_system.in_set(RenderSet::Queue),
        );
    }
}
