use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

pub struct InputState {
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
        }
    }

    pub fn update(&mut self, input: &KeyboardInput) {
        match input.virtual_keycode {
            Some(VirtualKeyCode::Up) => self.up_pressed = input.state.eq(&ElementState::Pressed),
            Some(VirtualKeyCode::Down) => {
                self.down_pressed = input.state.eq(&ElementState::Pressed)
            }
            Some(VirtualKeyCode::Left) => {
                self.left_pressed = input.state.eq(&ElementState::Pressed)
            }
            Some(VirtualKeyCode::Right) => {
                self.right_pressed = input.state.eq(&ElementState::Pressed)
            }
            _ => {}
        }
    }
}
