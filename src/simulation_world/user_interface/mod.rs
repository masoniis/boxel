pub mod components;
pub mod layout;
pub mod screens;
pub mod text;

// INFO: ----------------
//         Plugin
// ----------------------

use self::layout::handle_window_resize_system;
use crate::simulation_world::scheduling::RenderPrepSet;
use crate::simulation_world::user_interface::screens::{
    DebugScreenPlugin, GameScreenPlugin, LoadingScreenPlugin,
};
use bevy::app::PreStartup;
use bevy::app::{App, Plugin, PostUpdate, Update};
use bevy::ecs::prelude::*;
use {
    layout::{
        EntityToNodeMap, IsLayoutDirty, UiLayoutTree, compute_and_apply_layout_system,
        compute_ui_depth_system, handle_hierarchy_changes_system, handle_structural_changes_system,
        update_changed_styles_system,
    },
    screens::spawn_ui_root_system,
    text::setup_font_system,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // INFO: -------------------
        //         Resources
        // -------------------------

        app.world_mut().init_non_send_resource::<UiLayoutTree>();
        app.insert_resource(EntityToNodeMap::default())
            .insert_resource(IsLayoutDirty::default());

        // INFO: -----------------
        //         Plugins
        // -----------------------

        app.add_plugins((LoadingScreenPlugin, DebugScreenPlugin, GameScreenPlugin));

        // INFO: -----------------
        //         Systems
        // -----------------------

        app.add_systems(
            PreStartup,
            (setup_font_system, spawn_ui_root_system).chain(),
        );

        app.add_systems(Update, (handle_window_resize_system,));

        app.add_systems(
            PostUpdate,
            (
                (
                    handle_structural_changes_system,
                    handle_hierarchy_changes_system,
                    update_changed_styles_system,
                )
                    .chain(),
                (compute_and_apply_layout_system, compute_ui_depth_system)
                    .run_if(resource_equals(IsLayoutDirty(true)))
                    .in_set(RenderPrepSet),
            ),
        );
    }
}
