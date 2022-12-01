use bevy::{
    prelude::*,
};
use crate::player::player_control::{player_movement, update_camera};

#[derive(Component, Deref, DerefMut)]
struct PlayerAnimationTimer(Timer);

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(update_camera)
            .add_system(animate_sprite);
    }
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut PlayerAnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
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
    let player_position = Vec2::default();

    commands.spawn((
        Player {
            pos: player_position,
            vel: Vec2::default(),
            acc: Vec2::default()
        },
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(player_position.extend(1.0)),
            ..default()
        },
        PlayerAnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating))
    ));
}

#[derive(Component)]
pub struct Player {
    pub pos: Vec2,
    pub(crate) vel: Vec2,
    pub(crate) acc: Vec2
}

impl Player {
    pub fn update(&mut self, dt: f32) {
        self.vel += dt * self.acc;
        self.pos += dt * self.vel;

        self.acc = Vec2::default();
        self.vel -= self.vel * self.vel.length() * 0.8 * dt;
    }

    pub fn add_acc(&mut self, acc: Vec2) {
        self.acc += acc;
    }
}