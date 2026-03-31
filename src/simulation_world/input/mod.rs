pub mod messages;
pub mod resources;
pub mod systems;
pub mod types;

pub use resources::{
    ActionStateResource, Buttons, CursorMovement, InputActionMapResource, WindowSizeResource,
};
pub use types::*;

// INFO: -----------------------------
//         input module plugin
// -----------------------------------

use crate::{
    ecs_core::state_machine::AppState,
    simulation_world::{
        input::{
            messages::{
                KeyboardInputMessage, MouseButtonInputMessage, MouseMoveMessage,
                MouseScrollMessage, RawDeviceMessage, RawWindowMessage,
                internal::MouseResizeMessage,
            },
            resources::DesiredCursorState,
            systems::{
                toggle_chunk_borders::ChunkBoundsToggle, toggle_chunk_borders_system,
                toggle_cursor_system, toggle_opaque_wireframe::OpaqueWireframeMode,
                toggle_opaque_wireframe_mode_system,
            },
        },
        scheduling::SimulationSet,
    },
};
use bevy::app::{App, Plugin, Update};
use bevy::ecs::{
    message::Messages,
    schedule::{IntoScheduleConfigs, SystemSet},
    system::Res,
};
use bevy::state::state::OnExit;
use systems::processing;
use winit::{event::MouseButton, keyboard::PhysicalKey};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSystemSet {
    WindowEvents,
    DeviceEvents,
}

pub struct InputModulePlugin;

impl Plugin for InputModulePlugin {
    fn build(&self, app: &mut App) {
        // resources
        app.insert_resource(InputActionMapResource::default())
            .insert_resource(ActionStateResource::default());

        app.insert_resource(Buttons::<PhysicalKey>::default())
            .insert_resource(Buttons::<MouseButton>::default())
            .insert_resource(CursorMovement::default());

        // external events (comes from the app wrapper)
        app.init_resource::<Messages<RawWindowMessage>>()
            .init_resource::<Messages<RawDeviceMessage>>();

        // internal events (an ecs system fires them)
        app.init_resource::<Messages<KeyboardInputMessage>>()
            .init_resource::<Messages<MouseMoveMessage>>()
            .init_resource::<Messages<MouseScrollMessage>>()
            .init_resource::<Messages<MouseResizeMessage>>()
            .init_resource::<Messages<MouseButtonInputMessage>>();

        // schedules
        app.add_systems(
            Update,
            (
                processing::window_events_system
                    .in_set(InputSystemSet::WindowEvents)
                    .in_set(SimulationSet::Input),
                processing::device_events_system
                    .in_set(InputSystemSet::DeviceEvents)
                    .in_set(SimulationSet::Input),
                processing::handle_resize_system
                    .after(InputSystemSet::WindowEvents)
                    .in_set(SimulationSet::Input),
                processing::update_action_state_system
                    .after(InputSystemSet::WindowEvents)
                    .after(InputSystemSet::DeviceEvents)
                    .in_set(SimulationSet::Input),
            ),
        );

        app.add_systems(
            OnExit(AppState::StartingUp),
            processing::clear_stale_input_events_system,
        );

        // INFO: -------------------------------------
        //         keybind-based actions below
        // -------------------------------------------

        // set desired cursor state on pause action
        app.insert_resource(DesiredCursorState::default())
            .add_systems(
                Update,
                toggle_cursor_system.run_if(|action_state: Res<ActionStateResource>| {
                    action_state.just_happened(SimulationAction::TogglePause)
                }),
            );

        // toggle opaque wireframe mode
        app.insert_resource(OpaqueWireframeMode::default())
            .add_systems(
                Update,
                toggle_opaque_wireframe_mode_system.run_if(
                    |action_state: Res<ActionStateResource>| {
                        action_state.just_happened(SimulationAction::ToggleOpaqueWireframeMode)
                    },
                ),
            );

        // toggle chunk borders
        app.insert_resource(ChunkBoundsToggle::default())
            .add_systems(
                Update,
                toggle_chunk_borders_system.run_if(|action_state: Res<ActionStateResource>| {
                    action_state.just_happened(SimulationAction::ToggleChunkBorders)
                }),
            );
    }
}
