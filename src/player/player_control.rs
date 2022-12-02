use bevy::{
    prelude::*,
};
use bevy_debug_text_overlay::screen_print;
use crate::{MainCamera, Player};
use crate::chunk::chunk::{AIR, CHUNK_SIDE_SIZE, CHUNK_SIZE, TILE_SIZE};
use crate::chunk::chunk_handler::ChunkHandler;
use crate::physics::collision::chunk_raycast;

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
    mut chunk_handler: ResMut<ChunkHandler>,
    mut players: Query<&mut Player, With<Player>>
) {
    // Iter through each player (this should only happen once).
    for mut player in players.iter_mut() {
        // Get the 4 Raycast Corners
        let bl = player.pos - PLAYER_HALF_HEIGHT;
        let br = Vec2::new(player.pos.x + PLAYER_HALF_HEIGHT, player.pos.y - PLAYER_HALF_HEIGHT);
        let tl = Vec2::new(player.pos.x - PLAYER_HALF_HEIGHT, player.pos.y + PLAYER_HALF_HEIGHT);
        let tr = player.pos + PLAYER_HALF_HEIGHT;

        // Array to store minimum distances [U, D, L, R]
        let mut raycast_minimum_distances: [f32; 4] = [CHUNK_SIDE_SIZE; 4];

        raycast_minimum_distances[1] = raycast_minimum_distances[1].min(chunk_raycast(bl, &mut chunk_handler));
        raycast_minimum_distances[1] = raycast_minimum_distances[1].min(chunk_raycast(br, &mut chunk_handler));

        player.distance_to_ground = raycast_minimum_distances[1];
    }
}

pub fn update_camera(
    transforms: Query<&mut Player, With<Player>>,
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>
) {
    for player in transforms.iter() {
        for mut camera in camera.iter_mut() {
            camera.translation.x = player.pos.x;
            camera.translation.y = player.pos.y;
        }
    }
}