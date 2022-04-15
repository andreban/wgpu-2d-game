use crate::{InputState, Sprite};

pub const CANVAS_WIDTH: f32 = 600.0;
pub const CANVAS_HEIGHT: f32 = 650.0;

pub struct BombJackGame {
    pub background: Sprite,
    pub jack: Sprite,
}

impl BombJackGame {
    pub fn new() -> Self {
        Self {
            background: Sprite {
                position: (0.0, 0.0).into(),
                size: (600.0, 650.0).into(),
                texture: (0.0, 0.0, CANVAS_WIDTH / 1162.0, CANVAS_HEIGHT / 650.0).into(),
            },
            jack: Sprite {
                position: (100.0, 100.0).into(),
                size: (39.0, 45.0).into(),
                texture: (601.0 / 1162.0, 256.0 / 650.0, 639.0 / 1162.0, 301.0 / 650.0).into(),
            },
        }
    }

    pub fn update(&mut self, input_state: &InputState) {
        // Update game
        if input_state.up_pressed {
            self.jack.position.y += 1.0;
        }

        if input_state.down_pressed {
            self.jack.position.y -= 1.0;
        }

        if input_state.left_pressed {
            self.jack.position.x -= 1.0;
        }

        if input_state.right_pressed {
            self.jack.position.x += 1.0;
        }
    }
}
