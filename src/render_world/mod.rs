pub mod global_extract;
pub mod graphics_context;
pub mod passes;
pub mod scheduling;
pub mod textures;
pub mod types;

use crate::render_world::textures::StagingTextureImages;
pub use scheduling::{RenderSchedule, RenderSet};

// INFO: --------------------------------
//         render world interface
// --------------------------------------

use crate::ecs_core::worlds::RenderWorldMarker;
use crate::render_world::{
    global_extract::{
        RenderMeshStorageResource, RenderTimeResource, RenderWindowSizeResource,
        SimulationExtractionPlugin,
    },
    graphics_context::{GraphicsContext, GraphicsContextPlugin},
    passes::{RenderPassManagerPlugin, core::setup_render_graph},
};
use bevy::app::App;
use bevy::ecs::prelude::*;
use std::ops::{Deref, DerefMut};

pub struct RenderWorldInterface {
    pub app: App,
}

// Safety: We do not use Bevy's internal runner, and we carefully manage threading manually.
unsafe impl Send for RenderWorldInterface {}
unsafe impl Sync for RenderWorldInterface {}

impl Deref for RenderWorldInterface {
    type Target = App;
    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl DerefMut for RenderWorldInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

impl RenderWorldInterface {
    /// Adds a resource to the world via insertion.
    pub fn add_resource<R: Resource>(&mut self, resource: R) {
        self.app.insert_resource(resource);
    }

    /// Retrieves a resource from the world, if it exists.
    pub fn get_resource<R: Resource>(&self) -> Option<&R> {
        self.app.world().get_resource::<R>()
    }

    /// Run a schedule by its label, if it exists.
    pub fn run_schedule(&mut self, label: impl bevy::ecs::schedule::ScheduleLabel + Clone) {
        self.app.world_mut().run_schedule(label);
    }

    /// Clears the world's internal change trackers.
    ///
    /// This MUST be called at the end of a world's update cycle to ensure
    /// change detection works correctly on the next frame.
    pub fn clear_trackers(&mut self) {
        self.app.world_mut().clear_trackers();
    }

    /// Provides mutable access to the underlying world.
    pub fn world_mut(&mut self) -> &mut World {
        self.app.world_mut()
    }

    /// Provides access to the underlying world.
    pub fn world(&self) -> &World {
        self.app.world()
    }
}

impl RenderWorldInterface {
    /// Creates a new render world with a sane default configuration
    pub fn new(
        graphics_context: GraphicsContext,
        staging_texture_images: StagingTextureImages,
    ) -> Self {
        let mut app = App::new();

        // INFO: -----------------------------------------------------
        //         set up graphics-context dependent resources
        // -----------------------------------------------------------

        // Setup render graph runs as an early system since it needs mutable world access
        setup_render_graph(app.world_mut());

        // Add any resources that require specific app input
        app.insert_resource(staging_texture_images)
            .insert_resource(RenderWorldMarker);

        // INFO: --------------------------------
        //         non-mod specific setup
        // --------------------------------------

        app.configure_sets(
            RenderSchedule::Main,
            (RenderSet::Prepare, RenderSet::Queue, RenderSet::Render).chain(),
        );

        // Resources for rendering
        app.init_resource::<RenderTimeResource>()
            .init_resource::<RenderWindowSizeResource>()
            .init_resource::<RenderMeshStorageResource>();

        // Specifically implemented plugins
        app.add_plugins((
            GraphicsContextPlugin::new(graphics_context),
            RenderPassManagerPlugin,
            SimulationExtractionPlugin,
        ));

        Self::build_render_world(app)
    }

    /// Builds the final state and returns the final render world interface.
    fn build_render_world(app: App) -> RenderWorldInterface {
        RenderWorldInterface { app }
    }
}
