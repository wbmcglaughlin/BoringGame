use bevy::{
    prelude::*,
};
use crate::chunk::meshing::ChunkTileMapBuilder;

pub const TEXTURE_DIMENSION: f32 = 8.0;
pub const TEXTURES: usize = 64;
pub const AIR: usize = TEXTURES - 1;

pub const DIRT: usize = 0;
pub const GRASS: usize = 1;
pub const STONE: usize = 2;

pub const CHUNK_SIZE: usize = 16;

pub const TILE_SIZE: f32 = 1.0;
pub const CHUNK_SIDE_SIZE: f32 = TILE_SIZE * CHUNK_SIZE as f32;

pub const GROUND_LEVEL: f32 = 0.0;
pub const STONE_LEVEL: f32 = -10.0;

#[derive(Component)]
pub struct Chunk {
    pub blocks: [[usize; CHUNK_SIZE]; CHUNK_SIZE],
    pub coordinate: Vec2,
    chunk_tile_map_builder: ChunkTileMapBuilder,
}

#[derive(Component)]
pub struct ChunkCoordinate {
    pub coordinate: Vec2,
}


impl Chunk {
    pub fn new(
        cord: Vec2,
        seed: u32
    ) -> Self {
        let mut blocks = [[AIR; CHUNK_SIZE]; CHUNK_SIZE];
        let coordinate = cord;

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let level = coordinate.y * CHUNK_SIDE_SIZE + y as f32 * TILE_SIZE;
                if level < GROUND_LEVEL {
                    if level < STONE_LEVEL {
                        blocks[x][y] = STONE;
                    } else {
                        blocks[x][y] = DIRT;
                    }
                }
                if coordinate.y * CHUNK_SIDE_SIZE + y as f32 * TILE_SIZE == GROUND_LEVEL - 1.0 {
                    blocks[x][y] = GRASS;
                }
            }
        }

        Chunk {
            blocks,
            coordinate,
            chunk_tile_map_builder: ChunkTileMapBuilder::default()
        }
    }

    pub fn generate_mesh (
        &mut self,
    ) -> Mesh {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                self.chunk_tile_map_builder.add_tile(
                    Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE),
                    self.blocks[x][y]);
            }
        }

        let mesh = self.chunk_tile_map_builder.build();

        mesh
    }
}

