mod chunk;
mod player;

use bevy::{
    prelude::*,
    render::{render_resource::WgpuFeatures},
};
use bevy::window::PresentMode;
use crate::chunk::chunk::Chunk;

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
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let mut chunk_1 = Chunk::new(Vec2::default(), 0);
    chunk_1.spawn(
        &mut commands, &asset_server, &mut meshes, &mut materials
    );
}
