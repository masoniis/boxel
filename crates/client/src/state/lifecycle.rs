use crate::state::enums::{ClientAppState, ClientGameState};
use bevy::prelude::*;
use shared::load::{OnLoadComplete, master_finalize_loading_system, reset_loading_tracker_system};

pub struct ClientLifecyclePlugin;

impl Plugin for ClientLifecyclePlugin {
    fn build(&self, app: &mut App) {
        // load cleanup to run after transitions
        app.add_systems(
            OnExit(ClientAppState::StartingUp),
            reset_loading_tracker_system,
        );

        // INFO: ---------------------------
        //         state transitions
        // ---------------------------------
        app.init_state::<ClientAppState>();
        app.init_state::<ClientGameState>();

        app.add_systems(
            Update,
            (
                master_finalize_loading_system::<ClientAppState>,
                master_finalize_loading_system::<ClientGameState>,
            )
                .run_if(in_state(ClientAppState::StartingUp)),
        );

        // initial startup loading state should take us from loading
        // to running/playing once they finish.
        app.insert_resource(OnLoadComplete::new(ClientAppState::Running))
            .insert_resource(OnLoadComplete::new(ClientGameState::Playing));
    }
}
