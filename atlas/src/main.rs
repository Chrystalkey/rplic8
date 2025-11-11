use std::sync::Arc;

use cgmath::Vector2;
#[allow(unused)]
use tracing::{debug, error, info, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use crate::{
    maprender::{MapRenderpass, Metadata},
    renderpass::ColorRenderPass,
};

mod figure_render;
mod maprender;
mod renderpass;
mod uniform;

struct ColorRenderpasses {
    map_bg_rp: MapRenderpass,
}
impl ColorRenderpasses {
    fn new(
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
    ) -> Self {
        Self {
            map_bg_rp: maprender::MapRenderpass::new(surface_format, &device, &queue),
        }
    }
    fn render(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        col_render_target: &wgpu::TextureView,
        dep_render_target: Option<&wgpu::TextureView>,
        ued: Metadata,
    ) {
        self.map_bg_rp
            .render(device, encoder, col_render_target, dep_render_target, ued);
    }
    fn reload_shaders(&mut self, device: &wgpu::Device) {
        self.map_bg_rp.reload_shaders(device);
    }
}

/// TODO: This actually has to be a "state-global" structure containing all updated data. `struct ColorRenderpasses` above takes it on himself
/// to form this into the Uniform structs all the renderpasses require
struct RenderState {
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    renderpasses: ColorRenderpasses,
    metadata: Metadata,
    start_time: std::time::Instant,
}

impl RenderState {
    async fn new(window: Arc<Window>) -> RenderState {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features {
                    features_wgpu: wgpu::FeaturesWGPU::ADDRESS_MODE_CLAMP_TO_BORDER,
                    ..Default::default()
                },
                ..Default::default()
            })
            .await
            .unwrap();

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

        let crp = ColorRenderpasses::new(&queue, &device, surface_format);

        let state = RenderState {
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
            renderpasses: crp,
            metadata: Metadata {
                time: 0.,
                map_zoom: 1.,
                map_translation: cgmath::Vector2 { x: 0., y: 0. },
                window_size: cgmath::Vector2 {
                    x: size.width as f32,
                    y: size.height as f32,
                },
                mouse_pos: cgmath::Vector2 { x: 0., y: 0. },
                dnd_map_movacc: cgmath::Vector2 { x: 0., y: 0. },
            },
            start_time: std::time::Instant::now(),
        };

        // Configure surface for the first time
        state.configure_surface();

        state
    }

    fn get_window(&self) -> &Window {
        &self.window
    }

    fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.metadata.window_size = cgmath::Vector2 {
            x: new_size.width as f32,
            y: new_size.height as f32,
        };

        // reconfigure the surface
        self.configure_surface();
    }

    fn render(&mut self) {
        // Create texture view
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                // Without add_srgb_suffix() the image we will be working with
                // might not be "gamma correct".
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });
        self.metadata.time = (std::time::Instant::now() - self.start_time).as_secs_f32();
        // Renders a GREEN screen
        let mut encoder = self.device.create_command_encoder(&Default::default());
        // Create the renderpass which will clear the screen.
        self.renderpasses.render(
            &self.device,
            &mut encoder,
            &texture_view,
            None,
            self.metadata,
        );

        // Submit the command in the queue to execute
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }
}

#[derive(PartialEq)]
enum DNDState {
    Free,
    Left,
    Right,
}
struct App {
    state: Option<RenderState>,
    dnd_state: DNDState,
    dnd_start: Vector2<f32>,
}
impl App {
    fn new() -> Self {
        Self {
            state: None,
            dnd_state: DNDState::Free,
            dnd_start: Vector2 { x: 0., y: 0. },
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window object
        let min_size = winit::dpi::Size::Logical(winit::dpi::LogicalSize::new(20., 20.));

        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_min_inner_size(min_size))
                .unwrap(),
        );
        let rt = tokio::runtime::Runtime::new().unwrap();
        let state = rt.block_on(RenderState::new(window.clone()));
        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let app_state = self.state.as_mut().unwrap();
        // reset zoom position
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                app_state.render();
                // Emits a new redraw requested event.
                app_state.get_window().request_redraw();
            }
            WindowEvent::Resized(size) => {
                // Reconfigures the size of the surface. We do not re-render
                // here as this event is always followed up by redraw request.
                app_state.resize(size);
            }
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    text: Some(input), ..
                },
                ..
            } => {
                if input == "r" {
                    app_state.renderpasses.reload_shaders(&app_state.device)
                }
            }
            WindowEvent::MouseInput { button, state, .. } => {
                // right click means drag the map

                match (button, state) {
                    (winit::event::MouseButton::Right, winit::event::ElementState::Pressed) => {
                        if self.dnd_state == DNDState::Free {
                            self.dnd_state = DNDState::Right;
                            self.dnd_start = app_state.metadata.mouse_pos;
                        }
                    }
                    (winit::event::MouseButton::Right, winit::event::ElementState::Released) => {
                        if self.dnd_state == DNDState::Right {
                            self.dnd_state = DNDState::Free;
                            app_state.metadata.dnd_map_movacc = app_state.metadata.map_translation;
                        }
                    }
                    (winit::event::MouseButton::Left, winit::event::ElementState::Pressed) => {
                        if self.dnd_state == DNDState::Free {
                            self.dnd_state = DNDState::Left;
                            self.dnd_start = app_state.metadata.mouse_pos;
                        }
                    }
                    (winit::event::MouseButton::Left, winit::event::ElementState::Released) => {
                        if self.dnd_state == DNDState::Left {
                            self.dnd_state = DNDState::Free;
                            self.dnd_start = app_state.metadata.mouse_pos;
                        }
                    }
                    _ => {}
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                app_state.metadata.mouse_pos = cgmath::Vector2 {
                    x: position.x as f32,
                    y: app_state.metadata.window_size.y - position.y as f32,
                };
                if self.dnd_state == DNDState::Right {
                    app_state.metadata.map_translation = app_state.metadata.dnd_map_movacc
                        + Vector2 {
                            x: self.dnd_start.x - app_state.metadata.mouse_pos.x,
                            y: app_state.metadata.mouse_pos.y - self.dnd_start.y,
                        };
                }
                if self.dnd_state == DNDState::Left {
                    //TODO
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let delta = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, dy) => dy,
                    winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                };
                let old_zoom = app_state.metadata.map_zoom;
                app_state.metadata.map_zoom = f32::clamp(old_zoom * (1. + delta / 4.), 0.1, 10.);
            }
            _ => (),
        }
    }
}

fn main() {
    // wgpu uses `log` for all of our logging, so we initialize a logger with the `env_logger` crate.
    //
    // To change the log level, set the `RUST_LOG` environment variable. See the `env_logger`
    // documentation for more information.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "RUST_LOG=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let event_loop = EventLoop::new().unwrap();

    // When the current loop iteration finishes, immediately begin a new
    // iteration regardless of whether or not new events are available to
    // process. Preferred for applications that want to render as fast as
    // possible, like games.
    event_loop.set_control_flow(ControlFlow::Poll);

    // When the current loop iteration finishes, suspend the thread until
    // another event arrives. Helps keeping CPU utilization low if nothing
    // is happening, which is preferred if the application might be idling in
    // the background.
    // event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
