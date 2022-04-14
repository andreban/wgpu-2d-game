use wgpu::VertexAttribute;

mod sprite;
mod square;

pub use sprite::SpritePipeline;
pub use square::SquarePipeline;

const SQUARE_VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0, 0.0],
    },
];
const SQUARE_INDICES: &[u16] = &[0, 1, 2, 2, 1, 3];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            }],
        }
    }
}
