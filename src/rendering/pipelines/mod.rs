use crate::rendering::camera2d::Camera2d;

pub mod squares;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, camera: &Camera2d) {
        let matrix = crate::rendering::OPENGL_TO_WGPU_MATRIX * camera.build_matrix();
        self.view_proj = matrix.into();
    }
}

