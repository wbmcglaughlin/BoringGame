use bevy::{
    prelude::*,
};
use crate::Chunk;
use crate::chunk::chunk::CHUNK_SIDE_SIZE;
use crate::player::player::Player;

pub const VISIBLE_CHUNKS: i32 = 4;

pub struct ChunkHandlerPlugin;
impl Plugin for ChunkHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkHandler {
            chunk_coordinates: Vec::new()
        }).add_system(update_chunks);
    }
}

#[derive(Resource)]
pub struct ChunkHandler {
    pub chunk_coordinates: Vec<Vec2>
}

fn update_chunks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    players: Query<(&Player), With<Player>>,
    mut chunk_handler: ResMut<ChunkHandler>
) {
    for player in players.iter() {
        let player_coordinate = Vec2::new((player.pos.x / CHUNK_SIDE_SIZE).floor(), (player.pos.y / CHUNK_SIDE_SIZE).floor());

        for x in (-VISIBLE_CHUNKS+1)..VISIBLE_CHUNKS {
            for y in (-VISIBLE_CHUNKS+1)..VISIBLE_CHUNKS {
                let coord = player_coordinate + Vec2::new(x as f32, y as f32);
                if !chunk_handler.chunk_coordinates.contains(&coord) {
                    let mut chunk = Chunk::new(coord, 0);
                    chunk.spawn(
                        &mut commands, &asset_server, &mut meshes, &mut materials
                    );
                    chunk_handler.chunk_coordinates.push(coord);
                }
            }
        }
    }
}