use cgmath::{Vector2, Vector3, Vector4};
use winit::dpi::{LogicalSize, PhysicalSize};

pub struct Square {
    pub position: Vector2<f32>,
    pub size: PhysicalSize<f32>,
    pub color: Vector3<f32>,
}

pub struct Sprite {
    pub position: Vector2<f32>,
    pub size: LogicalSize<f32>,
    pub texture: Vector4<f32>, //x0, y0, x1, y1 - or (0.0, 0.0, 1.0, 1.0)
}
