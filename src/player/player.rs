use bevy::{
    prelude::*,
};

#[derive(Component)]
pub struct Player {
    pub pos: Vec2,
    pub(crate) vel: Vec2,
    pub(crate) acc: Vec2
}