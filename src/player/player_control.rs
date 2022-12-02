use bevy::{
    prelude::*,
};
use crate::{MainCamera, Player};
use crate::chunk::chunk::{AIR, CHUNK_SIDE_SIZE};
use crate::chunk::chunk_handler::ChunkHandler;

pub const SPEED: f32 = 100.0;
pub const SIDE_SPEED_FACTOR: f32 = 1.;
pub const GRAVITY: f32 = 3.;

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut transforms: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>
) {
    for (mut transform, mut player) in transforms.iter_mut() {
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
    mut players: Query<(&Transform, &mut Player), With<Player>>
) {
    for (transform, mut player) in players.iter_mut() {
        let feet_position = player.pos.y - 0.5;
        let feet_chunk = Vec2::new((player.pos.x / CHUNK_SIDE_SIZE).floor(), (feet_position / CHUNK_SIDE_SIZE).floor());

        let chunk = chunk_handler.get_chunk(feet_chunk);

        let x = (player.pos.x - chunk.coordinate.x * CHUNK_SIDE_SIZE).floor() as usize;
        let y = (feet_position - chunk.coordinate.y * CHUNK_SIDE_SIZE).floor() as usize;

        for y_iter in (0..y).rev() {
            if chunk.blocks[x][y_iter] != AIR {
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