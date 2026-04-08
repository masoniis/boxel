use crate::{
    prelude::*,
    simulation::chunk::{CHUNK_SIDE_LENGTH, ChunkCoord},
};
use bevy::ecs::prelude::*;
use bevy::prelude::{Camera3d, PerspectiveProjection, Projection, Transform, Vec3};

const DEFAULT_CAMERA_STARTING_X: f32 = (CHUNK_SIDE_LENGTH / 2) as f32;
const DEFAULT_CAMERA_STARTING_Y: f32 = 64.0;
const DEFAULT_CAMERA_STARTING_Z: f32 = (CHUNK_SIDE_LENGTH / 2) as f32;

/// A resource that holds the currently active camera entity (regarding rendering)
#[derive(Resource)]
pub struct ActiveCamera(pub Entity);

impl FromWorld for ActiveCamera {
    fn from_world(world: &mut World) -> Self {
        info!("Spawning default graphics camera via FromWorld.");

        let start_position = Vec3::new(
            DEFAULT_CAMERA_STARTING_X,
            DEFAULT_CAMERA_STARTING_Y,
            DEFAULT_CAMERA_STARTING_Z,
        );

        let start_chunk = ChunkCoord::world_to_chunk_pos(start_position);

        let camera_entity = world
            .spawn((
                Camera3d::default(),
                Projection::Perspective(PerspectiveProjection {
                    fov: 45.0f32.to_radians(),
                    near: 1.0,
                    far: f32::INFINITY,
                    ..Default::default()
                }),
                Transform::from_translation(start_position),
                ChunkCoord { pos: start_chunk },
            ))
            .id();

        Self(camera_entity)
    }
}
