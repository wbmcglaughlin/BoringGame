use bevy::{
    prelude::*,
};
use crate::chunk::chunk::{AIR, TILE_SIZE};
use crate::chunk::chunk_handler::ChunkHandler;
use crate::Player;
use crate::player::player_control::PLAYER_HALF_HEIGHT;

pub fn bore(
    keyboard_input: Res<Input<KeyCode>>,
    mut chunk_handler: ResMut<ChunkHandler>,
    mut transforms: Query<&mut Player, With<Player>>,
    time: Res<Time>
) {
    let mut chunks_to_remesh = Vec::new();

    for player in transforms.iter_mut() {
        let mut player_mine_region = player.pos;
        player_mine_region.y -= PLAYER_HALF_HEIGHT + TILE_SIZE / 2.0;

        if keyboard_input.pressed(KeyCode::M) {
            let (chunk, x, y)
                = chunk_handler.get_chunk_xy(player_mine_region);

            if chunk.blocks[x][y] != AIR {
                chunk.set_block(x, y, AIR);

                chunks_to_remesh.push(chunk.coordinate);
            }
        }
    }

    chunk_handler.chunks_to_remesh.extend_from_slice(&*chunks_to_remesh);
}