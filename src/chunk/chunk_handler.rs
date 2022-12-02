use bevy::{
    prelude::*,
};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::HashSet;
use crate::Chunk;
use crate::chunk::chunk::{CHUNK_SIDE_SIZE, ChunkCoordinate};
use crate::player::player::{Player};

pub const VISIBLE_CHUNKS: i32 = 3;

pub struct ChunkHandlerPlugin;
impl Plugin for ChunkHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkHandler {
            chunks: Vec::new()
        }).add_system(update_chunks)
            .add_system(remove_chunks);
    }
}

#[derive(Resource)]
pub struct ChunkHandler {
    pub chunks: Vec<Chunk>
}

impl ChunkHandler {
    pub fn contains_chunk(
        &self,
        chunk_coordinate: Vec2
    ) -> bool {
        for chunk in &self.chunks {
            if chunk.coordinate == chunk_coordinate {
                return true;
            }
        }

        false
    }

    pub fn get_chunk(
        &self,
        chunk_coordinate: Vec2
    ) -> &Chunk {
        for chunk in &self.chunks {
            if chunk.coordinate == chunk_coordinate {
                return chunk;
            }
        }

        panic!("Chunk does not exist")
    }
}

fn remove_chunks(
    mut commands: Commands,
    mut chunk_handler: ResMut<ChunkHandler>,
    mut player: Query<(&Transform, &mut Player), (With<Player>, Without<ChunkCoordinate>)>,
    mut chunks: Query<(Entity, &mut ChunkCoordinate), With<ChunkCoordinate>>
) {
    for (transform, mut player) in player.iter_mut() {
        if player.distance_moved > CHUNK_SIDE_SIZE / 2.0 {
            player.distance_moved = 0.;

            let mut chunks_to_remove: HashSet<Entity> = HashSet::new();

            for (chunk_entity, mut chunk_coordinate) in chunks.iter_mut() {
                let distance = transform.translation.distance_squared(chunk_coordinate.coordinate.extend(0.0) * CHUNK_SIDE_SIZE);

                if distance > CHUNK_SIDE_SIZE * CHUNK_SIDE_SIZE * VISIBLE_CHUNKS as f32 * VISIBLE_CHUNKS as f32 {
                    chunk_handler.chunks.retain(|chunk| (*chunk).coordinate != chunk_coordinate.coordinate);
                    chunks_to_remove.insert(chunk_entity);
                }
            }

            for chunk_ent in chunks_to_remove.iter() {
                commands.entity(*chunk_ent).despawn_recursive();
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
                if !chunk_handler.contains_chunk(coord) {
                    let mut chunk = Chunk::new(coord, 0);
                    let mesh = chunk.generate_mesh();

                    let chunk_ent = commands.spawn((ChunkCoordinate {
                        coordinate: coord
                    }, MaterialMesh2dBundle  {
                        mesh: meshes.add(mesh).into(),
                        material: materials.add(ColorMaterial::from(asset_server.load("tiles/tiles.png"))),
                        transform: Transform::from_xyz(
                            coord.x * CHUNK_SIDE_SIZE,
                            coord.y * CHUNK_SIDE_SIZE,
                            0.0),
                        ..Default::default()
                    })).id();

                    chunk_handler.chunks.push(chunk);
                }
            }
        }
    }
}