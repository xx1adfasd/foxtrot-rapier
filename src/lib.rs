#![feature(stmt_expr_attributes)]
#![feature(let_chains)]
#![feature(fs_try_exists)]
#![feature(never_type)]

mod bevy_config;
pub mod dev;
mod file_system_interaction;
mod level_design;
mod menu;
mod movement_gameplay;
mod shader;
mod util;
mod world_interaction;

use crate::bevy_config::BevyConfigPlugin;
use crate::dev::DevPlugin;
use crate::file_system_interaction::FileSystemInteractionPlugin;
use crate::level_design::LevelDesignPlugin;
use crate::menu::MenuPlugin;
use crate::movement_gameplay::MovementGameplayPlugin;
use crate::shader::ShaderPlugin;
use crate::world_interaction::WorldInteractionPlugin;
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(BevyConfigPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(MovementGameplayPlugin)
            .add_plugin(WorldInteractionPlugin)
            .add_plugin(LevelDesignPlugin)
            .add_plugin(FileSystemInteractionPlugin)
            .add_plugin(ShaderPlugin)
            .add_plugin(DevPlugin);
    }
}
