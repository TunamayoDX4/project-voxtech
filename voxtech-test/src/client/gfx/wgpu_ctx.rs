use std::sync::Arc;
use wgpu::{
  Device, Queue, Surface, SurfaceConfiguration,
};
use winit::window::Window;

pub struct WGPUCtx {
  pub(super) window: Arc<Window>,
  pub(super) surface: Surface<'static>,
  pub(super) device: Device,
  pub(super) queue: Queue,
  pub(super) config: SurfaceConfiguration,
}
impl WGPUCtx {
  pub async fn new(
    window: Arc<Window>,
  ) -> crate::util::StdResult<Self> {
    let instance = wgpu::Instance::new(
      wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: wgpu::InstanceFlags::default(),
        memory_budget_thresholds:
          wgpu::MemoryBudgetThresholds::default(
          ),
        backend_options:
          wgpu::BackendOptions::default(),
        display: Some(Box::new(Arc::clone(
          &window,
        ))),
      },
    );
    let surface = instance
      .create_surface(Arc::clone(&window))?;
    let adapter = instance
      .request_adapter(
        &wgpu::RequestAdapterOptionsBase {
          power_preference:
            wgpu::PowerPreference::HighPerformance,
          force_fallback_adapter: false,
          compatible_surface: Some(&surface),
        },
      )
      .await?;
    let (device, queue) = adapter
      .request_device(&wgpu::DeviceDescriptor {
        label: Some("gfx devices"),
        required_features: wgpu::Features::default(),
        required_limits: wgpu::Limits::defaults(),
        experimental_features:
          wgpu::ExperimentalFeatures::disabled(),
        memory_hints: wgpu::MemoryHints::Performance,
        trace: wgpu::Trace::Off,
      })
      .await?;
    let config = wgpu::SurfaceConfiguration {
      usage:
        wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface
        .get_capabilities(&adapter)
        .formats[0],
      width: window.inner_size().width,
      height: window.inner_size().height,
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode:
        wgpu::CompositeAlphaMode::Auto,
      view_formats: vec![],
      desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    Ok(Self {
      window,
      surface,
      device,
      queue,
      config,
    })
  }

  pub fn reconfigure(&self) {
    self
      .surface
      .configure(&self.device, &self.config);
  }

  pub fn resize(&mut self) {
    let size = self.window.inner_size();
    if size.width == 0 || size.height == 0 {
      return;
    }
    self.config.width = size.width;
    self.config.height = size.height;
    self.reconfigure();
  }

  pub fn rendering(
    &self,
    renderer: &mut super::renderer::Renderer,
  ) -> crate::util::StdResult<()> {
    struct GetSurfaceTextureResult {
      /// サーフェステクスチャ本体
      texture: Option<wgpu::SurfaceTexture>,
      /// 再コンフィグが必要か？
      require_reconfigure: bool,
      /// リカバリ不能でプログラムの終了を推奨するか？
      cannot_recoverable: bool,
    }

    let surface_texture = match self
      .surface
      .get_current_texture()
    {
      wgpu::CurrentSurfaceTexture::Success(
        surface_texture,
      ) => GetSurfaceTextureResult {
        texture: Some(surface_texture),
        require_reconfigure: false,
        cannot_recoverable: false,
      },
      wgpu::CurrentSurfaceTexture::Suboptimal(
        surface_texture,
      ) => GetSurfaceTextureResult {
        texture: Some(surface_texture),
        require_reconfigure: true,
        cannot_recoverable: false,
      },
      wgpu::CurrentSurfaceTexture::Timeout
      | wgpu::CurrentSurfaceTexture::Outdated
      | wgpu::CurrentSurfaceTexture::Lost => {
        GetSurfaceTextureResult {
          texture: None,
          require_reconfigure: true,
          cannot_recoverable: false,
        }
      }
      wgpu::CurrentSurfaceTexture::Occluded => {
        GetSurfaceTextureResult {
          texture: None,
          require_reconfigure: false,
          cannot_recoverable: false,
        }
      }
      wgpu::CurrentSurfaceTexture::Validation => {
        GetSurfaceTextureResult {
          texture: None,
          require_reconfigure: false,
          cannot_recoverable: true,
        }
      }
    };

    if let Some(texture) =
      surface_texture.texture
    {
      let mut enc =
        self.device.create_command_encoder(
          &wgpu::CommandEncoderDescriptor {
            label: Some(
              "Renderer command encoder",
            ),
          },
        );
      renderer.rendering(&texture, &mut enc);

      self.queue.submit([enc.finish()]);
      texture.present();
    }
    if surface_texture.require_reconfigure {
      self.reconfigure();
    }
    if surface_texture.cannot_recoverable {
      return Err(
        "can not recoverable error occured."
          .into(),
      );
    }

    Ok(())
  }
}
