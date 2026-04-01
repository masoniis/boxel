use bevy::log::LogPlugin;
use bevy::{app::App, prelude::*, window::WindowResolution};
use boxel::ecs_core::{LoadingTracker, load_config};
use boxel::prelude::*;
use boxel::render::BoxelRenderPlugin;
use boxel::simulation::SimulationPlugin;

#[instrument(skip_all, fields(name = "main"))]
fn main() {
    attach_logger();

    // setup default bevy app
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "🅱️oxel".to_string(),
                    resolution: WindowResolution::new(1280, 720),
                    ..default()
                }),
                ..default()
            })
            .disable::<LogPlugin>(),
    );

    // load config & loading trackers into main world
    app.insert_resource(load_config());
    app.insert_resource(LoadingTracker::default());

    // initialize simulation and renderer
    app.add_plugins(SimulationPlugin);
    app.add_plugins(BoxelRenderPlugin);

    // run the app...
    app.run();
    info!("App exited safely!");
}
