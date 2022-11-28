use bevy::{
    prelude::*,
};

use noise::{NoiseFn, Perlin};
use rand::{Rng, thread_rng};

pub const TEXTURES: usize = 64;
pub const AIR: usize = TEXTURES - 1;

pub const DIRT: usize = 0;
pub const GRASS: usize = 1;
pub const STONE: usize = 2;

pub const CHUNK_SIZE: usize = 16;

#[derive(Component)]
pub struct Chunk {
    pub blocks: [[usize; CHUNK_SIZE]; CHUNK_SIZE],
    pub coordinate: Vec2
}

impl Chunk {
    pub fn new(
        cord: Vec2,
        seed: u32
    ) -> Chunk {
        let mut blocks = [[AIR; CHUNK_SIZE]; CHUNK_SIZE];
        let coordinate = cord;

        let prng = Perlin::new(seed);

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                blocks[x][y] = DIRT;
            }
        }

        Chunk {
            blocks,
            coordinate
        }
    }
}