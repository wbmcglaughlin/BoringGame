use bevy::{
    prelude::*,
};
use bevy_debug_text_overlay::screen_print;
use crate::chunk::chunk::CHUNK_SIDE_SIZE;
use crate::physics::collision::CollisionDistances;
use crate::player::boring::bore;
use crate::player::player_control::{GRAVITY, player_movement, update_camera};
use crate::physics::hitbox::{HitBox, Direction};

#[derive(Component, Deref, DerefMut)]
struct PlayerAnimationTimer(Timer);

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(bore)
            .add_system(player_movement)
            .add_system(update_camera)
            .add_system(animate_sprite);
    }
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &Player,
        &mut PlayerAnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (player, mut timer,
        mut sprite, texture_atlas_handle) in &mut query {
        // Check if the player is moving.
        if player.vel.x.abs() > 0. {
            // Check which direction player is moving in.
            if player.vel.x < 0. {
                sprite.flip_x = true;
            } else {
                sprite.flip_x = false;
            }

            // Tick animation timer
            timer.tick(time.delta());
            if timer.just_finished() {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
        } else {
            sprite.index = 0;
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load texture for player
    let texture_handle = asset_server.load("sprites/player/player_walk.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle,
                                                Vec2::new(16.0, 16.0),
                                                3, 1,
                                                None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Set player position
    let player_position = Vec2::new(0.0, 2.0);

    let player_entity = commands.spawn((
        Player {
            pos: player_position,
            vel: Vec2::default(),
            acc: Vec2::default(),
            distance_moved: 0.0,
        },
        HitBox {
          offsets: [0.5, 0.5, 0.24, 0.24],
            collide_with_chunks: true,
        },
        CollisionDistances {
            distances: [CHUNK_SIDE_SIZE, CHUNK_SIDE_SIZE, CHUNK_SIDE_SIZE, CHUNK_SIDE_SIZE]
        },
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(
                player_position.extend(1.0))
                .with_scale(Vec3::splat(1. / 16.)),
            ..default()
        },
        PlayerAnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating))
    )).id();
}

#[derive(Component)]
pub struct Player {
    pub pos: Vec2,
    pub(crate) vel: Vec2,
    pub(crate) acc: Vec2,

    pub distance_moved: f32,
}

impl Player {
    pub fn update(&mut self, dt: f32, distances: [f32; 4]) {
        self.vel += dt * self.acc;

        if distances[Direction::D as usize] <= 0.0 && self.vel.y * dt <= 0.0 && self.acc.y * dt <= 0.0 {
            self.vel.y = 0.;
        }

        let mut distance = self.vel * dt;
        if -distances[Direction::D as usize] > distance.y && distance.y < 0.0 {
            distance.y = -distances[Direction::D as usize];
        }
        self.pos += distance;

        if distances[Direction::D as usize] <= 0.0 && self.vel.y * dt <= 0.0 && self.acc.y * dt <= 0.0 {
            self.pos.y = (self.pos.y + 0.5).round() - 0.5;
        }

        self.distance_moved += (dt * self.vel).length();

        self.vel -= self.vel * self.vel.length() * 0.9 * dt;

        if self.vel.x.abs() < 2.0 && self.acc.x.abs() == 0. {
            self.vel.x = 0.0
        }

        self.acc = Vec2::default();
    }

    pub fn add_acc(&mut self, acc: Vec2, distances: [f32; 4]) {
        let mut accel = acc.clone();
        if distances[Direction::D as usize] > 0.0 {
            accel.y = -GRAVITY;
        }
        self.acc += accel;
    }
}