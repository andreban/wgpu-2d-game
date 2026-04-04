pub mod camera;
pub mod pipelines;
pub mod shapes;
pub mod text;
pub mod texture;

use crate::error::EngineError;
pub use glyphon::FontSystem;
use glyphon::{
    Cache, Color, Resolution, SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer, Viewport,
};
use pipelines::{SpritePipeline, SquarePipeline};
use shapes::{Sprite, Square};
use std::iter;
use std::sync::Arc;
pub use text::TextLabel;
use wgpu::util::StagingBelt;
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};
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
    pub surface: Surface<'static>,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub configuration: SurfaceConfiguration,
    pub square_pipeline: SquarePipeline,
    pub sprite_pipeline: SpritePipeline,
    pub staging_belt: StagingBelt,
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
    pub text_cache: Cache,
    pub text_viewport: Viewport,
    pub text_atlas: TextAtlas,
    pub text_renderer: TextRenderer,
}

impl Graphics {
    pub async fn new(
        window: Arc<Window>,
        texture_bytes: &[u8],
        world_width: f32,
        world_height: f32,
    ) -> Result<Self, EngineError> {
        let size = window.inner_size();

        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                ..Default::default()
            })
            .await?;

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];
        let configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &configuration);

        let staging_belt = wgpu::util::StagingBelt::new(device.clone(), 1024);

        let square_pipeline =
            SquarePipeline::new(&device, &queue, &configuration, world_width, world_height);
        let sprite_pipeline = SpritePipeline::new(
            &device,
            &queue,
            &configuration,
            texture_bytes,
            world_width,
            world_height,
        )?;

        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let text_cache = Cache::new(&device);
        let text_viewport = Viewport::new(&device, &text_cache);
        let mut text_atlas = TextAtlas::new(&device, &queue, &text_cache, swapchain_format);
        let text_renderer = TextRenderer::new(
            &mut text_atlas,
            &device,
            wgpu::MultisampleState::default(),
            None,
        );

        Ok(Graphics {
            size,
            instance,
            surface,
            adapter,
            device,
            queue,
            configuration,
            square_pipeline,
            sprite_pipeline,
            staging_belt,
            font_system,
            swash_cache,
            text_cache,
            text_viewport,
            text_atlas,
            text_renderer,
        })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.configuration.width = new_size.width;
            self.configuration.height = new_size.height;
            self.surface.configure(&self.device, &self.configuration);
        }
    }

    pub fn prepare_text(&mut self, texts: &[(&TextLabel, f32, f32)]) -> Result<(), EngineError> {
        let (width, height) = (self.size.width, self.size.height);

        self.text_viewport
            .update(&self.queue, Resolution { width, height });

        let text_areas: Vec<TextArea> = texts
            .iter()
            .map(|(label, x, y)| TextArea {
                buffer: &label.buffer,
                left: *x,
                top: *y,
                scale: 1.0,
                bounds: TextBounds {
                    left: 0,
                    top: 0,
                    right: width as i32,
                    bottom: height as i32,
                },
                default_color: Color::rgb(255, 255, 0),
                custom_glyphs: &[],
            })
            .collect();

        self.text_renderer.prepare(
            &self.device,
            &self.queue,
            &mut self.font_system,
            &mut self.text_atlas,
            &self.text_viewport,
            text_areas,
            &mut self.swash_cache,
        )?;
        Ok(())
    }
}

pub struct Canvas<'a> {
    graphics: &'a mut Graphics,
    sprites: Vec<Sprite>,
    rects: Vec<Square>,
    texts: Vec<(&'a TextLabel, f32, f32)>,
    rendered: bool,
}

impl<'a> Canvas<'a> {
    pub fn new(graphics: &'a mut Graphics) -> Self {
        Self {
            graphics,
            sprites: vec![],
            rects: vec![],
            texts: vec![],
            rendered: false,
        }
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite) {
        self.sprites.push(*sprite);
    }

    pub fn draw_rect(&mut self, rect: &Square) {
        self.rects.push(*rect);
    }

    pub fn draw_text(&mut self, label: &'a TextLabel, x: f32, y: f32) {
        self.texts.push((label, x, y));
    }

    pub fn font_system(&mut self) -> &mut FontSystem {
        &mut self.graphics.font_system
    }

    pub fn done(mut self) -> Result<(), EngineError> {
        self.render()?;
        self.rendered = true;
        Ok(())
    }

    fn render(&mut self) -> Result<(), EngineError> {
        let output = match self.graphics.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t)
            | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            other => return Err(EngineError::SurfaceTexture(format!("{other:?}"))),
        };
        let view = &output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            self.graphics
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        if !self.texts.is_empty() {
            self.graphics.prepare_text(&self.texts)?;
        }

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            ..Default::default()
        });

        self.graphics.sprite_pipeline.render(
            &mut render_pass,
            &self.graphics.queue,
            &self.sprites,
        )?;

        if !self.rects.is_empty() {
            self.graphics.square_pipeline.render(
                &mut render_pass,
                &self.graphics.queue,
                &self.rects,
            )?;
        }

        if !self.texts.is_empty() {
            self.graphics.text_renderer.render(
                &self.graphics.text_atlas,
                &self.graphics.text_viewport,
                &mut render_pass,
            )?;
        }

        drop(render_pass);

        self.graphics.staging_belt.finish();
        self.graphics.queue.submit(iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

impl<'a> Drop for Canvas<'a> {
    fn drop(&mut self) {
        if !self.rendered {
            if let Err(e) = self.render() {
                eprintln!("Render error: {e}");
            }
        }
    }
}
