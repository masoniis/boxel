pub mod extract;
pub mod prepare;
pub mod queue;
pub mod render;
pub mod startup;

pub use render::OpaquePassRenderNode;

// INFO: ---------------------------
//         Plugin definition
// ---------------------------------

use crate::{
    render::pipeline::main_passes::opaque_pass::queue::Opaque3dRenderPhase, VantablockNode,
};
use bevy::app::{App, Plugin};
use bevy::ecs::prelude::*;
use bevy::prelude::IntoScheduleConfigs;
use bevy::render::render_graph::{RenderGraphExt, ViewNodeRunner};
use bevy::render::view::ExtractedView;
use bevy::render::{Render, RenderSystems};
use startup::OpaquePipelines;

pub struct OpaqueRenderPassPlugin;

impl Plugin for OpaqueRenderPassPlugin {
    fn build(&self, app: &mut App) {
        // INFO: -----------------
        //         prepare
        // -----------------------

        app.add_systems(
            Render,
            prepare::prepare_opaque_meshes_system.in_set(RenderSystems::Prepare),
        );

        // INFO: ---------------
        //         queue
        // ---------------------

        app.add_systems(
            Render,
            (
                |mut commands: Commands, query: Query<Entity, Added<ExtractedView>>| {
                    for entity in query.iter() {
                        commands
                            .entity(entity)
                            .insert(Opaque3dRenderPhase::default());
                    }
                },
                queue::queue_opaque_system,
            )
                .chain()
                .in_set(RenderSystems::Queue),
        );

        // INFO: -----------------------------------------
        //         render graph integration
        // -----------------------------------------------

        app.add_render_graph_node::<ViewNodeRunner<OpaquePassRenderNode>>(
            bevy::core_pipeline::core_3d::graph::Core3d,
            VantablockNode::OpaquePass,
        );
    }

    fn finish(&self, app: &mut App) {
        // INFO: -----------------
        //         Startup
        // -----------------------

        app.init_resource::<OpaquePipelines>();
    }
}
