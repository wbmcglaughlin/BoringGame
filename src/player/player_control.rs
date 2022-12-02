use bevy::{
    prelude::*,
};
use bevy_debug_text_overlay::screen_print;
use crate::{MainCamera, Player};
use crate::chunk::chunk::{AIR, CHUNK_SIDE_SIZE};
use crate::chunk::chunk_handler::ChunkHandler;

pub const SPEED: f32 = 100.0;
pub const SIDE_SPEED_FACTOR: f32 = 1.;
pub const GRAVITY: f32 = 10.;
pub const PLAYER_HALF_HEIGHT: f32 = 0.5;

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut transforms: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>
) {
    for (mut transform, mut player) in transforms.iter_mut() {
        screen_print!("{}", player.distance_to_ground);

        let player_pos = player.pos;

        let mut side = 0f32;
        let mut up = 0f32;

        if keyboard_input.pressed(KeyCode::A) {
            side -= SPEED * SIDE_SPEED_FACTOR;
        }
        if keyboard_input.pressed(KeyCode::D) {
            side += SPEED * SIDE_SPEED_FACTOR;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            up += SPEED * SIDE_SPEED_FACTOR;
        }

        // Update the players accelerations
        player.add_acc(Vec2::new(side, up - GRAVITY));

        player.update(time.delta_seconds());

        transform.translation = player_pos.extend(1.0);
    }
}

pub fn update_distance_to_ground(
    chunk_handler: ResMut<ChunkHandler>,
    mut players: Query<&mut Player, With<Player>>
) {
    // Iter through each player (this should only happen once).
    for mut player in players.iter_mut() {
        // Players foot is PLAYER_HALF_HEIGHT units from center.
        let feet_position = player.pos.y - PLAYER_HALF_HEIGHT;

        // Get chunk that players foot is in and retrieve chunk.
        let feet_chunk = Vec2::new(
            (player.pos.x / CHUNK_SIDE_SIZE).floor(),
            (feet_position / CHUNK_SIDE_SIZE).floor()
        );
        let chunk = chunk_handler.get_chunk(feet_chunk);

        // Get x and y array positions.
        let x = (player.pos.x - chunk.coordinate.x * CHUNK_SIDE_SIZE).floor() as usize;
        let y = (feet_position - chunk.coordinate.y * CHUNK_SIDE_SIZE).floor() as usize;

        // Iterate down from current foot position to bottom of chunk.
        for y_iter in (0..y).rev() {
            // If the block is not an air block or the edge of the chunk is reached, return distance
            if chunk.blocks[x][y_iter as usize] != AIR || y_iter == 0 {
                player.distance_to_ground = feet_position - y_iter as f32;
                break
            }
        }
    }
}

pub fn update_camera(
    mut transforms: Query<(&mut Player), With<Player>>,
    mut camera: Query<(&mut Transform), (With<MainCamera>, Without<Player>)>
) {
    for player in transforms.iter() {
        for mut camera in camera.iter_mut() {
            camera.translation.x = player.pos.x;
            camera.translation.y = player.pos.y;
        }
    }
}