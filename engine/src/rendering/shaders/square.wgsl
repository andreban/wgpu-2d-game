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
    @location(1) color: vec3<f32>,
    @location(2) transform_1: vec4<f32>,
    @location(3) transform_2: vec4<f32>,
    @location(4) transform_3: vec4<f32>,
    @location(5) transform_4: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
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
    out.clip_position = camera.view_proj * transform_matrix * vec4<f32>(model.position, 1.0);
    out.color = instance.color;
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
