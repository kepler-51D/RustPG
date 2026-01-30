use bytemuck::NoUninit;
use wgpu::{Device, util::DeviceExt};

pub fn vec_to_buffer<T: NoUninit>(vec: Vec<T>,label: String, device: &Device) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
        label: Some(&label),
        contents: bytemuck::cast_slice(&vec),
        usage: wgpu::BufferUsages::VERTEX,
    })
}