use cgmath::{Vector2, Vector3};
use winit::dpi::PhysicalSize;

pub struct Quad {
    pub position: Vector2<f32>,
    pub size: PhysicalSize<f32>,
    pub color: Vector3<f32>,
}

