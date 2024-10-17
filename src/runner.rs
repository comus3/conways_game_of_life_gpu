use std::sync::Arc;
use crate::app::App;

use pollster::FutureExt;
use wgpu::{Adapter, Device, Instance, PresentMode, Queue, Surface, SurfaceCapabilities};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

pub struct Runner {
    context: Option<Context>,
    app: Option<App>,
}

impl Runner {
    pub fn new() -> Self {
        Self { context: None, app: None }
    }

    pub async fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        let _ = event_loop.run_app(self);
    }
}

impl ApplicationHandler for Runner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("Game of Life"))
            .unwrap();
        self.context = Some(Context::new(window));
        self.app = Some(App::new(self.context.as_mut().unwrap()));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = self.context.as_ref().unwrap().window();

        if window.id() == window_id {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    self.context.as_mut().unwrap().resize(physical_size);
                }
                WindowEvent::RedrawRequested => {
                    self.app
                        .as_mut()
                        .unwrap()
                        .update(self.context.as_mut().unwrap());
                    self.app
                        .as_mut()
                        .unwrap()
                        .render(self.context.as_mut().unwrap())
                        .unwrap();
                }
                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let window = self.context.as_ref().unwrap().window();
        window.request_redraw();
    }
}

pub struct Context {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: wgpu::SurfaceConfiguration,

    size: PhysicalSize<u32>,
    window: Arc<Window>,
}

impl Context {
    pub fn new(window: Window) -> Self {
        let window_arc = Arc::new(window);
        let size = window_arc.inner_size();
        let instance = Self::create_gpu_instance();
        let surface = instance.create_surface(window_arc.clone()).unwrap();
        let adapter = Self::create_adapter(instance, &surface);
        let (device, queue) = Self::create_device(&adapter);
        let surface_caps = surface.get_capabilities(&adapter);
        let config = Self::create_surface_config(size, surface_caps);
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window: window_arc,
        }
    }

    fn create_surface_config(
        size: PhysicalSize<u32>,
        capabilities: SurfaceCapabilities,
    ) -> wgpu::SurfaceConfiguration {
        let surface_format = capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);

        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::AutoNoVsync,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }

    fn create_device(adapter: &Adapter) -> (Device, Queue) {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: Default::default(),
                },
                None,
            )
            .block_on()
            .unwrap()
    }

    fn create_adapter(instance: Instance, surface: &Surface) -> Adapter {
        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .block_on()
            .unwrap()
    }

    fn create_gpu_instance() -> Instance {
        Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;

            self.config.width = new_size.width;
            self.config.height = new_size.height;

            self.surface.configure(&self.device, &self.config);

            println!("Resized to {:?} from state!", new_size);
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }
    
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }
    
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
    
    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    
}