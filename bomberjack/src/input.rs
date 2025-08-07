use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{Key, NamedKey},
};

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

    pub fn update(&mut self, input: &KeyEvent) {
        match input.logical_key {
            Key::Named(NamedKey::ArrowUp) => {
                self.up_pressed = input.state.eq(&ElementState::Pressed)
            }
            Key::Named(NamedKey::ArrowDown) => {
                self.down_pressed = input.state.eq(&ElementState::Pressed)
            }
            Key::Named(NamedKey::ArrowLeft) => {
                self.left_pressed = input.state.eq(&ElementState::Pressed)
            }
            Key::Named(NamedKey::ArrowRight) => {
                self.right_pressed = input.state.eq(&ElementState::Pressed)
            }
            _ => {}
        }
    }
}
