mod rendering;
mod shapes;

use winit::dpi::LogicalSize;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::rendering::pipelines::squares::square::SquarePipeline;
use crate::rendering::pipelines::squares::sprite::SpritePipeline;
use crate::rendering::Render;
use crate::shapes::{Sprite, Square};
use rendering::WebGpu;

const CANVAS_WIDTH: f32 = 600.0;
const CANVAS_HEIGHT: f32 = 650.0;

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(LogicalSize::new(CANVAS_WIDTH, CANVAS_HEIGHT))
        .build(&event_loop)
        .unwrap();
    let mut webgpu = WebGpu::new(&window).await;
    let mut squares_pipeline = SquarePipeline::new(&mut webgpu);
    let mut sprites_pipeline = SpritePipeline::new(&mut webgpu);
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
                            webgpu.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            webgpu.resize(**new_inner_size);
                        }
                        WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                            Some(VirtualKeyCode::Up) => {
                                sprites[1].position.y += 10.0;
                            }
                            Some(VirtualKeyCode::Down) => {
                                sprites[1].position.y -= 10.0;
                            }
                            Some(VirtualKeyCode::Left) => {
                                sprites[1].position.x -= 10.0;
                            }
                            Some(VirtualKeyCode::Right) => {
                                sprites[1].position.x += 10.0;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) => {
                if window_id != window.id() {
                    return;
                }
                let mut render = webgpu.start_render().unwrap();
                let mut render_pass = Render::render_pass(&mut render.encoder, &render.view);
                sprites_pipeline.render(&mut render_pass, &mut render.webgpu.queue, &sprites);
                squares_pipeline.render(&mut render_pass, &mut render.webgpu.queue, &quads);
                drop(render_pass);
                render.draw();
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
