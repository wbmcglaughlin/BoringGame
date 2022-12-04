use bevy::{
    prelude::*,
};
use bevy_debug_text_overlay::screen_print;
use crate::{MainCamera, Player};
use crate::chunk::chunk::{AIR, CHUNK_SIDE_SIZE, CHUNK_SIZE, TILE_SIZE};
use crate::chunk::chunk_handler::ChunkHandler;
use crate::physics::collision::{chunk_raycast, CollisionDistances};

pub const SPEED: f32 = 100.0;
pub const SIDE_SPEED_FACTOR: f32 = 1.;
pub const GRAVITY: f32 = 100.;
pub const PLAYER_HALF_HEIGHT: f32 = 0.5;

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut transforms: Query<(&mut Transform, &mut Player, &CollisionDistances), With<Player>>,
    time: Res<Time>
) {
    for (mut transform, mut player, collision_distances) in transforms.iter_mut() {
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
            up += 3. * GRAVITY;
        }

        // Update the players accelerations
        player.add_acc(Vec2::new(side, up - GRAVITY), collision_distances.distances);

        player.update(time.delta_seconds(), collision_distances.distances);

        transform.translation = player_pos.extend(1.0);
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