mod chunk;
mod player;

use bevy::{
    prelude::*,
    render::{render_resource::WgpuFeatures},
};
use bevy::window::PresentMode;
use crate::chunk::chunk::Chunk;
use crate::chunk::chunk_handler::ChunkHandlerPlugin;
use crate::player::player::Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Boring Game".to_string(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .insert_resource(Msaa { samples: 1 })
        .add_startup_system(setup)
        .add_plugin(ChunkHandlerPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Player {
        pos: Vec2::default(),
        vel: Vec2::default(),
        acc: Vec2::default()
    });
}
