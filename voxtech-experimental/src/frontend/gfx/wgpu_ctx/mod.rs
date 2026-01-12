//! WGPUのコンテキスト関連

use crate::common::aliases::*;
use std::sync::{Arc, atomic::AtomicBool};

use parking_lot::Mutex;
use wgpu::{
  Adapter, Device, Queue, Surface,
  SurfaceConfiguration, SurfaceTexture, TextureView,
};
use winit::{dpi::PhysicalSize, window::Window};

/// WGPUのコンテキストをまとめた構造体
pub struct WGPUCtx {
  pub surface: Surface<'static>,
  pub device: Device,
  pub queue: Queue,
  pub adapter: Adapter,
  pub config: Mutex<SurfaceConfiguration>,
  pub config_changed: AtomicBool,
}
impl WGPUCtx {
  /// コンテキストの初期化
  pub async fn new(
    window: Arc<Window>,
  ) -> StdResult<Self> {
    // WGPUのインスタンスの初期化
    let instance =
      wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: wgpu::InstanceFlags::default(),
        memory_budget_thresholds:
          wgpu::MemoryBudgetThresholds::default(),
        backend_options: wgpu::BackendOptions::default(
        ),
      });

    // ウィンドウの内部サイズの取得
    let window_inner_size = window.inner_size();

    // 描画先であるサーフェスのWGPU構造体の初期化
    let surface = instance.create_surface(window)?;

    // WGPUにおける仮想的なGPU構造体であるAdapterの初期化
    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference:
          wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
      })
      .await?;

    // メッシュパイプの有効化
    let experimental_features =
      unsafe { wgpu::ExperimentalFeatures::enabled() };
    let required_features = wgpu::Features::default()
      | wgpu::Features::EXPERIMENTAL_MESH_SHADER
      | wgpu::Features::EXPERIMENTAL_PASSTHROUGH_SHADERS;
    let required_limits = wgpu::Limits::defaults()
      .using_recommended_minimum_mesh_shader_values();

    // WGPUにおけるGPUのプロキシであるDeviceとそこへのコマンド送信Queueの初期化
    let (device, queue) = adapter
      .request_device(&wgpu::DeviceDescriptor {
        label: Some("Device descripter"),
        required_features,
        experimental_features,
        required_limits,
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
      width: window_inner_size.width,
      height: window_inner_size.height,
      present_mode: wgpu::PresentMode::Fifo,
      desired_maximum_frame_latency: 2,
      alpha_mode: wgpu::CompositeAlphaMode::Auto,
      view_formats: Vec::new(),
    };
    surface.configure(&device, &config);

    let config = Mutex::new(config);
    println!(
      "c: {}, dim_1d: {}, dim_2d: {}, dim_3d: {}",
      adapter
        .limits()
        .max_texture_array_layers,
      adapter
        .limits()
        .max_texture_dimension_1d,
      adapter
        .limits()
        .max_texture_dimension_2d,
      adapter
        .limits()
        .max_texture_dimension_3d,
    );

    Ok(Self {
      surface,
      device,
      queue,
      adapter,
      config,
      config_changed: AtomicBool::new(false),
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
  pub fn resize(&self, new_size: PhysicalSize<u32>) {
    let mut config = self.config.lock();
    config.width = new_size.width.max(1);
    config.height = new_size.height.max(1);
    self.config_changed.store(
      true,
      std::sync::atomic::Ordering::Release,
    );
  }

  /// 描画処理
  pub fn rendering<'a>(
    &'a self,
  ) -> Result<RenderTarget<'a>, wgpu::SurfaceError> {
    // コンフィグが変わってたら更新する
    if self.config_changed.swap(
      false,
      std::sync::atomic::Ordering::AcqRel,
    ) {
      self.reconfigure();
    }
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
