pub mod biome_definition;
pub mod biome_registry;

pub use biome_definition::BiomeDefinition;
pub use biome_registry::{BiomeId, BiomeRegistryResource, load_biome_defs_from_disk};

// INFO: ----------------------
//         Biome plugin
// ----------------------------

use crate::simulation_world::{
    biome::biome_registry::initialize_biome_registry_system, scheduling::StartupSet,
};
use bevy::app::{App, Plugin, Startup};
use bevy::ecs::schedule::IntoScheduleConfigs;

pub struct BiomePlugin;

impl Plugin for BiomePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BiomeRegistryResource::default());

        app.add_systems(
            Startup,
            initialize_biome_registry_system.in_set(StartupSet::Tasks),
        );
    }
}
