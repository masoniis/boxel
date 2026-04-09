use crate::prelude::*;
use crate::render::texture::{BlockTextureArray, VoxelTextureProcessor};
use bevy::{
    asset::Assets,
    prelude::{Commands, Image, Res, World},
    tasks::AsyncComputeTaskPool,
};
use crossbeam::channel::unbounded;
use shared::{
    load::{SimulationWorldLoadingTaskComponent, TaskResultCallback},
    simulation::block::BlockRegistryResource,
};
use utils::PersistentPaths;

/// A system that starts the asynchronous initialization of texture and block registries
pub fn start_async_registry_initialization(
    mut commands: Commands,
    client_settings: Res<ClientSettings>,
    persistent_paths: Res<PersistentPaths>,
) {
    info!("Starting asynchronous simulation registry initialization...");

    let (sender, _receiver) = unbounded();
    let receiver = _receiver;
    let settings = client_settings.clone();
    let paths = persistent_paths.clone();

    AsyncComputeTaskPool::get()
        .spawn(async move {
            info!("Initializing simulation registries in background...");

            // texture stitching
            let (texture_array_image, texture_registry) =
                VoxelTextureProcessor::new(paths.assets_dir.clone(), &settings.texture_pack)
                    .load_and_stitch()
                    .expect("Failed to load and stitch textures");

            // block registry generation (depends on texture registry)
            let block_registry =
                BlockRegistryResource::load_from_disk(Some(&texture_registry), &paths);

            // prepare callback to apply results on main thread
            let callback: TaskResultCallback = Box::new(move |commands: &mut Commands| {
                info!("Applying simulation registry results to the world.");

                // access world via commands to insert resources
                commands.queue(move |world: &mut World| {
                    let mut image_assets = world.resource_mut::<Assets<Image>>();
                    let texture_handle = image_assets.add(texture_array_image);

                    // insert both registries
                    world.insert_resource(texture_registry);
                    world.insert_resource(block_registry);

                    // insert texture array handle
                    world.insert_resource(BlockTextureArray {
                        handle: texture_handle,
                    });
                });

                info!("Simulation registries initialized successfully.");
            });

            sender
                .send(callback)
                .expect("Failed to send registry task result");
        })
        .detach();

    // register this as a loading task so the game waits for it
    commands.spawn(SimulationWorldLoadingTaskComponent { receiver });
}
