use crate::{InputState, Sprite};
use cgmath::{Vector2, Vector4};
use winit::dpi::LogicalSize;

pub const CANVAS_WIDTH: f32 = 600.0;
pub const CANVAS_HEIGHT: f32 = 650.0;

const MAX_JUMP_FRAME: u32 = 200;

pub struct TextureHelper {
    size: LogicalSize<f32>,
}

impl TextureHelper {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            size: LogicalSize::new(width, height),
        }
    }

    pub fn texture_coord(&self, x: f32, y: f32, width: f32, height: f32) -> Vector4<f32> {
        (
            x / self.size.width,
            y / self.size.height,
            (x + width) / self.size.width,
            (y + height) / self.size.height,
        )
            .into()
    }
}

pub struct Rect {
    pub bottom_left: Vector2<f32>,
    pub top_right: Vector2<f32>,
}

pub struct BombJackGame {
    pub background: Sprite,
    pub jack: Sprite,
    pub game_bounds: Rect,
    pub platforms: Vec<Sprite>,
    jump_frames: u32,
}

impl BombJackGame {
    pub fn new() -> Self {
        let texture_atlas = TextureHelper::new(1162.0, 650.0);
        Self {
            background: Sprite {
                position: (0.0, 0.0).into(),
                size: (600.0, 650.0).into(),
                texture: texture_atlas.texture_coord(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT),
            },
            jack: Sprite {
                position: (300.0, 300.0).into(),
                size: (39.0, 45.0).into(),
                // Add 0.5 to X, to avoid a red line showing sometimes to the left of jack.
                texture: texture_atlas.texture_coord(600.5, 256.0, 39.0, 45.0),
            },
            game_bounds: Rect {
                bottom_left: (20.0, 20.0).into(),
                top_right: (580.0, 580.0).into(),
            },
            platforms: vec![
                Sprite {
                    position: (325.0, 459.0).into(),
                    size: (150.0, 22.0).into(),
                    texture: texture_atlas.texture_coord(780.0, 42.0, 150.0, 22.0),
                },
                Sprite {
                    position: (330.0, 59.0).into(),
                    size: (180.0, 22.0).into(),
                    texture: texture_atlas.texture_coord(600.0, 42.0, 180.0, 22.0),
                },
                Sprite {
                    position: (261.0, 199.0).into(),
                    size: (117.0, 22.0).into(),
                    texture: texture_atlas.texture_coord(930.0, 42.0, 117.0, 22.0),
                },
                Sprite {
                    position: (135.0, 389.0).into(),
                    size: (91.0, 22.0).into(),
                    texture: texture_atlas.texture_coord(1047.0, 42.0, 91.0, 22.0),
                },
                Sprite {
                    position: (75.0, 129.0).into(),
                    size: (91.0, 22.0).into(),
                    texture: texture_atlas.texture_coord(1047.0, 42.0, 91.0, 22.0),
                },
            ],
            jump_frames: 0,
        }
    }

    pub fn update(&mut self, input_state: &InputState) {
        // Update game
        let on_ground = self.jack_on_ground();

        if on_ground {
            self.jump_frames = 0;
        }

        if input_state.up_pressed
            && self.jump_frames < MAX_JUMP_FRAME
            && (self.jack.position.y + self.jack.size.height) < self.game_bounds.top_right.y
        {
            self.jump_frames += 1;
            self.jack.position.y += 1.0;
        } else if !on_ground {
            self.jack.position.y -= 1.0;
        }

        if input_state.left_pressed && self.jack.position.x > self.game_bounds.bottom_left.x {
            self.jack.position.x -= 1.0;
        }

        if input_state.right_pressed
            && (self.jack.position.x + self.jack.size.width) < self.game_bounds.top_right.x
        {
            self.jack.position.x += 1.0;
        }
    }

    fn jack_on_ground(&self) -> bool {
        if self.jack.position.y <= self.game_bounds.bottom_left.y {
            return true;
        }

        for platform in &self.platforms {
            if self.jack.position.x + self.jack.size.width >= platform.position.x
                && self.jack.position.x < platform.position.x + platform.size.width
                && self.jack.position.y <= platform.position.y + platform.size.height
                && self.jack.position.y >= platform.position.y + platform.size.height - 1.0
            {
                return true;
            }
        }
        false
    }
}
