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
    let ray_distance = TILE_SIZE / 4.0;

    // Get chunk that point is in and retrieve chunk.
    let (chunk, x, y) = chunk_handler.get_chunk_xy(point);

    let mut x_chunk_pos = point.x - chunk.coordinate.x * CHUNK_SIDE_SIZE;
    let mut y_chunk_pos = point.y - chunk.coordinate.y * CHUNK_SIDE_SIZE;

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
        x_step + x_chunk_pos >= 0.0 && x_step + x_chunk_pos <= CHUNK_SIZE as f32 &&
        y_step + y_chunk_pos >= 0.0 && y_step + y_chunk_pos <= CHUNK_SIZE as f32
    {
        if chunk.blocks[(x_step + x_chunk_pos) as usize][(y_step + y_chunk_pos) as usize] != AIR {
            return raycast_result(x_step, y_step, &direction)
        }

        x_step += step.0 as f32 * ray_distance;
        y_step += step.1 as f32 * ray_distance;
    }

    let (chunk, x, y) = chunk_handler.get_chunk_xy(Vec2::new(point.x + x_chunk_pos, y_step + y_chunk_pos));
    x_chunk_pos = point.x + x_step - chunk.coordinate.x * CHUNK_SIDE_SIZE;
    y_chunk_pos = point.y + y_step - chunk.coordinate.y * CHUNK_SIDE_SIZE;

    return if chunk.blocks[(x_chunk_pos) as usize][(y_chunk_pos) as usize] != AIR {
        raycast_result(x_step, y_step, &direction)
    } else {
        raycast_result(x_step + step.0 as f32 * TILE_SIZE as f32, y_step + step.1 as f32 * TILE_SIZE as f32, &direction)
    };
}

fn raycast_result(x_step: f32, y_step: f32, dir: &Direction) -> f32 {
    return match dir {
        Direction::U => {y_step}
        Direction::D => {y_step.abs()}
        Direction::L => {x_step.abs()}
        Direction::R => {x_step}
    }
}