// constant data
    /* a quad should be rendered in the following order:
        0,1,2, = first triangle
        2,1,3 = second triangle
    */

    // doing rotations at compile time would save a bunch of multiplying at compile time
    // however would mean 24 extra bytes stored.
    // in hindsight this is insignificiant
    const UP_QUAD = array<vec3<f32>,4>(
        vec3<f32>(-0.5, 0.5, -0.5),
        vec3<f32>(0.5, 0.5, -0.5),
        vec3<f32>(-0.5, 0.5, 0.5),
        vec3<f32>(0.5, 0.5, 0.5),
    );
    const ROTATIONS = array<mat3x3<f32>,6>(
        mat3x3<f32>( // up
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ),
        mat3x3<f32>( // down
            -1.0,  0.0,  0.0,
             0.0, -1.0,  0.0,
             0.0,  0.0, -1.0,
        ),

        mat3x3<f32>( // left
            0.0,  1.0, 0.0,
            -1.0, 0.0, 0.0,
            0.0,  0.0, 1.0,
        ),
        mat3x3<f32>( // right
            0.0, -1.0, 0.0,
            1.0,  0.0, 0.0,
            0.0,  0.0, 1.0,
        ),

        mat3x3<f32>( // front
            1.0, 0.0,  0.0,
            0.0, 0.0, -1.0,
            0.0, 1.0,  0.0,
        ),
        mat3x3<f32>( // back
            1.0,  0.0, 0.0,
            0.0,  0.0, 1.0,
            0.0, -1.0, 0.0,
        ),
    );

// uniform data
    struct CameraUniform {
        // view_pos: vec4<f32>,
        view_proj: mat4x4<f32>,
    };
    @group(0) @binding(0)
    var<uniform> camera: CameraUniform;
    
    struct ChunkMeshInput { // gives chunk mesh data
        // chunk_pos: vec3<f32>,
        chunk_pos_x: f32,
        chunk_pos_y: f32,
        chunk_pos_z: f32,
        
        orientation: u32,
        /*
        0: up
        1: down
        2: left
        3: right
        4: forward
        5: back
        */
    }
    @group(0) @binding(1)
    var<uniform> chunk_mesh_data: ChunkMeshInput;

    struct Quad {
        @location(0) offset: vec3<f32>,
    }

    @group(0) @binding(2)
    var<storage,read> quads: array<Quad>;

// vertex data input
    struct QuadInput { // gives one corner of the quad per thread
        // data per thread
        @location(0) corner_index: u32,
        // todo: bitpack
    }

// instance data input
    struct QuadInstanceInput { // gives offset of quad
        // data per group of 4 threads
        @location(1) offset: vec3<f32>,

        /*
        each axis number of the offset vector can only be between 0 and 31 (inclusive)
        so can be represented with only 5 bits
        */
    }


// vertex shader output
    struct QuadOutput { // gives the global position of the corner of the quad
        @builtin(position) clip_position: vec4<f32>,
    }


// shader code
    @vertex
    fn vs_main(@builtin(vertex_index) index: u32) -> QuadOutput {
        let global_pos = quads[index & 4294967292] // not 3
            .offset +
            UP_QUAD[index & 3] *
            ROTATIONS[chunk_mesh_data.orientation];

        var out: QuadOutput;
        out.clip_position = camera.view_proj * vec4<f32>(global_pos, 1.0);
        return out;
    }

    @fragment
    fn fs_main(pos: QuadOutput) -> @location(0) vec4<f32> {

        return vec4<f32>(1.0,1.0,1.0,1.0); // todo: colour
    }