use crate::ecs_core::state_machine::State;
use bevy::ecs::prelude::*;
use bevy::state::state::States;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(States, Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
}
impl State for GameState {}
