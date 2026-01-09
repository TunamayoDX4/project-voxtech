use crate::common::*;
use std::{any::Any, sync::Arc, thread::JoinHandle};
use wgpu::SurfaceError;
use winit::window::Window;

pub mod rendering;
use rendering::{
  RenderCommand, RenderingError, RenderingSuccess,
};
pub mod wgpu_ctx;

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
    self.render_sender.rendering()
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
}
impl GfxModule {
  pub async fn new(
    window: Arc<Window>,
  ) -> StdResult<Self> {
    let wgpu_ctx =
      wgpu_ctx::WGPUCtx::new(window).await?;
    Ok(Self { wgpu_ctx })
  }

  fn module_run(
    mut self,
    render_perker: rendering::RenderCommandRecv,
  ) -> Result<
    rendering::RenderingSuccess,
    rendering::RenderingError,
  > {
    loop {
      match render_perker.wait_and_rendering(|comm| match comm {
        RenderCommand::Redraw => {
          tracing::debug!("Rendering process start.");
          let target = match self.wgpu_ctx.rendering() {
            Ok(target) => target,
            Err(SurfaceError::OutOfMemory) => {
              tracing::error!(
                "Out of memory error occured!"
              );
              eprintln!("Out of memory error occured!");
              return Err(RenderingError::OutOfMemory)
            }
            Err(SurfaceError::Lost) => {
              self.wgpu_ctx.reconfigure();
              return Ok(RenderingSuccess::Nop)
            }
            Err(e) => {
              tracing::warn!("Surface error occured.");
              tracing::warn!("detail: {e}");
              eprintln!("Surface error occured.");
              eprintln!("detail: {e}");
              self.wgpu_ctx.reconfigure();
              return Ok(RenderingSuccess::Nop)
            }
          };

          /* 描画処理はここから */


          /* 描画処理はここまで */

          target.present();

          Ok(RenderingSuccess::Nop)
        },
        RenderCommand::Resize => {
          self.wgpu_ctx.resize();
          Ok(RenderingSuccess::Nop)
        },
        RenderCommand::Exit => {
        // 終了指示を受領している
        tracing::info!(
          "Rendering stop request received"
        );
        Ok(RenderingSuccess::StopRequested)
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
