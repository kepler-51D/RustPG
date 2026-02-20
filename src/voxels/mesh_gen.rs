use glam::{IVec3, UVec3, Vec3};
use std::array::from_fn;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::voxels::{
    chunk::{BlockData, BlockID, CHUNKSIZE},
    chunk_manager::ChunkManager,
};

pub const QUAD: [Vec3; 4] = [
    Vec3::new(-0.5, 0.5, -0.5),
    Vec3::new(0.5, 0.5, -0.5),
    Vec3::new(-0.5, 0.5, 0.5),
    Vec3::new(0.5, 0.5, 0.5),
];

#[repr(u32)]
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}
pub const DIRECTION_VECS: [Vec3; 6] = [
    Vec3::Y,
    Vec3::NEG_Y,
    Vec3::NEG_X,
    Vec3::X,
    Vec3::Z,
    Vec3::NEG_Z,
];

pub struct VoxelMeshSide {
    pub main_offset: Vec3,
    pub direction: Direction,
    pub offset_buffer: Vec<Vec3>,
}
pub struct VoxelMesh {
    pub global_offset: Vec3,
    pub offset_buffer: [Vec<Vec3>; 6],
}
impl VoxelMesh {
    pub fn new() -> Self {
        let mut offset_buffer: [Vec<Vec3>; 6] = [
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];
        Self {
            global_offset: Vec3::ZERO,
            offset_buffer,
        }
    }
    pub fn get_side(&self, direction: Direction) -> VoxelMeshSide {
        VoxelMeshSide {
            direction,
            main_offset: self.global_offset,
            offset_buffer: self.offset_buffer[direction as usize].clone(), // note: inefficient but
                                                                           // i dont care
        }
    }
}

impl ChunkManager {
    pub fn gen_mesh(&self, chunk_index: IVec3) -> VoxelMesh {
        let mut mesh: VoxelMesh = VoxelMesh::new();
        for x in 0..CHUNKSIZE {
            for y in 0..CHUNKSIZE {
                for z in 0..CHUNKSIZE {
                    let block_index: UVec3 = UVec3::new(x as u32, y as u32, z as u32);

                    if self.get_block_seperate(&chunk_index, &block_index) == BlockID::Air {
                        continue;
                    }
                    for dir in Direction::iter() {
                        if self.get_block_seperate(&chunk_index, &block_index) != BlockID::Air {
                            mesh.offset_buffer[dir as usize].push(Vec3::new(
                                block_index.x as f32,
                                block_index.y as f32,
                                block_index.z as f32,
                            ));
                        }
                    }
                }
            }
        }
        todo!()
    }
}
