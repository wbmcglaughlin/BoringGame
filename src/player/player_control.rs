use bevy::{
    prelude::*,
};
use crate::{MainCamera, Player};

pub const SPEED: f32 = 10.0;
pub const SIDE_SPEED_FACTOR: f32 = 1.;

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut transforms: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>
) {
    for (mut transform, mut player) in transforms.iter_mut() {
        let player_pos = player.pos;

        let mut forward = 0f32;
        let mut side = 0f32;
        let mut up = 0f32;

        if keyboard_input.pressed(KeyCode::A) {
            side -= SPEED * SIDE_SPEED_FACTOR;
        }
        if keyboard_input.pressed(KeyCode::D) {
            side += SPEED * SIDE_SPEED_FACTOR;
        }

        // Update the players accelerations
        player.add_acc(Vec2::new(side, 0.0));

        player.update(time.delta_seconds());

        transform.translation = player_pos.extend(1.0);
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