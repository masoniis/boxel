pub mod asset_storage;
pub mod mesh_asset;

pub use asset_storage::{Asset, AssetStorageResource, Handle};
pub use mesh_asset::{MeshAsset, MeshDeletionRequest, delete_stale_mesh_assets};

// INFO: ---------------------------------
//         Asset Management Plugin
// ---------------------------------------

use crate::{
    SimulationSet,
    simulation_world::asset_management::mesh_asset::{
        MeshRefCounts, OpaqueMeshShadow, TransparentMeshShadow, opaque_mesh_added_observer,
        opaque_mesh_removed_observer, transparent_mesh_added_observer,
        transparent_mesh_removed_observer,
    },
};
use bevy::app::{App, Plugin, Update};
use bevy::ecs::{message::Messages, schedule::IntoScheduleConfigs};

pub struct AssetManagementPlugin;

impl Plugin for AssetManagementPlugin {
    fn build(&self, app: &mut App) {
        // the mesh asset storage
        app.insert_resource(AssetStorageResource::<MeshAsset>::default())
            .insert_resource(TransparentMeshShadow::default())
            .insert_resource(OpaqueMeshShadow::default());

        // mesh ref count tracking
        app.insert_resource(MeshRefCounts::default())
            .add_observer(opaque_mesh_added_observer)
            .add_observer(opaque_mesh_removed_observer)
            .add_observer(transparent_mesh_added_observer)
            .add_observer(transparent_mesh_removed_observer);

        // mesh deletion handling
        app.init_resource::<Messages<MeshDeletionRequest>>()
            .add_systems(
                Update,
                delete_stale_mesh_assets.in_set(SimulationSet::RenderPrep),
            );
    }
}
