use cgmath::Vector2;
use winit::dpi::PhysicalSize;
use cgmath::SquareMatrix;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn update_view_proj(&mut self, camera: &Camera2d) {
        let matrix = crate::rendering::OPENGL_TO_WGPU_MATRIX * camera.build_matrix();
        self.view_proj = matrix.into();
    }
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }
}

pub struct Camera2d {
    pub position: Vector2<f32>,
    pub size: PhysicalSize<f32>,
}

impl Camera2d {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            position: Vector2::new(0.0, 0.0),
            size: (width, height).into(),
        }
    }

    pub fn build_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(
            (self.position.x, self.position.y, 1.0).into(),
            (self.position.x, self.position.y, 0.0).into(),
            cgmath::Vector3::unit_y(),
        );
        let proj = cgmath::ortho(0.0, self.size.width, 0.0, self.size.height, 0.1, 100.0);
        proj * view
    }
}
