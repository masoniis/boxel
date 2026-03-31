pub mod crosshair;

// INFO: ----------------
//         Plugin
// ----------------------

use crate::prelude::*;
use crate::simulation_world::user_interface::screens::game::crosshair::spawn_crosshair;
use bevy::app::{App, Plugin};
use bevy::state::state::OnEnter;

pub struct GameScreenPlugin;

impl Plugin for GameScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_crosshair);
    }
}
