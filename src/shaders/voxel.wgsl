struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct QuadInput {
    location(0) offset: vec3<f32>,
    // todo: bitpack
}
struct MeshData {
    global_offset: vec3<f32>,
    direction: u32,
}
@group(0) binding(0) var<uniform> mesh_data: MeshData; 

struct InstanceInput {
    @location(1) offset: vec4<f32>,
};

struct QuadOutput {
    @location(0) clip_position: vec4<f32>,
}

@vertex
fn vs_main(quad_instance_input: QuadInput) -> QuadOutput {
    quad_
}
