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

use crate::prelude::*;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // insert resources
        builder
            .init_resource::<BlockRegistryResource>()
            .init_resource::<TargetedBlock>();
    }
}
