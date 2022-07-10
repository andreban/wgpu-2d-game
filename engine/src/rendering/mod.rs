pub mod camera;
pub mod pipelines;
pub mod texture;
pub mod shapes;

use shapes::{Sprite, Square};
use pipelines::{SpritePipeline, SquarePipeline};
use std::iter;
use wgpu::util::StagingBelt;
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};
use winit::dpi::PhysicalSize;
use winit::window::Window;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Graphics {
    pub size: PhysicalSize<u32>,
    pub instance: Instance,
    pub surface: Surface,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub configuration: SurfaceConfiguration,
    pub square_pipeline: SquarePipeline,
    pub sprite_pipeline: SpritePipeline,
    pub glyph_brush: GlyphBrush<()>,
    pub staging_belt: StagingBelt,
}

impl Graphics {
    pub async fn new(window: &Window) -> Self {
        let inconsolata =
            ab_glyph::FontArc::try_from_slice(include_bytes!("../../../bomberjack/src/assets/Inconsolata-Regular.ttf"))
                .unwrap();

        let size = window.inner_size();

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
        let (mut device, mut queue) = adapter
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
        let format = surface.get_supported_formats(&adapter)[0];
        let configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &configuration);
        let staging_belt = wgpu::util::StagingBelt::new(1024);

        let glyph_brush =
            GlyphBrushBuilder::using_font(inconsolata).build(&device, configuration.format);

        let square_pipeline = SquarePipeline::new(&mut device, &mut queue, &configuration);
        let sprite_pipeline = SpritePipeline::new(&mut device, &mut queue, &configuration);

        Graphics {
            size,
            instance,
            surface,
            adapter,
            device,
            queue,
            configuration,
            square_pipeline,
            sprite_pipeline,
            glyph_brush,
            staging_belt,
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

    pub fn render(
        &mut self,
        squares: &[Square],
        sprites: &[Sprite],
        score: u32,
    ) -> Result<(), wgpu::SurfaceError> {
        // Setup render.
        let output = self.surface.get_current_texture()?;
        let view = &output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
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
            })],
            depth_stencil_attachment: None,
        });

        // Render pipelines.
        self.sprite_pipeline
            .render(&mut render_pass, &mut self.queue, sprites);
        if !squares.is_empty() {
            self.square_pipeline
                .render(&mut render_pass, &mut self.queue, squares);
        }

        // Submit to screen.
        drop(render_pass);

        self.glyph_brush.queue(Section {
            screen_position: (350.0, 10.0),
            bounds: (self.size.width as f32, self.size.height as f32),
            text: vec![Text::new(&format!("{}", score))
                .with_color([1.0, 0.0, 0.0, 1.0])
                .with_scale(40.0)],
            ..Section::default()
        });

        self.glyph_brush
            .draw_queued(
                &self.device,
                &mut self.staging_belt,
                &mut encoder,
                view,
                self.size.width,
                self.size.height,
            )
            .expect("Draw queued");

        self.staging_belt.finish();
        self.queue.submit(iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

pub struct Canvas<'a> {
    graphics: &'a mut Graphics,
    sprites: Vec<Sprite>,
    rects: Vec<Square>,
}

impl <'a> Canvas <'a> {
    pub fn new(graphics: &'a mut Graphics) -> Self {
        Self {
            graphics,
            sprites: vec![],
            rects: vec![],
        }
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite) {
        self.sprites.push(*sprite);
    }

    pub fn draw_rect(&mut self, rect: &Square) {
        self.rects.push(*rect);
    }

    pub fn done(self) {
        drop(self);
    }

    fn render(&mut self) {
        // Setup render.
        let output = self.graphics.surface.get_current_texture().unwrap();
        let view = &output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .graphics
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
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
            })],
            depth_stencil_attachment: None,
        });

        // Render pipelines.
        self.graphics.sprite_pipeline
            .render(&mut render_pass, &mut self.graphics.queue, &self.sprites);

        if !self.rects.is_empty() {
            self.graphics.square_pipeline
                .render(&mut render_pass, &mut self.graphics.queue, &self.rects);
        }

        // Submit to screen.
        drop(render_pass);

        self.graphics.glyph_brush.queue(Section {
            screen_position: (350.0, 10.0),
            bounds: (self.graphics.size.width as f32, self.graphics.size.height as f32),
            text: vec![Text::new(&format!("{}", 1000))
                .with_color([1.0, 0.0, 0.0, 1.0])
                .with_scale(40.0)],
            ..Section::default()
        });

        self.graphics.glyph_brush
            .draw_queued(
                &self.graphics.device,
                &mut self.graphics.staging_belt,
                &mut encoder,
                view,
                self.graphics.size.width,
                self.graphics.size.height,
            )
            .expect("Draw queued");

        self.graphics.staging_belt.finish();
        self.graphics.queue.submit(iter::once(encoder.finish()));
        output.present();
    }
}

impl <'a> Drop for Canvas<'a> {
    fn drop(&mut self) {
        self.render();
    }
}
