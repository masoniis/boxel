use crate::prelude::*;
use crate::simulation_world::input::messages::RawDeviceMessage;
use bevy::ecs::message::Messages;
use bevy::ecs::prelude::*;

/// Clears any input events that accumulated, likely during the loading screen or something.
#[instrument(skip_all)]
pub fn clear_stale_input_events_system(mut device_events: ResMut<Messages<RawDeviceMessage>>) {
    info!("Clearing accumulated input events...");
    device_events.clear();
}
