mod input;
mod rendering;
mod shapes;

use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use input::InputState;
use rendering::{
    pipelines::{SpritePipeline, SquarePipeline},
    Graphics,
};
use shapes::{Sprite, Square};

const CANVAS_WIDTH: f32 = 600.0;
const CANVAS_HEIGHT: f32 = 650.0;

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(LogicalSize::new(CANVAS_WIDTH, CANVAS_HEIGHT))
        .build(&event_loop)
        .unwrap();
    let mut input_state = InputState::new();
    let mut graphics = Graphics::new(&window).await;
    let mut sprites = vec![
        Sprite {
            position: (0.0, 0.0).into(),
            size: (600.0, 650.0).into(),
            texture: (0.0, 0.0, CANVAS_WIDTH / 1162.0, CANVAS_HEIGHT / 650.0).into(),
        },
        Sprite {
            position: (100.0, 100.0).into(),
            size: (39.0, 45.0).into(),
            texture: (601.0 / 1162.0, 256.0 / 650.0, 639.0 / 1162.0, 301.0 / 650.0).into(),
        },
    ];

    let quads = vec![Square {
        position: (200.0, 200.0).into(),
        size: (50.0, 50.0).into(),
        color: (1.0, 0.0, 0.0).into(),
    }];

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

                // Update game
                if input_state.up_pressed {
                    sprites[1].position.y += 1.0;
                }

                if input_state.down_pressed {
                    sprites[1].position.y -= 1.0;
                }

                if input_state.left_pressed {
                    sprites[1].position.x -= 1.0;
                }

                if input_state.right_pressed {
                    sprites[1].position.x += 1.0;
                }

                // Render game
                graphics.render(&quads, &sprites).unwrap();
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
