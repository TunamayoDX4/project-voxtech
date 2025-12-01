type StdErr = Box<dyn std::error::Error>;
type StdResult<T> = Result<T, StdErr>;
use std::sync::Arc;

struct Camera {
  position: nalgebra::Point3<f64>,
  rotation: nalgebra::Vector3<f64>,
  fov_y: f64,
  near: f64,
  far: f64,
}

struct Viewer {
  camera: Camera,
}

pub mod server;

struct App {
  wgpu_context: Option<WGPUContext>,
  window_attr: winit::window::WindowAttributes,
  viewer: Viewer,
}
impl App {
  pub fn new(
    window_attr: winit::window::WindowAttributes,
    viewer: Viewer,
  ) -> Self {
    Self {
      wgpu_context: None,
      window_attr,
      viewer,
    }
  }
}
impl winit::application::ApplicationHandler for App {
  fn resumed(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
  ) {
    let window = event_loop
      .create_window(self.window_attr.clone())
      .expect("Failed to create window");
    let window = Arc::new(window);
    let wgpu_context = pollster::block_on(WGPUContext::new(
      Arc::clone(&window),
    ))
    .expect("Failed to create WGPU context");
    self.wgpu_context = Some(wgpu_context);
  }

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    match event {
      winit::event::WindowEvent::CloseRequested => {
        event_loop.exit();
      }
      winit::event::WindowEvent::RedrawRequested => {
        if let Some(wgpu_context) = self.wgpu_context.as_mut() {
          match wgpu_context.rendering() {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost) => {
              wgpu_context
                .resize(wgpu_context.window.inner_size());
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
              event_loop.exit();
            }
            Err(e) => {
              eprintln!("Rendering error: {:?}", e);
            }
          }
        }
      }
      winit::event::WindowEvent::Resized(new_size) => {
        if let Some(wgpu_context) = self.wgpu_context.as_mut() {
          wgpu_context.resize(new_size);
        }
      }
      winit::event::WindowEvent::KeyboardInput {
        device_id: _,
        event,
        is_synthetic: _,
      } => match event.logical_key {
        winit::keyboard::Key::Named(k) => match k {
          winit::keyboard::NamedKey::Escape => {
            event_loop.exit();
          }
          _ => {}
        },
        _ => {}
      },
      _ => {}
    }
  }
}

struct WGPUContext {
  surface: wgpu::Surface<'static>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  window: Arc<winit::window::Window>,

  render_pipeline_layout: wgpu::PipelineLayout,
  render_pipeline: wgpu::RenderPipeline,
}
impl WGPUContext {
  async fn new(
    window: Arc<winit::window::Window>,
  ) -> StdResult<Self> {
    let size = window.inner_size();

    let instance = wgpu::Instance::default();
    let surface =
      instance.create_surface(Arc::clone(&window))?;
    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference:
          wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await?;
    let (device, queue) = adapter
      .request_device(&wgpu::DeviceDescriptor {
        label: Some("WGPU device"),
        required_features: wgpu::Features::default(),
        required_limits: wgpu::Limits::defaults(),
        experimental_features:
          wgpu::ExperimentalFeatures::default(),
        memory_hints: wgpu::MemoryHints::Performance,
        trace: wgpu::Trace::Off,
      })
      .await?;
    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface
        .get_capabilities(&adapter)
        .formats[0],
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode: wgpu::CompositeAlphaMode::Auto,
      view_formats: vec![],
      desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    let render_pipeline_layout = device.create_pipeline_layout(
      &wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
      },
    );

    let render_pipeline = {
      let shader = device.create_shader_module(
        wgpu::ShaderModuleDescriptor {
          label: Some("Shader"),
          source: wgpu::ShaderSource::Wgsl(
            include_str!("main.wgsl").into(),
          ),
        },
      );
      device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
          label: Some("Render Pipeline"),
          layout: Some(&render_pipeline_layout),
          vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options:
              wgpu::PipelineCompilationOptions {
                constants: &[],
                zero_initialize_workgroup_memory: false,
              },
          },
          fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
              format: config.format,
              blend: Some(wgpu::BlendState::ALPHA_BLENDING),
              write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options:
              wgpu::PipelineCompilationOptions {
                constants: &[],
                zero_initialize_workgroup_memory: false,
              },
          }),
          primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
          },
          depth_stencil: None,
          multisample: wgpu::MultisampleState::default(),
          multiview: None,
          cache: None,
        },
      )
    };

    Ok(Self {
      surface,
      device,
      queue,
      config,
      window,
      render_pipeline_layout,
      render_pipeline,
    })
  }

  pub fn resize(
    &mut self,
    new_size: winit::dpi::PhysicalSize<u32>,
  ) {
    if new_size.width == 0 || new_size.height == 0 {
      return;
    }
    self.config.width = new_size.width;
    self.config.height = new_size.height;
    self
      .surface
      .configure(&self.device, &self.config);
  }

  pub fn aspect_ratio(&self) -> f64 {
    self.config.width as f64 / self.config.height as f64
  }

  pub fn rendering(&self) -> Result<(), wgpu::SurfaceError> {
    self.window.request_redraw();
    let output = self
      .surface
      .get_current_texture()?;
    let view = output
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = self
      .device
      .create_command_encoder(
        &wgpu::CommandEncoderDescriptor {
          label: Some("Render Encoder"),
        },
      );
    {
      let mut rpass = encoder.begin_render_pass(
        &wgpu::RenderPassDescriptor {
          label: Some("Render pass"),
          color_attachments: &[Some(
            wgpu::RenderPassColorAttachment {
              view: &view,
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
              depth_slice: None,
            },
          )],
          depth_stencil_attachment: None,
          timestamp_writes: None,
          occlusion_query_set: None,
        },
      );

      rpass.set_pipeline(&self.render_pipeline);
      rpass.draw(0..3, 0..1);
    }
    self
      .queue
      .submit(std::iter::once(
        encoder.finish(),
      ));
    output.present();

    Ok(())
  }
}

fn main() -> StdResult<()> {
  let event_loop = winit::event_loop::EventLoop::new()?;
  let viewer = Viewer {
    camera: Camera {
      position: nalgebra::Point3::new(0.0, 0.0, 5.0),
      rotation: nalgebra::Vector3::new(0.0, 0.0, 0.0),
      fov_y: 45.0,
      near: 0.1,
      far: 100.0,
    },
  };
  let mut app = App::new(
    winit::window::Window::default_attributes()
      .with_title("VoxTech")
      .with_inner_size(winit::dpi::PhysicalSize::new(
        1280u32, 720u32,
      ))
      .with_resizable(false)
      .with_active(true)
      .with_visible(true)
      .with_enabled_buttons(
        winit::window::WindowButtons::CLOSE
          | winit::window::WindowButtons::MINIMIZE,
      ),
    viewer,
  );
  event_loop.run_app(&mut app)?;

  Ok(())
}
