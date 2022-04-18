use crate::game::{Animation, TextureHelper};
use crate::Sprite;
use cgmath::{Vector2, Vector4};
use winit::dpi::LogicalSize;
use enum_map::{enum_map, Enum, EnumMap};

#[derive(Copy, Clone, Enum, Eq, PartialEq)]
pub enum State {
    Live,
    Collected,
}

pub struct Bomb {
    pub position: Vector2<f32>,
    pub size: LogicalSize<f32>,
    pub texture: Vector4<f32>, //x0, y0, x1, y1 - or (0.0, 0.0, 1.0, 1.0)
    pub disarmed: bool,
    pub state: State,
    animations: EnumMap<State, Animation>,
    update_count: u32,
}

impl Bomb {
    pub fn new(x: f32, y: f32, texture_helper: &TextureHelper) -> Self {
        let animations = enum_map! {
            State::Live => Animation::new(vec![texture_helper.texture_coord(601.0, 112.0, 35.0, 47.0)], false),
            State::Collected  => Animation::new(vec![
                texture_helper.texture_coord(600.0, 112.0, 48.0, 48.0),
                texture_helper.texture_coord(1138.0, 42.0, 48.0, 48.0),
                texture_helper.texture_coord(677.0, 112.0, 48.0, 48.0),
                texture_helper.texture_coord(600.0, 160.0, 48.0, 48.0),
                texture_helper.texture_coord(601.0, 64.0, 34.0, 47.0),
            ], false),
        };
        Self {
            position: (x, y).into(),
            size: LogicalSize::new(36.0, 48.0),
            texture: texture_helper.texture_coord(601.0, 112.0, 35.0, 47.0),
            disarmed: false,
            state: State::Live,
            animations,
            update_count: 0,
        }
    }

    pub fn next_frame(&mut self) {
        self.update_count += 1;
        if self.update_count % 2 == 0 {
            self.animations[self.state].next_frame();
        }
    }
}

impl From<&Bomb> for Sprite {
    fn from(bomb: &Bomb) -> Self {
        Sprite {
            position: bomb.position,
            size: bomb.size,
            texture: bomb.animations[bomb.state].current_frame(),
        }
    }
}
