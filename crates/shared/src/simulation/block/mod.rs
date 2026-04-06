pub mod block_definition;
pub mod block_registry;
pub mod targeted_block;

pub use block_definition::{
    BlockDescription, BlockFaceTextures, BlockRenderData, load_block_from_str,
};
pub use block_registry::{AIR_BLOCK_ID, BlockId, BlockRegistryResource, SOLID_BLOCK_ID};
pub use targeted_block::TargetedBlock;

// INFO: ----------------------
//         Block plugin
// ----------------------------

use crate::simulation::block::block_registry::initialize_block_registry_system;
use bevy::app::{App, Plugin, Startup};

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        // insert resources
        app.init_resource::<TargetedBlock>();
        app.init_resource::<BlockRegistryResource>();

        // We run the initialization at startup.
        // Note: For the client, this should run after texture registry is ready.
        app.add_systems(Startup, initialize_block_registry_system);
    }
}

pub mod texture_registry;
