use bevy::{
    prelude::*,
};
use crate::Chunk;

pub const VISIBLE_CHUNKS: u32 = 10;

pub struct ChunkHandlerPlugin;
impl Plugin for ChunkHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkHandler {
            chunks: Vec::new()
        });
    }
}

#[derive(Resource)]
pub struct ChunkHandler {
    chunks: Vec<Chunk>
}