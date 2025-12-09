use std::sync::Arc;

use winit::window::Window;

use wgpu::{
  Device, Queue, Surface, SurfaceConfiguration,
};

use crate::gfx::world_renderer::block_rdr;

pub mod camera;
pub mod world_renderer;

/// WGPUのコンテキスト構造体
pub struct WGPUContext {
  surface: Surface<'static>,
  device: Device,
  queue: Queue,
  config: SurfaceConfiguration,
  camera: Arc<crate::PRwLock<camera::CameraConfig>>,

  window: Arc<Window>,
}
impl WGPUContext {
  /// コンテキストの初期化
  pub async fn new(
    window: Arc<Window>,
  ) -> crate::StdResult<Self> {
    // WGPUのインスタンスの初期化
    let instance =
      wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        flags: wgpu::InstanceFlags::default(),
        memory_budget_thresholds:
          wgpu::MemoryBudgetThresholds::default(),
        backend_options: wgpu::BackendOptions::default(
        ),
      });

    // 描画先であるサーフェスのWGPU構造体の初期化
    let surface =
      instance.create_surface(Arc::clone(&window))?;

    // WGPUにおける仮想的なGPU構造体であるAdapterの初期化
    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference:
          wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
      })
      .await?;

    // WGPUにおけるGPUのプロキシであるDeviceとそこへのコマンド送信Queueの初期化
    let (device, queue) = adapter
      .request_device(&wgpu::DeviceDescriptor {
        label: Some("Device descripter"),
        required_features: wgpu::Features::default(),
        experimental_features:
          wgpu::ExperimentalFeatures::default(),
        required_limits: wgpu::Limits::defaults(),
        memory_hints: wgpu::MemoryHints::Performance,
        trace: wgpu::Trace::Off,
      })
      .await?;

    // WGPUのサーフェスの設定の初期化
    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface
        .get_capabilities(&adapter)
        .formats
        .iter()
        .copied()
        .next()
        .unwrap(),
      width: window.inner_size().width,
      height: window.inner_size().height,
      present_mode: wgpu::PresentMode::Fifo,
      desired_maximum_frame_latency: 2,
      alpha_mode: wgpu::CompositeAlphaMode::Auto,
      view_formats: Vec::new(),
    };
    surface.configure(&device, &config);

    // カメラの基底コンフィグ
    let camera = camera::CameraConfig {
      fovy: 45. * std::f64::consts::PI / 180.,
      near: 0.001,
      far: 1000.,
    };
    let camera = Arc::new(crate::PRwLock::new(camera));

    Ok(Self {
      surface,
      device,
      queue,
      config,
      window,
      camera,
    })
  }

  /// 再コンフィグ
  pub fn reconfigure(&self) {
    self
      .surface
      .configure(&self.device, &self.config);
  }

  /// ウィンドウのリサイズ
  pub fn resize(&mut self) {
    let size = self.window.inner_size();
    self.config.width = size.width;
    self.config.height = size.height;
    self.reconfigure();
  }

  /// 描画処理
  pub fn rendering(
    &self,
    renderer: &world_renderer::WorldRenderer,
    block_rdr: &[block_rdr::BlockRenderInstance],
  ) -> Result<(), wgpu::SurfaceError> {
    self.window.request_redraw();
    let output = self
      .surface
      .get_current_texture()?;
    let view = output.texture.create_view(
      &wgpu::TextureViewDescriptor::default(),
    );
    renderer.rendering(&view, self, block_rdr);
    output.present();

    Ok(())
  }
}
