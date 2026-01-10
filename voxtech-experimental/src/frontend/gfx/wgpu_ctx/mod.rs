//! WGPUのコンテキスト関連

use crate::common::aliases::*;
use std::sync::Arc;

use parking_lot::Mutex;
use wgpu::{
  Device, Queue, Surface, SurfaceConfiguration,
  SurfaceTexture, TextureView,
};
use winit::window::Window;

/// WGPUのコンテキストをまとめた構造体
pub struct WGPUCtx {
  pub window: Arc<Window>,
  pub surface: Surface<'static>,
  pub device: Device,
  pub queue: Queue,
  pub config: Mutex<SurfaceConfiguration>,
}
impl WGPUCtx {
  /// コンテキストの初期化
  pub async fn new(
    window: Arc<Window>,
  ) -> StdResult<Self> {
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

    let config = Mutex::new(config);

    Ok(Self {
      window,
      surface,
      device,
      queue,
      config,
    })
  }

  /// 再コンフィグ
  pub fn reconfigure(&self) {
    let config = self.config.lock();
    self
      .surface
      .configure(&self.device, &config);
  }

  /// ウィンドウのリサイズ
  pub fn resize(&self) {
    let mut config = self.config.lock();
    let size = self.window.inner_size();
    if 0 < size.width && 0 < size.height {
      config.width = size.width;
      config.height = size.height;
    } else {
      config.width = 1;
      config.height = 1;
    }
    self
      .surface
      .configure(&self.device, &config);
  }

  /// 描画処理
  pub fn rendering<'a>(
    &'a self,
  ) -> Result<RenderTarget<'a>, wgpu::SurfaceError> {
    self.window.request_redraw();
    let output = self
      .surface
      .get_current_texture()?;
    let view = output.texture.create_view(
      &wgpu::TextureViewDescriptor::default(),
    );

    Ok(RenderTarget {
      ctx: self,
      output,
      view,
    })
  }
}

pub struct RenderTarget<'a> {
  pub ctx: &'a WGPUCtx,
  pub output: SurfaceTexture,
  pub view: TextureView,
}
impl RenderTarget<'_> {
  pub fn present(self) {
    self.output.present();
  }
}
