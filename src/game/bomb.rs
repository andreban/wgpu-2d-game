use crate::game::TextureHelper;
use crate::Sprite;
use cgmath::{Vector2, Vector4};
use winit::dpi::LogicalSize;

pub struct Bomb {
    pub position: Vector2<f32>,
    pub size: LogicalSize<f32>,
    pub texture: Vector4<f32>, //x0, y0, x1, y1 - or (0.0, 0.0, 1.0, 1.0)
    pub disarmed: bool,
}

impl Bomb {
    pub fn new(x: f32, y: f32, texture_helper: &TextureHelper) -> Self {
        Self {
            position: (x, y).into(),
            size: LogicalSize::new(36.0, 48.0),
            texture: texture_helper.texture_coord(601.0, 112.0, 35.0, 47.0),
            disarmed: false,
        }
    }
}

impl From<&Bomb> for Sprite {
    fn from(bomb: &Bomb) -> Self {
        Sprite {
            position: bomb.position,
            size: bomb.size,
            texture: bomb.texture,
        }
    }
}
