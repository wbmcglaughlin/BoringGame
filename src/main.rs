mod chunk;
mod player;

use bevy::{
    prelude::*,
    render::{render_resource::WgpuFeatures},
};
use bevy::window::PresentMode;
use crate::chunk::chunk::Chunk;
use crate::chunk::chunk_handler::ChunkHandlerPlugin;
use crate::player::player::{Player, PlayerPlugin};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Boring Game".to_string(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_plugin(ChunkHandlerPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}

#[derive(Component)]
pub struct MainCamera;

fn setup(
    mut commands: Commands,
) {
    commands.spawn((Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.05,
            ..default()
        },
            ..default()
    },MainCamera));
}
