use bevy::{
    prelude::*,
};

#[derive(Component)]
pub struct HitBox {
    offsets: [f32; 4]
}