use crate::game::{Animation, TextureHelper};
use crate::Sprite;
use cgmath::Vector2;
use enum_map::{enum_map, Enum, EnumMap};
use winit::dpi::LogicalSize;

#[derive(Copy, Clone, Enum, Eq, PartialEq)]
pub enum Direction {
    Idle,
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
}

pub struct Jack {
    pub position: Vector2<f32>,
    pub size: LogicalSize<f32>,
    pub thrust: f32,
    pub direction: Direction,
    texture_map: EnumMap<Direction, Animation>,
}

impl Jack {
    pub fn new(texture_helper: &TextureHelper) -> Self {
        let texture_map = enum_map! {
            Direction::Idle => Animation::new(vec![texture_helper.texture_coord(601.0, 256.0, 39.0, 45.0)], false),
            Direction::Up => Animation::new(vec![texture_helper.texture_coord(600.0, 208.0, 40.0, 48.0)], false),
            Direction::Down => Animation::new(vec![texture_helper.texture_coord(636.0, 64.0, 40.0, 48.0)], false),
            Direction::Left => Animation::new(vec![
                texture_helper.texture_coord(600.0, 301.0, 40.0, 48.0),
                texture_helper.texture_coord(639.0, 256.0, 40.0, 48.0),
            ], true),
            Direction::Right => Animation::new(vec![
                texture_helper.texture_coord(640.0, 208.0, 40.0, 48.0),
                texture_helper.texture_coord(648.0, 160.0, 40.0, 48.0),
            ], true),
            Direction::UpRight => Animation::new(vec![texture_helper.texture_coord(676.0, 64.0, 40.0, 48.0)], false),
            Direction::UpLeft => Animation::new(vec![texture_helper.texture_coord(636.0, 112.0, 40.0, 48.0)], false),
        };

        Self {
            position: (300.0, 300.0).into(),
            size: (39.0, 45.0).into(),
            thrust: 0.0,
            direction: Direction::Idle,
            texture_map,
        }
    }

    pub fn next_frame(&mut self) {
        self.texture_map[self.direction].next_frame();
    }
}

impl From<&Jack> for Sprite {
    fn from(jack: &Jack) -> Self {
        Sprite {
            position: jack.position,
            size: jack.size,
            texture: jack.texture_map[jack.direction].current_frame(),
        }
    }
}
