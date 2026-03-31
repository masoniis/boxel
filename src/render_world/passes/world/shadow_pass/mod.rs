pub mod gpu_resources;
pub mod prepare;
pub mod render;

// INFO: ---------------------------
//         plugin definition
// ---------------------------------

use bevy::ecs::schedule::IntoScheduleConfigs;
use gpu_resources::{
    ShadowDepthTextureResource, ShadowPassPipeline, ShadowViewBuffer,
    shadow_view_uniform::ShadowViewBindGroupLayout,
};

use crate::{
    RenderSet,
    render_world::{
        RenderSchedule, passes::world::shadow_pass::prepare::update_shadow_view_buffer_system,
    },
};
use bevy::app::{App, Plugin};

pub struct ShadowRenderPassPlugin;

impl Plugin for ShadowRenderPassPlugin {
    fn build(&self, app: &mut App) {
        // INFO: -----------------
        //         startup
        // -----------------------

        app.init_resource::<ShadowViewBindGroupLayout>()
            .init_resource::<ShadowPassPipeline>()
            .init_resource::<ShadowViewBuffer>()
            .init_resource::<ShadowDepthTextureResource>();

        // INFO: -----------------
        //         prepare
        // -----------------------

        app.add_systems(
            RenderSchedule::Main,
            update_shadow_view_buffer_system.in_set(RenderSet::Prepare),
        );
    }
}
