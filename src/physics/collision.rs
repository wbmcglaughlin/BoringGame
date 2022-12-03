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
            let raycast = chunk_raycast(position_xy + offset, &mut chunk_handler);
            collision_distances.distances[Direction::D as usize]
                = collision_distances.distances[Direction::D as usize].min(raycast);
        }
    }
}

pub fn chunk_raycast(
    point: Vec2,
    chunk_handler: &mut ResMut<ChunkHandler>
) -> f32 {
    // Get chunk that players foot is in and retrieve chunk.
    let (chunk, x, y) = chunk_handler.get_chunk_xy(point);

    let distance_from_chunk_bottom = point.y - chunk.coordinate.y * CHUNK_SIDE_SIZE;

    // Iterate down from current foot position to bottom of chunk.
    for chunk_y_coordinate in (0..y + 1).rev() {
        // If the block is not an air block
        if chunk.blocks[x][chunk_y_coordinate as usize] != AIR {
            return distance_from_chunk_bottom - chunk_y_coordinate as f32 - 1.0;
        }
    }
    let (chunk, x, _) = chunk_handler.get_chunk_xy(Vec2::new(point.x, point.y - CHUNK_SIDE_SIZE / 2.0));

    return if chunk.blocks[x][CHUNK_SIZE - 1] == AIR {
        distance_from_chunk_bottom + TILE_SIZE
    } else {
        distance_from_chunk_bottom
    }
}