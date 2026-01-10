use crate::common::*;

use std::{any::Any, sync::Arc, thread::JoinHandle};
use wgpu::SurfaceError;
use winit::window::Window;

use super::{rendering, wgpu_ctx};

pub struct GfxHandler {
  jh: JoinHandle<
    Result<
      rendering::RenderingSuccess,
      rendering::RenderingError,
    >,
  >,
  render_sender: rendering::RenderCommandSend,
}
impl GfxHandler {
  pub async fn new(
    window: Arc<Window>,
  ) -> StdResult<Self> {
    let module = GfxModule::new(window).await?;
    let (sender, receiver) =
      rendering::RenderCommandSend::new();

    let jh = std::thread::spawn(move || {
      module.module_run(receiver)
    });
    Ok(Self {
      jh,
      render_sender: sender,
    })
  }

  pub fn rendering(&self) {
    self
      .render_sender
      .command_send(rendering::RenderCommand::Redraw)
  }

  pub fn resized(&self) {
    self
      .render_sender
      .command_send(rendering::RenderCommand::Resize);
  }

  pub fn stop(
    self,
  ) -> Result<
    Result<
      rendering::RenderingSuccess,
      rendering::RenderingError,
    >,
    Box<dyn Any + Send>,
  > {
    std::mem::drop(self.render_sender);
    self.jh.join()
  }
}

struct GfxModule {
  wgpu_ctx: wgpu_ctx::WGPUCtx,
  world_rdr: super::renderer::WorldRenderer,
}
impl GfxModule {
  pub async fn new(
    window: Arc<Window>,
  ) -> StdResult<Self> {
    let wgpu_ctx =
      wgpu_ctx::WGPUCtx::new(window).await?;
    let world_rdr =
      super::renderer::WorldRenderer::new();
    Ok(Self {
      wgpu_ctx,
      world_rdr,
    })
  }

  /// 描画処理本体
  fn rendering(&self, target: wgpu_ctx::RenderTarget) {
    self
      .world_rdr
      .rendering(&target);
    target.present();
  }

  /// Executing asynchronous renderer module
  fn module_run(
    self,
    render_perker: rendering::RenderCommandRecv,
  ) -> Result<
    rendering::RenderingSuccess,
    rendering::RenderingError,
  > {
    // Main loop
    loop {
      match render_perker.wait_and_rendering(|comm| match comm {
        rendering::RenderCommand::Redraw => {
          tracing::trace!("Rendering process start.");
          let target = match self.wgpu_ctx.rendering() {
            Ok(target) => target,
            Err(SurfaceError::OutOfMemory) => {
              tracing::error!(
                "Out of memory error occured!"
              );
              eprintln!("Out of memory error occured!");
              return Err(rendering::RenderingError::OutOfMemory)
            }
            Err(SurfaceError::Lost) => {
              self.wgpu_ctx.reconfigure();
              return Ok(rendering::RenderingSuccess::Nop)
            }
            Err(e) => {
              tracing::warn!("Surface error occured.");
              tracing::warn!("detail: {e}");
              eprintln!("Surface error occured.");
              eprintln!("detail: {e}");
              self.wgpu_ctx.reconfigure();
              return Ok(rendering::RenderingSuccess::Nop)
            }
          };
          self.rendering(target);

          Ok(rendering::RenderingSuccess::Nop)
        },
        rendering::RenderCommand::Resize => {
          self.wgpu_ctx.resize();
          Ok(rendering::RenderingSuccess::Nop)
        },
        rendering::RenderCommand::Exit => {
        // 終了指示を受領している
        tracing::info!(
          "Rendering stop request received"
        );
        Ok(rendering::RenderingSuccess::StopRequested)
      },
    })? {
        rendering::RenderingSuccess::Nop => {},
        ok @ rendering::RenderingSuccess::StopRequested => return Ok(
          ok
        ),
      }
    }
  }
}
