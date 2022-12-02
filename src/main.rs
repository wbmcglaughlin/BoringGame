mod chunk;
mod player;

use bevy::{
    prelude::*,
};
use bevy::window::PresentMode;
use bevy_debug_text_overlay::OverlayPlugin;
use crate::chunk::chunk::Chunk;
use crate::chunk::chunk_handler::ChunkHandlerPlugin;
use crate::player::player::{Player, PlayerPlugin};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Boring Game".to_string(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::rgb(0.6, 0.8, 1.0)))
        .add_plugin(OverlayPlugin { font_size: 22.0, ..default() })
        .add_startup_system(setup)
        .add_plugin(ChunkHandlerPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}

#[derive(Component)]
pub struct MainCamera;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // commands
    //     .spawn(SpriteBundle {
    //         texture: asset_server.load("background/sky.png"),
    //         transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(0.08)),
    //         ..Default::default()
    //     });

    commands.spawn((Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.03,
            ..default()
        },
            ..default()
    },MainCamera));
}
