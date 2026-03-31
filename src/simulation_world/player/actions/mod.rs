pub mod voxel;

pub use voxel::*;

// INFO: -------------------------------
//         actions module plugin
// -------------------------------------

use crate::{
    SimulationAction, SimulationSet,
    simulation_world::{
        input::ActionStateResource,
        player::{
            break_targeted_voxel::{BreakVoxelEvent, handle_break_voxel_events_system},
            place_voxel_at_target::{
                PlaceVoxelEvent, handle_place_voxel_events_system, place_targeted_voxel_system,
            },
        },
    },
};
use bevy::app::{App, FixedUpdate, Plugin, Update};
use bevy::ecs::{
    message::Messages,
    schedule::{IntoScheduleConfigs, SystemSet},
    system::Res,
};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystemSet {
    WindowEvents,
    DeviceEvents,
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        // update targeted block
        app.add_systems(FixedUpdate, update_targeted_block_system);

        // break voxel on click
        app.init_resource::<Messages<BreakVoxelEvent>>()
            .add_systems(
                Update,
                (
                    handle_break_voxel_events_system,
                    break_targeted_voxel_system
                        .in_set(SimulationSet::Update)
                        .run_if(|action_state: Res<ActionStateResource>| {
                            action_state.just_happened(SimulationAction::BreakVoxel)
                        }),
                ),
            );

        // add voxel on right click
        app.init_resource::<Messages<PlaceVoxelEvent>>()
            .add_systems(
                Update,
                (
                    handle_place_voxel_events_system,
                    place_targeted_voxel_system
                        .in_set(SimulationSet::Update)
                        .run_if(|action_state: Res<ActionStateResource>| {
                            action_state.just_happened(SimulationAction::PlaceVoxel)
                        }),
                ),
            );
    }
}
