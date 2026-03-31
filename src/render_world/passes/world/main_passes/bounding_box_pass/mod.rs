pub mod extract;
pub mod gpu_resources;
pub mod queue;
pub mod render;

pub use render::BoundingBoxNode;

// INFO: ---------------------------
//         plugin definition
// ---------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    render_world::{
        RenderSchedule, RenderSet,
        global_extract::{clone_resource_system, extract_resource_system},
        passes::world::main_passes::bounding_box_pass::{
            extract::WireframeToggleExtractor, queue::queue_wireframe_system,
        },
    },
    simulation_world::block::TargetedBlock,
};
use bevy::ecs::schedule::IntoScheduleConfigs;
use gpu_resources::{
    UnitCubeMesh, WireframeObjectBuffer, WireframePipeline,
    object_binding::WireframeObjectBindGroupLayout,
};

pub struct WireframeRenderPassPlugin;

impl Plugin for WireframeRenderPassPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // INFO: -----------------
        //         startup
        // -----------------------

        builder
            .init_resource::<WireframeObjectBindGroupLayout>()
            .init_resource::<WireframeObjectBuffer>()
            .init_resource::<WireframePipeline>()
            .init_resource::<UnitCubeMesh>();

        // INFO: -----------------
        //         extract
        // -----------------------

        builder
            .schedule_entry(RenderSchedule::Extract)
            .add_systems((
                extract_resource_system::<WireframeToggleExtractor>,
                clone_resource_system::<TargetedBlock>,
            ));

        // INFO: ---------------
        //         queue
        // ---------------------

        builder
            .schedule_entry(RenderSchedule::Main)
            .add_systems(queue_wireframe_system.in_set(RenderSet::Queue));
    }
}
