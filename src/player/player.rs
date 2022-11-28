use bevy::{
    prelude::*,
};

#[derive(Component)]
pub struct Player {
    pub pos: Vec2,
    vel: Vec2,
    acc: Vec2
}