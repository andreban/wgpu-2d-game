pub mod camera2d;
pub mod pipelines;

use std::iter;
use wgpu::{
    Adapter, CommandEncoder, Device, Instance, Queue, RenderPass, Surface, SurfaceConfiguration,
    SurfaceTexture, TextureView,
};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct WebGpu {
    pub size: PhysicalSize<u32>,
    pub instance: Instance,
    pub surface: Surface,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub configuration: SurfaceConfiguration,
}

impl WebGpu {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        println!("{}", size.width as f32 / size.height as f32);

        // Create an instance of WebGPU.
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        // Create a Surface.
        let surface = unsafe { instance.create_surface(window) };

        // Request an adapter, compatible with the surface.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // Request a device.
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        // Create the configuration.
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        WebGpu {
            size,
            instance,
            surface,
            adapter,
            device,
            queue,
            configuration: config,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.configuration.width = new_size.width;
            self.configuration.height = new_size.height;
            self.surface.configure(&self.device, &self.configuration);
        }
    }

    pub fn start_render(&mut self) -> Result<Render, wgpu::SurfaceError> {
        Render::new(self)
    }
}

pub struct Render<'a> {
    pub output: SurfaceTexture,
    pub view: TextureView,
    pub encoder: CommandEncoder,
    pub webgpu: &'a mut WebGpu,
}

impl<'a> Render<'a> {
    fn new(webgpu: &'a mut WebGpu) -> Result<Self, wgpu::SurfaceError> {
        let output = webgpu.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = webgpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        Ok(Render {
            output,
            view,
            encoder,
            webgpu,
        })
    }

    pub fn render_pass<'b>(
        encoder: &'b mut CommandEncoder,
        view: &'b TextureView,
    ) -> RenderPass<'b> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        })
    }

    pub fn draw(self) {
        self.webgpu.queue.submit(iter::once(self.encoder.finish()));
        self.output.present();
    }
}
