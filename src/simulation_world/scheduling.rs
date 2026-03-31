use bevy::ecs::prelude::SystemSet;

// INFO: -------------------------
//         Core scheduling
// -------------------------------

/// The sets for the startup schedule of the sim
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StartupSet {
    ResourceInitialization,
    Tasks,
}

/// The sets for the fixed schedule of the
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum FixedUpdateSet {
    /// Handle state transitions and other pre-logic tasks.
    PreUpdate,
    /// The main sim logic: player movement, AI, block breaking, etc.
    MainLogic,
}

/// The core schedule sets for the simulation world.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SimulationSet {
    /// Process raw OS input and publish game-specific events.
    Input,
    /// Handle state transitions and other pre-logic tasks.
    PreUpdate,
    /// The main game logic: player movement, AI, block breaking, etc.
    Update,
    /// Physics, collision detection, and character controller logic.
    Physics,
    /// Cleanup after physics and logic (e.g., syncing transforms).
    PostUpdate,
    /// Collect all data needed for rendering into queues/buffers.
    RenderPrep,
}
