//! One window over a scene, shared by every client format.
//!
//! The two client binaries differ in where their text comes from — a mounted package's label nodes,
//! or an XML window definition lowered to lines — and not in how a window is opened, cleared and
//! drawn. That part lives here once, so the formats cannot drift apart in the thing they are being
//! compared on.
//!
//! Nothing here reads a package, a frame or a surface: the scene arrives already adopted.

use std::error::Error;
use std::sync::Arc;

use glyphon::{
    Attrs, Buffer, Cache, Color as TextColor, Family, FontSystem, Metrics, Shaping, SwashCache,
    TextArea, TextAtlas, TextBounds, TextRenderer, Viewport,
};
use pollster::block_on;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

/// What a window shows: a title and the lines beneath it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Scene {
    pub title: String,
    pub lines: Vec<String>,
}

impl Scene {
    pub fn new(title: impl Into<String>, lines: Vec<String>) -> Scene {
        Scene {
            title: title.into(),
            lines,
        }
    }

    pub fn body(&self) -> String {
        format!("{}\n{}", self.title, self.lines.join("\n"))
    }
}

struct State {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    font_system: FontSystem,
    cache: SwashCache,
    atlas_cache: Cache,
    atlas: TextAtlas,
    renderer: TextRenderer,
    viewport: Viewport,
    buffer: Buffer,
}

impl State {
    fn new(window: Arc<Window>, body: String) -> State {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window.clone()).expect("a surface");
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("an adapter");
        let (device, queue) =
            block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None))
                .expect("a device");
        let size = window.inner_size();
        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let mut font_system = FontSystem::new();
        let mut buffer = Buffer::new(&mut font_system, Metrics::new(16.0, 22.0));
        buffer.set_size(
            &mut font_system,
            Some(config.width as f32 - 24.0),
            Some(config.height as f32 - 24.0),
        );
        buffer.set_text(
            &mut font_system,
            &body,
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );
        buffer.shape_until_scroll(&mut font_system, false);
        let cache = SwashCache::new();
        let atlas_cache = Cache::new(&device);
        let viewport = Viewport::new(&device, &atlas_cache);
        let mut atlas = TextAtlas::new(&device, &queue, &atlas_cache, format);
        let renderer =
            TextRenderer::new(&mut atlas, &device, wgpu::MultisampleState::default(), None);

        State {
            window,
            surface,
            device,
            queue,
            config,
            font_system,
            cache,
            atlas_cache,
            atlas,
            renderer,
            viewport,
            buffer,
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.buffer.set_size(
            &mut self.font_system,
            Some(width as f32 - 24.0),
            Some(height as f32 - 24.0),
        );
        self.buffer.shape_until_scroll(&mut self.font_system, false);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        self.renderer
            .prepare(
                &self.device,
                &self.queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                [TextArea {
                    buffer: &self.buffer,
                    left: 12.0,
                    top: 12.0,
                    scale: 1.0,
                    bounds: TextBounds::default(),
                    default_color: TextColor::rgb(230, 232, 240),
                    custom_glyphs: &[],
                }],
                &mut self.cache,
            )
            .expect("text prepared");
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("frame") });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("window"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.07,
                            g: 0.08,
                            b: 0.10,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.renderer
                .render(&self.atlas, &self.viewport, &mut pass)
                .expect("text drawn");
        }
        self.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }
}

struct App {
    scene: Scene,
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(format!("{} — planet client", self.scene.title))
                        .with_inner_size(winit::dpi::LogicalSize::new(960.0, 640.0)),
                )
                .expect("a window"),
        );
        self.state = Some(State::new(window.clone(), self.scene.body()));
        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(state) = self.state.as_mut() {
                    state.resize(size.width, size.height);
                    state.window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                let Some(state) = self.state.as_mut() else {
                    return;
                };
                match state.render() {
                    Ok(()) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.surface.configure(&state.device, &state.config);
                        state.window.request_redraw();
                    }
                    Err(error) => {
                        eprintln!("render: {error}");
                        event_loop.exit();
                    }
                }
            }
            _ => {}
        }
    }
}

/// Open one window over `scene` and run its event loop until the window closes.
pub fn run(scene: Scene) -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App { scene, state: None };
    event_loop.run_app(&mut app)?;
    Ok(())
}
