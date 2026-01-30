use glam::{Vec2, Vec3};

use crate::advanced_rendering::render_vertex::Vertex;

#[derive(Clone, Copy)]
pub struct Quad {
    pub data: [Vertex; 4],
}
pub const RENDER_DISTANCE: u32 = 8;
pub const TOP_QUAD: Quad = Quad {data: [
    Vertex {pos:Vec3{x:0.0,y:1.0,z:1.0}, texture_coords: Vec2{x:0.0,y:1.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:1.0,y:1.0,z:1.0}, texture_coords: Vec2{x:1.0,y:1.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:1.0,y:1.0,z:0.0}, texture_coords: Vec2{x:1.0,y:0.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:0.0,y:1.0,z:0.0}, texture_coords: Vec2{x:0.0,y:0.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}}
]};
pub const BOTTOM_QUAD: Quad = Quad {data: [
    Vertex {pos:Vec3{x:0.0,y:0.0,z:1.0}, texture_coords: Vec2{x:0.0,y:1.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:1.0,y:0.0,z:1.0}, texture_coords: Vec2{x:1.0,y:1.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:1.0,y:0.0,z:0.0}, texture_coords: Vec2{x:1.0,y:0.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:0.0,y:0.0,z:0.0}, texture_coords: Vec2{x:0.0,y:0.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}}
]};
pub const LEFT_QUAD: Quad = Quad {data: [
    Vertex {pos:Vec3{x:0.0,y:0.0,z:0.0}, texture_coords: Vec2{x:0.0,y:0.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:0.0,y:0.0,z:1.0}, texture_coords: Vec2{x:0.0,y:1.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:0.0,y:1.0,z:1.0}, texture_coords: Vec2{x:1.0,y:1.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:0.0,y:1.0,z:0.0}, texture_coords: Vec2{x:1.0,y:0.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}}
]};
pub const RIGHT_QUAD: Quad = Quad {data: [
    Vertex {pos:Vec3{x:1.0,y:0.0,z:0.0}, texture_coords: Vec2{x:0.0,y:0.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:1.0,y:0.0,z:1.0}, texture_coords: Vec2{x:0.0,y:1.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:1.0,y:1.0,z:1.0}, texture_coords: Vec2{x:1.0,y:1.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:1.0,y:1.0,z:0.0}, texture_coords: Vec2{x:1.0,y:0.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}}
]};
pub const BACK_QUAD: Quad = Quad {data: [
    Vertex {pos:Vec3{x:0.0,y:0.0,z:0.0}, texture_coords: Vec2{x:0.0,y:0.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:1.0,y:0.0,z:0.0}, texture_coords: Vec2{x:1.0,y:0.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:1.0,y:1.0,z:0.0}, texture_coords: Vec2{x:1.0,y:1.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:0.0,y:1.0,z:0.0}, texture_coords: Vec2{x:0.0,y:1.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}}
]};
pub const FRONT_QUAD: Quad = Quad {data: [
    Vertex {pos:Vec3{x:0.0,y:0.0,z:1.0}, texture_coords: Vec2{x:0.0,y:0.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:1.0,y:0.0,z:1.0}, texture_coords: Vec2{x:1.0,y:0.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:1.0,y:1.0,z:1.0}, texture_coords: Vec2{x:1.0,y:1.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}},
    Vertex {pos:Vec3{x:0.0,y:1.0,z:1.0}, texture_coords: Vec2{x:0.0,y:1.0}, normal:Vec3{x:0.0,y:0.0,z:1.0}, tangent: Vec3{x:1.0,y:0.0,z:0.0}, bitangent: Vec3{x:0.0,y:1.0,z:0.0}}
]};