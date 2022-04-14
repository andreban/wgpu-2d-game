use cgmath::Vector2;
use winit::dpi::PhysicalSize;

pub struct Camera2d {
    position: Vector2<f32>,
    size: PhysicalSize<f32>,
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
