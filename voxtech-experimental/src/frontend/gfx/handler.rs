use crate::common::*;

use parking_lot::Mutex;
use std::{
  any::Any,
  sync::{Arc, atomic::AtomicU64},
  thread::JoinHandle,
};
use wgpu::SurfaceError;
use winit::{dpi::PhysicalSize, window::Window};

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
  ) -> StdResult<(
    Self,
    super::renderer::WorldRendererHandler,
  )> {
    let (module, handler) =
      GfxModule::new(window).await?;
    let (sender, receiver) =
      rendering::RenderCommandSend::new();

    let jh = std::thread::spawn(move || {
      module.module_run(receiver)
    });
    Ok((
      Self {
        jh,
        render_sender: sender,
      },
      handler,
    ))
  }

  pub fn rendering(&self) {
    self
      .render_sender
      .command_send(rendering::RenderCommand::Redraw)
  }

  pub fn resize(&self, new_size: PhysicalSize<u32>) {
    self.render_sender.command_send(
      rendering::RenderCommand::Resize { new_size },
    );
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
  render_cycle_time: Mutex<std::time::Instant>,
  cycle_per_sec: Mutex<f64>,
}
impl GfxModule {
  pub async fn new(
    window: Arc<Window>,
  ) -> StdResult<(
    Self,
    super::renderer::WorldRendererHandler,
  )> {
    let wgpu_ctx =
      wgpu_ctx::WGPUCtx::new(window).await?;
    let (world_rdr, world_rdr_handler) =
      super::renderer::WorldRenderer::new(&wgpu_ctx);
    let render_cycle_time = std::time::Instant::now();
    let render_cycle_time =
      Mutex::new(render_cycle_time);
    let cycle_per_sec = Mutex::new(0.0);
    Ok((
      Self {
        wgpu_ctx,
        world_rdr,
        render_cycle_time,
        cycle_per_sec,
      },
      world_rdr_handler,
    ))
  }

  /// Executing asynchronous renderer module
  fn module_run(
    mut self,
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

          // 描画処理本体
          {
            self
              .world_rdr
              .rendering(&target);
            target.present();
            let now = std::time::Instant::now();
            let mut rct = self.render_cycle_time.lock();
            let dur = now - *rct;
            let cps = 1_000_000_000f64 / dur.as_nanos() as f64;
            *self.cycle_per_sec.lock() = cps;
            *rct = now;
          }

          Ok(rendering::RenderingSuccess::Nop)
        },
        rendering::RenderCommand::Resize{
          new_size
        } => {
          self.wgpu_ctx.resize(new_size);
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
