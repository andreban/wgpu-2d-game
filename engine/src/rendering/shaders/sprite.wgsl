// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct InstanceInput {
    @location(1) transform_1: vec4<f32>,
    @location(2) transform_2: vec4<f32>,
    @location(3) transform_3: vec4<f32>,
    @location(4) transform_4: vec4<f32>,
    @location(5) texture_info: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let transform_matrix = mat4x4<f32>(
        instance.transform_1,
        instance.transform_2,
        instance.transform_3,
        instance.transform_4,
    );

    var out: VertexOutput;
    switch (i32(in_vertex_index)) {
        case 0: {
            out.tex_coords = vec2<f32>(instance.texture_info[0], instance.texture_info[1]);
        }
        case 1: {
            out.tex_coords = vec2<f32>(instance.texture_info[0], instance.texture_info[3]);
        }
        case 2: {
            out.tex_coords = vec2<f32>(instance.texture_info[2], instance.texture_info[1]);
        }
        default: {
            out.tex_coords = vec2<f32>(instance.texture_info[2], instance.texture_info[3]);
        }
    }

    out.clip_position = camera.view_proj * transform_matrix * vec4<f32>(model.position, 1.0);
    return out;
}

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;

@group(1) @binding(1)
var s_diffuse: sampler;

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
