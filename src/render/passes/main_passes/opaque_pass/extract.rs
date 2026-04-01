use crate::render::types::RenderTransformComponent;
use crate::simulation::{
    asset_management::VoxelChunkMeshAsset,
    chunk::{OpaqueMeshComponent, TransformComponent},
};
use bevy::asset::Handle;
use bevy::ecs::prelude::*;
use bevy::ecs::query::QueryItem;
use bevy::render::extract_component::ExtractComponent;

// INFO: --------------------------------
//         RenderWorld components
// --------------------------------------

/// A component in the render world holding the extracted mesh handle.
#[derive(Component, Clone)]
pub struct OpaqueRenderMeshComponent {
    pub mesh_handle: Handle<VoxelChunkMeshAsset>,
}

// INFO: -----------------------------------
//         SimWorld extraction logic
// -----------------------------------------

/// Mirror properties of `MeshComponent` from the simulation world
impl ExtractComponent for OpaqueMeshComponent {
    type QueryData = (&'static OpaqueMeshComponent, &'static TransformComponent);
    type QueryFilter = ();
    type Out = (OpaqueRenderMeshComponent, RenderTransformComponent);

    fn extract_component(item: QueryItem<'_, '_, Self::QueryData>) -> Option<Self::Out> {
        let (mesh, transform) = item;
        Some((
            OpaqueRenderMeshComponent {
                mesh_handle: mesh.mesh_handle.clone(),
            },
            RenderTransformComponent {
                transform: transform.to_matrix(),
            },
        ))
    }
}
