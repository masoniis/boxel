pub mod global_extract;
pub mod passes;
pub mod scheduling;
pub mod textures;
pub mod types;

// INFO: --------------------------------
//         render world interface
// --------------------------------------

use crate::prelude::*;
use crate::render::global_extract::{
    ExtractedSun, MeshesToUploadQueue, RenderMeshStorageResource, RenderTimeResource,
    SimulationExtractionPlugin,
};
use crate::render::passes::main_passes::bounding_box_pass::extract::WireframeToggleState;
use crate::render::passes::main_passes::opaque_pass::startup::OpaqueRenderMode;
use crate::render::passes::{RenderGraphEdgesPlugin, WorldRenderPassesPlugin};
use crate::render::textures::BlockTextureArray;
use crate::simulation::asset_management::VoxelChunkMeshAsset;
use crate::simulation::block::TargetedBlock;
use crate::simulation::chunk::OpaqueMeshComponent;
use crate::simulation::chunk::mesh::TransparentMeshComponent;
use bevy::app::{App, Plugin, SubApp};
use bevy::asset::AssetApp;
use bevy::render::RenderApp;
use bevy::render::extract_component::ExtractComponentPlugin;
use bevy::render::extract_resource::ExtractResourcePlugin;

/// Plugin responsible for attaching our custom render logic to Bevy's native RenderApp
pub struct VantablockRenderPlugin;

impl Plugin for VantablockRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(passes::shader_registry::VantablockShaderPlugin);

        app.init_asset::<VoxelChunkMeshAsset>();

        // register extraction plugins on the main app
        app.add_plugins((
            // resources
            ExtractResourcePlugin::<ExtractedSun>::default(),
            ExtractResourcePlugin::<RenderTimeResource>::default(),
            ExtractResourcePlugin::<OpaqueRenderMode>::default(),
            ExtractResourcePlugin::<TargetedBlock>::default(),
            ExtractResourcePlugin::<WireframeToggleState>::default(),
            ExtractResourcePlugin::<BlockTextureArray>::default(),
            // components
            ExtractComponentPlugin::<OpaqueMeshComponent>::default(),
            ExtractComponentPlugin::<TransparentMeshComponent>::default(),
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            error!("RenderApp not found! Ensure DefaultPlugins are added before this plugin.");
            return;
        };

        pre_setup_render_sub_app(render_app);
    }
}

/// Configures a sub-app with its base configuration, before graphics context is ready.
pub fn pre_setup_render_sub_app(sub_app: &mut SubApp) {
    // Resources for rendering
    sub_app
        .init_resource::<RenderTimeResource>()
        .init_resource::<RenderMeshStorageResource>()
        .init_resource::<MeshesToUploadQueue>();

    // Specifically implemented plugins (These run strictly in the Render World)
    sub_app.add_plugins((
        WorldRenderPassesPlugin,
        SimulationExtractionPlugin,
        RenderGraphEdgesPlugin,
    ));
}
