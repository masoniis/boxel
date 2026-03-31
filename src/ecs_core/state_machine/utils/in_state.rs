use crate::ecs_core::state_machine::{State, resources::CurrentState};
use bevy::ecs::prelude::*;

pub fn in_state<T: State>(check_state: T) -> impl Fn(Res<CurrentState<T>>) -> bool {
    move |current_state: Res<CurrentState<T>>| current_state.val == check_state
}
