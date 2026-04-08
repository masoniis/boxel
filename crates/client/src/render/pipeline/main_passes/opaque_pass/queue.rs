use crate::prelude::*;
use crate::render::{
    pipeline::main_passes::opaque_pass::extract::OpaqueRenderMeshComponent,
    types::RenderTransformComponent,
};
use bevy::ecs::prelude::*;
use bevy::render::view::ExtractedView;

#[derive(Debug)]
pub struct PhaseItem {
    pub entity: Entity,
    pub distance: f32, // for sorting front-to-back
}

#[derive(Resource, Default)]
pub struct Opaque3dRenderPhase {
    pub items: Vec<PhaseItem>,
}

// A temporary struct to hold all the data we need for sorting
struct SortableOpaqueItem {
    distance: f32,
    entity: Entity,
}

/// The system responsible for populating the `RenderQueueResource`.
///
/// Performs a query for all entities that have been extracted into the render
/// world and adds them to a list of draw calls for the renderer to consume.
#[instrument(skip_all)]
pub fn queue_opaque_system(
    // input
    view_query: Query<&ExtractedView>,
    meshes_query: Query<(
        Entity,
        &OpaqueRenderMeshComponent,
        &RenderTransformComponent,
    )>,

    // output
    mut opaque_phase: ResMut<Opaque3dRenderPhase>,
) {
    let Some(extracted_view) = view_query.iter().next() else {
        return;
    };

    opaque_phase.items.clear();

    // collect sortable items for the render pass
    let camera_position = extracted_view.world_from_view.translation();
    let mut sortable_items: Vec<SortableOpaqueItem> = Vec::with_capacity(meshes_query.iter().len());
    for (entity, _mesh, transform) in meshes_query.iter() {
        // TODO: Frustum culling here

        let object_position = transform.transform.w_axis.truncate();
        let distance_from_camera = (object_position - camera_position).length_squared();

        sortable_items.push(SortableOpaqueItem {
            distance: distance_from_camera,
            entity,
        });
    }

    // sort by front to back (which optimizes early z-culling in the GPU)
    sortable_items.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    // populate the phase and object buffer in correct sorted order
    for item in sortable_items {
        opaque_phase.items.push(PhaseItem {
            entity: item.entity,
            distance: item.distance,
        });
    }
}
