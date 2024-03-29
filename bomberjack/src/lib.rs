mod game;
mod input;

use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use engine::rendering::shapes::Sprite;
use engine::rendering::{Canvas, Graphics};
use game::BombJackGame;
use input::InputState;

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(LogicalSize::new(game::CANVAS_WIDTH, game::CANVAS_HEIGHT))
        .build(&event_loop)
        .unwrap();

    let mut input_state = InputState::new();
    let mut graphics = Graphics::new(&window).await.unwrap();
    let mut game = BombJackGame::new();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } => {
                if window_id == window.id() {
                    match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }
                        WindowEvent::Resized(physical_size) => {
                            graphics.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            graphics.resize(**new_inner_size);
                        }
                        WindowEvent::KeyboardInput { input, .. } => input_state.update(input),
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) => {
                if window_id != window.id() {
                    return;
                }
                // Update game state.
                game.update(&input_state);

                // Render game
                let mut canvas = Canvas::new(&mut graphics);
                game.render(&mut canvas);
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}
