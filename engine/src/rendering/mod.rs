pub mod camera;
pub mod pipelines;
pub mod shapes;
pub mod texture;

use crate::error::EngineError;
use glyphon::{
    Attrs, Buffer, Cache, Color, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache,
    TextArea, TextAtlas, TextBounds, TextRenderer, Viewport,
};
use pipelines::{SpritePipeline, SquarePipeline};
use shapes::{Sprite, Square};
use std::iter;
use std::sync::Arc;
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
    pub async fn new(window: Arc<Window>, texture_bytes: &[u8], world_width: f32, world_height: f32) -> Result<Self, EngineError> {
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

        let (mut device, mut queue) = adapter
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

        let square_pipeline = SquarePipeline::new(&mut device, &mut queue, &configuration, world_width, world_height);
        let sprite_pipeline = SpritePipeline::new(&mut device, &mut queue, &configuration, texture_bytes, world_width, world_height)?;

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
}

pub struct Canvas<'a> {
    graphics: &'a mut Graphics,
    sprites: Vec<Sprite>,
    rects: Vec<Square>,
    score: Option<u32>,
}

impl<'a> Canvas<'a> {
    pub fn new(graphics: &'a mut Graphics) -> Self {
        Self {
            graphics,
            sprites: vec![],
            rects: vec![],
            score: None,
        }
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite) {
        self.sprites.push(*sprite);
    }

    pub fn draw_rect(&mut self, rect: &Square) {
        self.rects.push(*rect);
    }

    pub fn set_score(&mut self, score: u32) {
        self.score = Some(score);
    }

    pub fn done(self) {
        drop(self);
    }

    fn render(&mut self) {
        let output = match self.graphics.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t)
            | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            _ => return,
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

        // Prepare text rendering before the render pass.
        if let Some(score) = self.score {
            let score_text = format!("SCORE: {score}");
            let g = &mut self.graphics;

            let mut buffer = Buffer::new(&mut g.font_system, Metrics::new(30.0, 42.0));
            buffer.set_size(
                &mut g.font_system,
                Some(g.size.width as f32),
                Some(g.size.height as f32),
            );
            buffer.set_text(
                &mut g.font_system,
                &score_text,
                &Attrs::new().family(Family::SansSerif),
                Shaping::Advanced,
                None,
            );
            buffer.shape_until_scroll(&mut g.font_system, false);

            g.text_viewport.update(
                &g.queue,
                Resolution {
                    width: g.size.width,
                    height: g.size.height,
                },
            );

            g.text_renderer
                .prepare(
                    &g.device,
                    &g.queue,
                    &mut g.font_system,
                    &mut g.text_atlas,
                    &g.text_viewport,
                    [TextArea {
                        buffer: &buffer,
                        left: 350.0,
                        top: 10.0,
                        scale: 1.0,
                        bounds: TextBounds {
                            left: 0,
                            top: 0,
                            right: g.size.width as i32,
                            bottom: g.size.height as i32,
                        },
                        default_color: Color::rgb(255, 255, 0),
                        custom_glyphs: &[],
                    }],
                    &mut g.swash_cache,
                )
                .unwrap();
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
            &mut self.graphics.queue,
            &self.sprites,
        );

        if !self.rects.is_empty() {
            self.graphics.square_pipeline.render(
                &mut render_pass,
                &mut self.graphics.queue,
                &self.rects,
            );
        }

        if self.score.is_some() {
            self.graphics
                .text_renderer
                .render(
                    &self.graphics.text_atlas,
                    &self.graphics.text_viewport,
                    &mut render_pass,
                )
                .unwrap();
        }

        drop(render_pass);

        self.graphics.staging_belt.finish();
        self.graphics.queue.submit(iter::once(encoder.finish()));
        output.present();
    }
}

impl<'a> Drop for Canvas<'a> {
    fn drop(&mut self) {
        self.render();
    }
}
