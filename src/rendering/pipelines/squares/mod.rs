use wgpu::VertexAttribute;

pub mod square;
pub mod textured_square;

const VERTICES: &[SquareVertex] = &[
    SquareVertex {
        position: [0.0, 1.0, 0.0],
    },
    SquareVertex {
        position: [0.0, 0.0, 0.0],
    },
    SquareVertex {
        position: [1.0, 1.0, 0.0],
    },
    SquareVertex {
        position: [1.0, 0.0, 0.0],
    },
];
const INDICES: &[u16] = &[0, 1, 2, 2, 1, 3];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SquareVertex {
    position: [f32; 3],
}

impl SquareVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SquareVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            }],
        }
    }
}
