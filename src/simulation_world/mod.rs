pub mod app_lifecycle;
pub mod asset_management;
pub mod biome;
pub mod block;
pub mod chunk;
pub mod input;
pub mod player;
pub mod scheduling;
pub mod showcase;
pub mod terrain;
pub mod time;
pub mod user_interface;

pub use scheduling::{FixedUpdateSet, SimulationSet, StartupSet};

// INFO: -----------------------------
//         Sim world interface
// -----------------------------------

use crate::render_world::{
    global_extract::utils::initialize_simulation_world_for_extract,
    textures::TextureRegistryResource,
};
use crate::simulation_world::app_lifecycle::AppLifecyclePlugin;
use crate::simulation_world::{
    asset_management::AssetManagementPlugin,
    biome::BiomePlugin,
    block::BlockPlugin,
    chunk::ChunkLoadingPlugin,
    input::{InputModulePlugin, WindowSizeResource},
    player::PlayerPlugin,
    showcase::ShowcasePlugin,
    terrain::TerrainGenerationPlugin,
    time::TimeControlPlugin,
    user_interface::UiPlugin,
};
use bevy::app::{App, FixedUpdate, Plugin, Startup, Update};
use bevy::ecs::prelude::*;
use std::ops::{Deref, DerefMut};
use winit::window::Window;

use crate::ecs_core::worlds::SimulationWorldMarker;

pub struct SimulationWorldInterface {
    pub app: App,
}

// Safety: We do not use Bevy's internal runner, and we carefully manage threading manually.
unsafe impl Send for SimulationWorldInterface {}
unsafe impl Sync for SimulationWorldInterface {}

impl SimulationWorldInterface {
    pub fn send_event<E: Message>(&mut self, event: E) {
        self.app.world_mut().write_message(event);
    }

    /// Runs a single frame of the simulation world.
    ///
    /// This manually runs the core bevy update schedule but purposefully skips
    /// any schedule that clears change trackers (like `Last`). Clearing
    /// trackers is handled manually after the render world extracts data.
    pub fn run_frame(&mut self) {
        self.app.world_mut().run_schedule(Update);
    }

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

impl Deref for SimulationWorldInterface {
    type Target = App;
    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl DerefMut for SimulationWorldInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

impl SimulationWorldInterface {
    pub fn new(window: &Window, texture_registry_resource: TextureRegistryResource) -> Self {
        let mut app = App::new();

        // add resources built from the app
        app.insert_resource(WindowSizeResource::new(window.inner_size()))
            .insert_resource(texture_registry_resource);

        // configure schedule sets before adding plugins
        app.configure_sets(
            Startup,
            (StartupSet::ResourceInitialization, StartupSet::Tasks).chain(),
        );

        app.configure_sets(
            FixedUpdate,
            (FixedUpdateSet::PreUpdate, FixedUpdateSet::MainLogic).chain(),
        );

        app.configure_sets(
            Update,
            (
                SimulationSet::Input,
                SimulationSet::PreUpdate,
                SimulationSet::Update,
                SimulationSet::Physics,
                SimulationSet::PostUpdate,
                SimulationSet::RenderPrep,
            )
                .chain(),
        );

        // now add plugins, which can safely use the configured sets
        app.add_plugins(SharedPlugins)
            .add_plugins(ClientOnlyPlugins);

        Self::build_simulation_world(app)
    }

    fn build_simulation_world(mut app: App) -> SimulationWorldInterface {
        initialize_simulation_world_for_extract(app.world_mut());
        app.world_mut().insert_resource(SimulationWorldMarker);

        SimulationWorldInterface { app }
    }
}

// INFO: ---------------------------------
//         Plugin Groups (private)
// ---------------------------------------

/// Plugins to run on both the server and client
struct SharedPlugins;
impl Plugin for SharedPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AppLifecyclePlugin,
            AssetManagementPlugin,
            BlockPlugin,
            BiomePlugin,
            ChunkLoadingPlugin,
            TerrainGenerationPlugin,
            TimeControlPlugin,
        ));
    }
}

/// Plugins to run on solely on a client (UI, etc)
struct ClientOnlyPlugins;
impl Plugin for ClientOnlyPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, UiPlugin, InputModulePlugin, ShowcasePlugin));
    }
}
