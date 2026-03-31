use bevy::ecs::prelude::*;

// INFO: ---------------------------------
//         World marker resources
// ---------------------------------------

// These are inserted into the corresponding world at runtime
// for shared systems that should have varying before.

// The state machine, for example, only should log state changed
// when it occurs in app, otherwise we get duplicate logs.
#[derive(Resource)]
pub struct SimulationWorldMarker;
#[derive(Resource)]
pub struct RenderWorldMarker;
