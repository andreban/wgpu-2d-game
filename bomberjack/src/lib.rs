// Copyright 2022 Andre Cipriani Bandarra
//
// SPDX-License-Identifier: Apache-2.0

mod game;
mod input;

use std::sync::Arc;

use winit::dpi::LogicalSize;
use winit::{application::ApplicationHandler, event::*, event_loop::EventLoop, window::Window};

use engine::rendering::shapes::Sprite;
use engine::rendering::{Canvas, Graphics};
use game::BombJackGame;
use input::InputState;

struct Game {
    input_state: InputState,
    graphics: Option<Graphics>,
    game: Option<BombJackGame>,
    window: Option<Arc<Window>>,
}

impl ApplicationHandler for Game {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_resizable(false)
                    .with_inner_size(LogicalSize::new(game::CANVAS_WIDTH, game::CANVAS_HEIGHT)),
            )
            .unwrap();
        let window = Arc::new(window);
        let texture_bytes = include_bytes!("assets/texture.png");
        let mut graphics =
            pollster::block_on(Graphics::new(window.clone(), texture_bytes, 600.0, 650.0)).unwrap();
        self.game = Some(BombJackGame::new(&mut graphics.font_system));
        self.graphics = Some(graphics);
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let (Some(graphics), Some(game)) = (&mut self.graphics, &mut self.game) {
                    let mut canvas = Canvas::new(graphics);
                    game.update(&self.input_state);
                    game.render(&mut canvas);
                    if let Err(e) = canvas.done() {
                        eprintln!("Render error: {e}");
                    }
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.input_state.update(&event);
            }
            _ => {}
        }
    }
}

pub async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let mut game: Game = Game {
        input_state: InputState::default(),
        graphics: None,
        game: None,
        window: None,
    };
    event_loop.run_app(&mut game).unwrap();
}
