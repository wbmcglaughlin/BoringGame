use bevy::{
    prelude::*,
};
use bevy::render::mesh::{Indices, PrimitiveTopology};
use crate::chunk::chunk::{TEXTURE_DIMENSION, TILE_SIZE};

#[derive(Default)]
pub struct ChunkTileMapBuilder {
    vertices: Vec<[f32; 3]>,
    triangles: Vec<u32>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    face_count: u32
}

impl ChunkTileMapBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// ```
    /// Tiles are numbered from bottom left to top right starting with rows first.
    /// i.e.
    /// ... N - 2, N - 1, N
    /// ... #      #      #
    /// ... #      #      #
    /// ... 0    , 1    , 2
    /// ```
    pub fn add_tile(&mut self, tile_offset: Vec2, tile_type: usize) {
        let bl = [tile_offset.x, tile_offset.y, 0.0];
        let tl = [tile_offset.x, tile_offset.y + TILE_SIZE, 0.0];
        let br = [tile_offset.x + TILE_SIZE, tile_offset.y, 0.0];
        let tr = [tile_offset.x + TILE_SIZE, tile_offset.y + TILE_SIZE, 0.0];
        let vertices = [bl, tl, br, tr];

        self.vertices.extend_from_slice(&vertices);

        let mut tri_arr: [u32; 6] = [1, 0, 2, 1, 2, 3];
        self.triangles.extend_from_slice({
            for i in &mut tri_arr {
                *i+=4*self.face_count;
            }
            &tri_arr
        });

        for _ in 0..4 {
            self.normals.push([0.0, 0.0, 1.0]);
        }

        let row = (tile_type as f32 / TEXTURE_DIMENSION).floor();
        let col = (tile_type as f32 % TEXTURE_DIMENSION).floor();
        let side_size = 1.0 / TEXTURE_DIMENSION;

        // bl, tl, br, tr
        let uvs = [
            [col * side_size + side_size, row * side_size + side_size],
            [col * side_size, row * side_size],
            [col * side_size, row * side_size + side_size],
            [col * side_size + side_size, row * side_size],
        ];

        self.uvs.extend_from_slice(&uvs);

        self.face_count += 1;
    }

    pub fn build(&mut self) -> Mesh {
        let mut msh= Mesh::new(PrimitiveTopology::TriangleList);

        msh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices.clone());
        msh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals.clone());
        msh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs.clone());

        msh.set_indices(Some(Indices::U32(self.triangles.clone())));
        msh
    }
}