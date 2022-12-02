use bevy::{
    prelude::*,
};
use bevy::utils::{HashMap, HashSet};
use crate::Chunk;
use crate::chunk::chunk::CHUNK_SIDE_SIZE;
use crate::player::player::{Player};

pub const VISIBLE_CHUNKS: i32 = 4;

pub struct ChunkHandlerPlugin;
impl Plugin for ChunkHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkHandler {
            chunk_coordinates: Vec::new()
        }).add_system(update_chunks)
            .add_system(remove_chunks);
    }
}

#[derive(Resource)]
pub struct ChunkHandler {
    pub chunk_coordinates: Vec<Vec2>
}

fn remove_chunks(
    mut commands: Commands,
    mut chunk_handler: ResMut<ChunkHandler>,
    mut player: Query<(&Transform, &mut Player), (With<Player>, Without<Chunk>)>,
    mut chunks: Query<(Entity, &mut Chunk), With<Chunk>>
) {
    for (transform, mut player) in player.iter_mut() {
        if player.distance_moved > CHUNK_SIDE_SIZE / 2.0 {
            player.distance_moved = 0.;

            let mut chunks_to_remove: HashSet<Entity> = HashSet::new();

            for (chunk_entity, mut chunk) in chunks.iter_mut() {
                let distance = transform.translation.distance_squared(chunk.coordinate.extend(0.0) * CHUNK_SIDE_SIZE);

                if distance > CHUNK_SIDE_SIZE * VISIBLE_CHUNKS as f32 {
                    chunks_to_remove.insert(chunk_entity);
                }
            }

            for chunk_ent in chunks_to_remove.iter() {
                commands.entity(*chunk_ent).despawn();
            }
        }
    }
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