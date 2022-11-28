use bevy::{
    prelude::*,
};
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::sprite::{MaterialMesh2dBundle};

pub const TEXTURE_DIMENSION: f32 = 8.0;
pub const TEXTURES: usize = 64;
pub const AIR: usize = TEXTURES - 1;

pub const DIRT: usize = 0;
pub const GRASS: usize = 1;
pub const STONE: usize = 2;

pub const CHUNK_SIZE: usize = 16;

pub const TILE_SIZE: f32 = 1.0;

#[derive(Component)]
pub struct Chunk {
    pub blocks: [[usize; CHUNK_SIZE]; CHUNK_SIZE],
    pub coordinate: Vec2,
    chunk_tile_map_builder: ChunkTileMapBuilder,
}

impl Chunk {
    pub fn new(
        cord: Vec2,
        seed: u32
    ) -> Self {
        let mut blocks = [[AIR; CHUNK_SIZE]; CHUNK_SIZE];
        let coordinate = cord;

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                blocks[x][y] = DIRT;
            }
        }

        Chunk {
            blocks,
            coordinate,
            chunk_tile_map_builder: ChunkTileMapBuilder::default()
        }
    }

    pub fn spawn(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Entity {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                self.chunk_tile_map_builder.add_tile(
                    self.coordinate,
                    Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE),
                    self.blocks[x][y]);
            }
        }

        let mesh = self.chunk_tile_map_builder.build();

        let mesh_ent = commands.spawn(MaterialMesh2dBundle  {
            mesh: meshes.add(mesh).into(),
            material: materials.add(ColorMaterial::from(asset_server.load("tiles/tiles.png"))),
            transform: Transform::from_xyz(
                self.coordinate.x * TILE_SIZE * CHUNK_SIZE as f32,
                self.coordinate.y * TILE_SIZE * CHUNK_SIZE as f32,
                0.0),
            ..Default::default()
        }).id();

        mesh_ent
    }
}

#[derive(Default, Clone)]
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
    pub fn add_tile(&mut self, chunk_coord: Vec2, tile_offset: Vec2, tile_type: usize) {
        let tile_coord = chunk_coord + tile_offset;
        let bl = [tile_coord.x, tile_coord.y, 0.0];
        let tl = [tile_coord.x, tile_coord.y + TILE_SIZE, 0.0];
        let br = [tile_coord.x + TILE_SIZE, tile_coord.y, 0.0];
        let tr = [tile_coord.x + TILE_SIZE, tile_coord.y + TILE_SIZE, 0.0];
        let vertices = [bl, tl, br, tr];

        self.vertices.extend_from_slice(&vertices);

        let mut tri_arr: [u32; 6] = [0, 2, 1, 3, 1, 2];
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

        let uvs = [
            [row * side_size, col * side_size],
            [row * side_size, col * side_size + side_size],
            [row * side_size + side_size, col * side_size],
            [row * side_size + side_size, col * side_size + side_size]
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