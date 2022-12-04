use std::ops::Range;
use bevy::{
    prelude::*,
};
use crate::chunk::chunk::{AIR, CHUNK_SIDE_SIZE, CHUNK_SIZE, TILE_SIZE};
use crate::chunk::chunk_handler::{ChunkHandler, update_chunks};
use crate::physics::hitbox::{HitBox, Direction};

#[derive(Component)]
pub struct CollisionDistances {
    pub distances: [f32; 4]
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_hitbox_for_collision_with_chunk.after(update_chunks));
    }
}

pub fn check_hitbox_for_collision_with_chunk(
    mut hitboxes: Query<(&Transform, &mut CollisionDistances, &HitBox)>,
    mut chunk_handler: ResMut<ChunkHandler>
) {
    for (transform, mut collision_distances, hitbox) in hitboxes.iter_mut() {
        if !hitbox.collide_with_chunks {
            continue;
        }
        let position = transform.translation;
        let position_xy = Vec2::new(position.x, position.y);
        collision_distances.distances = [CHUNK_SIDE_SIZE; 4];

        for offset in hitbox.get_offset_vectors() {
            if offset.y < 0.0 {
                let raycast = chunk_raycast(position_xy + offset, &mut chunk_handler, Direction::D);
                collision_distances.distances[Direction::D as usize]
                    = collision_distances.distances[Direction::D as usize].min(raycast);
            }
        }
    }
}

pub fn chunk_raycast(
    point: Vec2,
    chunk_handler: &mut ResMut<ChunkHandler>,
    direction: Direction
) -> f32 {
    let ray_distance = TILE_SIZE / 2.0;

    // Get chunk that point is in and retrieve chunk.
    let (chunk, x, y) = chunk_handler.get_chunk_xy(point);

    let mut x_middle = x as f32 + 0.5;
    let mut y_middle = y as f32 + 0.5;

    // Match direction to step direction
    let step: (i32, i32) = match direction {
        Direction::U => {(0, 1)}
        Direction::D => {(0, -1)}
        Direction::L => {(-1, 0)}
        Direction::R => {(1, 0)}
    };

    // Variables to hold raycast
    let mut x_step = 0 as f32;
    let mut y_step = 0 as f32;

    while
        x_step + x_middle >= 0.0 && x_step + x_middle < CHUNK_SIZE as f32 &&
        y_step + y_middle >= 0.0 && y_step + y_middle < CHUNK_SIZE as f32
    {
        if chunk.blocks[(x_step + x_middle as f32) as usize][(y_step + y_middle as f32) as usize] != AIR {
            return if x_step.abs() > y_step.abs() {
                x_step.abs()
            } else {
                y_step.abs()
            }
        }
        x_step += step.0 as f32 * ray_distance;
        y_step += step.1 as f32 * ray_distance;
    }

    let (chunk, x, y) = chunk_handler.get_chunk_xy(Vec2::new(point.x + x_middle as f32, y_step + y_middle as f32));
    x_middle = x as f32 + 0.5;
    y_middle = y as f32 + 0.5;

    return if chunk.blocks[(x_step + x_middle as f32) as usize][(y_step + y_middle as f32) as usize] != AIR {
        if x_step.abs() > y_step.abs() {
            x_step.abs().floor()
        } else {
            y_step.abs().floor()
        }
    } else {
        if x_step.abs() > y_step.abs() {
            x_step.abs().floor() + ray_distance
        } else {
            y_step.abs().floor() + ray_distance
        }
    };
}