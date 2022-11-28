use bevy::{
    prelude::*,
};
use crate::player::player_control::player_movement;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

pub fn spawn_player(
    mut commands: Commands
) {
    commands.spawn(Player {
        pos: Vec2::default(),
        vel: Vec2::default(),
        acc: Vec2::default()
    });
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
    }

    pub fn add_acc(&mut self, acc: Vec2) {
        self.acc += acc;
    }
}