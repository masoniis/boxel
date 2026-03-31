pub mod gpu_resources;
pub mod main_passes;
pub mod shadow_pass;

pub use gpu_resources::*;

// INFO: ---------------------------
//         plugin definition
// ---------------------------------

use crate::render_world::passes::world::{
    main_passes::PlayerCentricRenderPassPlugin, shadow_pass::ShadowRenderPassPlugin,
};
use bevy::app::{App, Plugin};

/// A plugin that sets up all the necessary resources and render
/// passes used in the rendering pipeline.
pub struct WorldRenderPassesPlugin;

impl Plugin for WorldRenderPassesPlugin {
    fn build(&self, app: &mut App) {
        // shared world uniform resources
        app.init_resource::<ChunkStorageBindGroupLayout>()
            .init_resource::<ChunkStorageManager>();

        // renderpass plugins
        app.add_plugins((ShadowRenderPassPlugin, PlayerCentricRenderPassPlugin));
    }
}
