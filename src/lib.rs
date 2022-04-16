mod game;
mod input;
mod rendering;
mod shapes;

use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use game::BombJackGame;
use input::InputState;
use rendering::Graphics;
use shapes::{Sprite, Square};

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(LogicalSize::new(game::CANVAS_WIDTH, game::CANVAS_HEIGHT))
        .build(&event_loop)
        .unwrap();

    let mut input_state = InputState::new();
    let mut graphics = Graphics::new(&window).await;
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
                graphics
                    .render(
                        &[],
                        &[
                            &game.background,
                            &game.platforms[0],
                            &game.platforms[1],
                            &game.platforms[2],
                            &game.platforms[3],
                            &game.platforms[4],
                            &(&game.jack).into(),
                        ],
                    )
                    .unwrap();
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
