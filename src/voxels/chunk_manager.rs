use std::hash::BuildHasherDefault;

use crate::voxels::{
    chunk::{BlockData, BlockID, Chunk, CHUNKSIZE},
    mesh_gen::VoxelMesh,
};
use glam::{IVec3, UVec3};
use wgpu::naga::FastHashMap;

pub const MAPSIZE: usize = 16 * 16 * 16;

pub struct ChunkManager {
    pub map: FastHashMap<IVec3, BlockData>,
}
impl ChunkManager {
    pub fn new() -> Self {
        let shader = wgpu::ShaderModuleDescriptor {
            label: Some("Voxel Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/voxel.wgsl").into()),
        };
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Voxel Pipeline Layout"),
                bind_group_layouts: &[
                    //&camera_bind_group_layout,
                    //&texture_bind_group_layout,
                    //&light_bind_group_layout,
                ],
                immediate_size: 0,
                // push_constant_ranges: &[],
            });
        let render_pipeline = create_render_pipeline(
            &device,
            &render_pipeline_layout,
            config.format,
            Some(Texture::DEPTH_FORMAT),
            &[
                //Vertex::desc(),
                //InstanceRaw::desc()
            ],
            shader,
        );
        Self {
            map: FastHashMap::with_capacity_and_hasher(MAPSIZE, BuildHasherDefault::default()),
        }
    }
    pub fn get_block(&self, pos: IVec3) -> BlockID {
        let chunk_index: IVec3 = pos / CHUNKSIZE as i32;
        self.map.get(&chunk_index).unwrap()[(pos.x % 16) as usize][(pos.y % 16) as usize]
            [(pos.z % 16) as usize]
    }
    pub fn get_block_seperate(&self, chunk_index: &IVec3, block_index: &UVec3) -> BlockID {
        self.map.get(chunk_index).unwrap()[block_index.x as usize][block_index.y as usize]
            [block_index.z as usize]
    }
    pub fn render_world() {
        
    }
}
