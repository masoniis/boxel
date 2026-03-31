pub mod extract;
pub mod gpu_resources;
pub mod prepare;
pub mod queue;
pub mod render;

use gpu_resources::{
    ScreenQuadResource, UiMaterialBindGroupLayout, UiMaterialBuffer, UiObjectBindGroupLayout,
    UiObjectBuffer, UiPipeline,
    view_binding::{UiViewBindGroupLayout, UiViewBuffer},
};
pub use render::UiRenderPassNode;

// INFO: ---------------------------
//         Plugin definition
// ---------------------------------

use crate::render_world::{
    global_extract::resources::RenderWindowSizeResource,
    passes::ui_pass::{
        extract::ExtractedUiEvents,
        prepare::UiChanges,
        queue::{IsGlyphonDirty, PreparedUiBatches, UiElementCache, UiElementSortBufferResource},
    },
    scheduling::{RenderSchedule, RenderSet},
};
use bevy::app::{App, Plugin};
use bevy::ecs::prelude::*;

pub struct UiRenderPassPlugin;

impl Plugin for UiRenderPassPlugin {
    fn build(&self, app: &mut App) {
        // INFO: -----------------
        //         startup
        // -----------------------

        // pipeline setup
        app
            // ui view uniform
            .init_resource::<UiViewBindGroupLayout>()
            .init_resource::<UiViewBuffer>()
            // ui material uniform
            .init_resource::<UiMaterialBindGroupLayout>()
            .init_resource::<UiMaterialBuffer>()
            // ui object uniform
            .init_resource::<UiObjectBindGroupLayout>()
            .init_resource::<UiObjectBuffer>()
            // pipeline
            .init_resource::<UiPipeline>();

        // general resources
        app.init_resource::<ScreenQuadResource>();
        app.add_systems(
            RenderSchedule::Startup,
            gpu_resources::setup_glyphon_resources,
        );

        // INFO: -----------------
        //         extract
        // -----------------------

        app
            // resources
            .insert_resource(ExtractedUiEvents::default())
            // systems
            .add_systems(RenderSchedule::Extract, extract::extract_ui_events_system);

        // INFO: -----------------
        //         prepare
        // -----------------------

        app
            // resources
            .init_resource::<PreparedUiBatches>()
            .init_resource::<UiElementSortBufferResource>()
            .init_resource::<IsGlyphonDirty>()
            .init_resource::<UiChanges>()
            .init_resource::<UiElementCache>()
            // schedule
            .add_systems(
                RenderSchedule::Main,
                (
                    (
                        prepare::update_ui_view_data_system,
                        prepare::prepare_glyphon_view_system,
                    )
                        .run_if(resource_changed::<RenderWindowSizeResource>),
                    (prepare::process_ui_events_system,).chain(),
                )
                    .in_set(RenderSet::Prepare),
            );

        // INFO: ---------------
        //         Queue
        // ---------------------

        app.add_systems(
            RenderSchedule::Main,
            (
                // make decisions based on the UiChanges determined above
                (
                    queue::mark_glyphon_dirty_system,
                    queue::rebuild_ui_batches_system,
                ),
                // makes changes based on the buffers from the systems just before it
                queue::preprocess_glyphon_text_system.run_if(resource_equals(IsGlyphonDirty(true))),
            )
                .in_set(RenderSet::Queue)
                .chain(),
        );
    }
}
