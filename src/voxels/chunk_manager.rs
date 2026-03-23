use std::hash::BuildHasherDefault;

use crate::{
    advanced_rendering::texture::Texture,
    app_manager::{render_pipeline::create_render_pipeline, state::State},
    voxels::{
        chunk::{BlockData, BlockID, CHUNKSIZE, Chunk},
        mesh_gen::{Direction, VoxelMesh, VoxelMeshSide},
}};
use glam::{IVec3, Mat4, UVec3, Vec3, Vec4};
use slotmap::SlotMap;
use wgpu::{BindGroup, Buffer, CurrentSurfaceTexture, Device, RenderPipeline, SurfaceConfiguration, TextureFormat, naga::FastHashMap, util::DeviceExt};

pub const MAPSIZE: usize = 16 * 16 * 16;

pub const RENDER_DISTANCE_HOR: usize = 16;
pub const RENDER_DISTANCE_VER: usize = 8;

pub struct ChunkManager {
    pub map: FastHashMap<IVec3, BlockData>,
    pub mesh_pool: FastHashMap<IVec3,(VoxelMesh, [Buffer; 6])>,

    pub render_pipeline: RenderPipeline,
    pub camera_buffer: Buffer,
    pub chunk_mesh_buffer: Buffer,
    pub chunk_offset_buffer: Buffer,
    pub voxel_bind_group: BindGroup,
}
impl ChunkManager {
    pub fn new(device: &Device, config: &SurfaceConfiguration<>) -> Self {
        let voxel_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry { // camera
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }, wgpu::BindGroupLayoutEntry { // ChunkMesh data
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }, wgpu::BindGroupLayoutEntry { // quads
                        binding: 2,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage {read_only: true},
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
                label: None,
            });
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("camera_buffer"),
                contents: bytemuck::cast_slice(&[Mat4::IDENTITY]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let chunk_mesh_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("chunk_mesh_buffer"),
                contents: bytemuck::cast_slice(&[Vec4::ZERO]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let chunk_offset_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("chunk_offset_buffer"),
                contents: bytemuck::cast_slice(&[Vec3::ZERO; 32]),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }
        );
        
        let voxel_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &voxel_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }, wgpu::BindGroupEntry {
                    binding: 1,
                    resource: chunk_mesh_buffer.as_entire_binding(),
                }, wgpu::BindGroupEntry {
                    binding: 2,
                    resource: chunk_offset_buffer.as_entire_binding(),
                }
            ],
            label: Some("Voxel Bind Group"),
        });

        let shader = wgpu::ShaderModuleDescriptor {
            label: Some("Voxel Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/voxel.wgsl").into()),
        };
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Voxel Pipeline Layout"),
                bind_group_layouts: &[
                    Some(&voxel_bind_group_layout)
                ],
                immediate_size: 0,
            });
        let render_pipeline = create_render_pipeline(
            device,
            &render_pipeline_layout,
            config.format,
            Some(Texture::DEPTH_FORMAT),
            &[],
            shader,
        );
        let size = RENDER_DISTANCE_HOR*RENDER_DISTANCE_HOR*RENDER_DISTANCE_VER;
        Self {
            voxel_bind_group,
            camera_buffer,
            chunk_mesh_buffer,
            chunk_offset_buffer,
            render_pipeline,
            mesh_pool: FastHashMap::with_capacity_and_hasher(size, BuildHasherDefault::default()),
            map: FastHashMap::with_capacity_and_hasher(size, BuildHasherDefault::default()),
        }
    }
    pub fn add_chunk(&mut self, pos: IVec3, data: BlockData) {
        self.map.insert(pos, data);
    }
    pub fn get_block(&self, pos: IVec3) -> BlockID {
        let chunk_index: IVec3 = pos / CHUNKSIZE as i32;
        match self.map.get(&chunk_index) {
            Some(val) => {val[(pos.x & 15) as usize][(pos.y & 15) as usize]
                [(pos.z & 15) as usize]},
            None => {BlockID::Air},
        }
    }
    pub fn get_block_seperate(&self, chunk_index: &IVec3, block_index: &UVec3) -> BlockID {
        match self.map.get(chunk_index) {
            Some(val) => {val[block_index.x as usize][block_index.y as usize]
            [block_index.z as usize]},
            None => {BlockID::Air}
        }
    }

    pub fn render_world(&self, state: &State) {
        let output = match state.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(val) => {
                val
            },
            CurrentSurfaceTexture::Lost => {
                todo!()
            },
            CurrentSurfaceTexture::Occluded => {
                todo!()
            },
            CurrentSurfaceTexture::Outdated => {
                todo!()
            },
            CurrentSurfaceTexture::Suboptimal(val) => {
                val
            },
            CurrentSurfaceTexture::Timeout => {
                todo!()
            },
            CurrentSurfaceTexture::Validation => {
                todo!()
            }
        };
        // let output = state.surface.get_current_texture();
        let view = output.texture.create_view(&Default::default());
        let mut encoder = state.device.create_command_encoder(&Default::default());

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: 0.05,
                                    g: 0.05,
                                    b: 0.025,
                                    a: 0.0,
                                }
                            ),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &state.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
                // ..Default::default()
            });

            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, &self.voxel_bind_group, &[]);
            println!("{}",self.mesh_pool.len());
            for (coords, sides) in &self.mesh_pool {
                let mesh = &sides.0;
                for buffer in &sides.1 {
                    // rpass.set_vertex_buffer(0, buffer.slice(..));
                    rpass.set_bind_group(0, &self.voxel_bind_group, &[]);
                    rpass.draw(0..mesh.offset_buffer[0].len() as u32, 0..1); 
                }
            }
        }

        state.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    pub fn get_mesh(&mut self, index: &IVec3) -> &(VoxelMesh, [Buffer; 6]) {
        self.mesh_pool.get(index).unwrap()
    }
}
