use std::hash::BuildHasherDefault;

use crate::{
    advanced_rendering::texture::Texture,
    app_manager::render_pipeline::create_render_pipeline,
    voxels::{
        chunk::{BlockData, BlockID, CHUNKSIZE, Chunk},
        mesh_gen::{Direction, VoxelMesh, VoxelMeshSide},
}};
use glam::{IVec3, UVec3};
use slotmap::SlotMap;
use wgpu::{Device, SurfaceConfiguration, TextureFormat, naga::FastHashMap, util::DeviceExt};

pub const MAPSIZE: usize = 16 * 16 * 16;

pub const RENDER_DISTANCE_HOR: usize = 16;
pub const RENDER_DISTANCE_VER: usize = 8;

pub struct ChunkManager {
    pub map: FastHashMap<IVec3, BlockData>,
    pub mesh_pool: FastHashMap<IVec3,[VoxelMeshSide; 6]>,
}
impl ChunkManager {
    pub fn new(device: &Device, config: &SurfaceConfiguration<>) -> Self {
        let voxel_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: None,
            });
        // let voxel_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     layout: &voxel_bind_group_layout,
        //     entries: &[wgpu::BindGroupEntry {
        //         binding: 0,
        //         resource: light_buffer.as_entire_binding(),
        //     }],
        //     label: None,
        // });

        let shader = wgpu::ShaderModuleDescriptor {
            label: Some("Voxel Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/voxel.wgsl").into()),
        };
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Voxel Pipeline Layout"),
                bind_group_layouts: &[
                    &voxel_bind_group_layout
                ],
                immediate_size: 0,
                // push_constant_ranges: &[],
            });
        let render_pipeline = create_render_pipeline(
            device,
            &render_pipeline_layout,
            config.format,
            Some(Texture::DEPTH_FORMAT),
            &[
                //Vertex::desc(),
                //InstanceRaw::desc()
            ],
            shader,
        );
        let size = RENDER_DISTANCE_HOR*RENDER_DISTANCE_HOR*RENDER_DISTANCE_VER;
        Self {
            mesh_pool: FastHashMap::with_capacity_and_hasher(size, BuildHasherDefault::default()),
            map: FastHashMap::with_capacity_and_hasher(size, BuildHasherDefault::default()),
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
    pub fn get_mesh(&mut self, index: &IVec3) -> &[VoxelMeshSide; 6] {
        self.mesh_pool.get(index).unwrap()
    }
    pub fn render_chunk(&mut self, device: &Device,camera_bind_group: &wgpu::BindGroup, index: &IVec3, side: Direction) {
        let mesh = &self.get_mesh(index)[side as usize];
        
        // set up instance buffer
        // let instance_buffer = &mesh[side as usize].offset_buffer;
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Voxel Instance Buffer"),
                contents: bytemuck::cast_slice(&mesh.offset_buffer),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        // set up uniforms

    }
}
