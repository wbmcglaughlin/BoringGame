use bevy::{
    prelude::*,
};


#[derive(Component)]
pub struct HitBox {
    pub(crate) offsets: [f32; 4], // U, D, L, R
    pub collide_with_chunks: bool,
}

impl HitBox {
    pub fn get_offset_vectors(
        &self,
    ) -> Vec<Vec2> {
        let mut offset_list = Vec::new();
        offset_list.push(Vec2::new(-self.offsets[Direction::L as usize], -self.offsets[Direction::D as usize]));
        offset_list.push(Vec2::new(-self.offsets[Direction::L as usize], self.offsets[Direction::U as usize]));
        offset_list.push(Vec2::new(self.offsets[Direction::R as usize], -self.offsets[Direction::D as usize]));
        offset_list.push(Vec2::new(self.offsets[Direction::R as usize], self.offsets[Direction::U as usize]));

        offset_list
    }
}

#[repr(usize)]
pub enum Direction {
    U = 0,
    D,
    L,
    R
}