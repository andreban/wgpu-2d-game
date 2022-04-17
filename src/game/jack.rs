use crate::Sprite;
use cgmath::{Vector2, Vector4};
use winit::dpi::LogicalSize;

pub struct Jack {
    pub position: Vector2<f32>,
    pub size: LogicalSize<f32>,
    pub texture: Vector4<f32>, //x0, y0, x1, y1 - or (0.0, 0.0, 1.0, 1.0)
    pub thrust: f32,
}

impl From<&Jack> for Sprite {
    fn from(jack: &Jack) -> Self {
        Sprite {
            position: jack.position,
            size: jack.size,
            texture: jack.texture,
        }
    }
}
