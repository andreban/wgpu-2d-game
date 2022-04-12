mod rendering;
mod shapes;

use winit::dpi::LogicalSize;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::rendering::pipelines::square::SquarePipeline;
use crate::rendering::Render;
use crate::shapes::Square;
use rendering::pipelines::tutorial3::Tutorial3;
use rendering::pipelines::tutorial4::Tutorial4Pipeline;
use rendering::WebGpu;

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(LogicalSize::new(600.0, 650.0))
        .build(&event_loop)
        .unwrap();
    let mut webgpu = WebGpu::new(&window).await;
    let mut tutorial3 = SquarePipeline::new(&mut webgpu);
    let quads = vec![
        Square {
            position: (0.0, 0.0).into(),
            size: (100.0, 100.0).into(),
            color: (1.0, 0.0, 1.0).into(),
        },
        Square {
            position: (200.0, 200.0).into(),
            size: (100.0, 150.0).into(),
            color: (1.0, 0.0, 0.0).into(),
        },
    ];

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
                tutorial3.render(&mut render_pass, &mut render.webgpu.queue, &quads);
                drop(render_pass);
                render.draw();
            }
            _ => {}
        }
    });
}
