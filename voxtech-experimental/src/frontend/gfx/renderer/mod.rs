use super::wgpu_ctx::*;

pub mod util;

pub mod prototype;
pub mod tile;

pub struct WorldRendererHandler {
  pub channel: crossbeam::channel::Sender<(
    [f64; 2],
    nalgebra::Point3<f64>,
  )>,
}

pub struct WorldRenderer {
  prototype: prototype::PrototypeRenderer,
}
impl WorldRenderer {
  pub fn new(
    ctx: &super::wgpu_ctx::WGPUCtx,
  ) -> (Self, WorldRendererHandler) {
    let (send, recv) = crossbeam::channel::unbounded();
    let prototype =
      prototype::PrototypeRenderer::new(ctx, recv);
    let handler =
      WorldRendererHandler { channel: send };
    (Self { prototype }, handler)
  }

  pub fn rendering(&mut self, target: &RenderTarget) {
    tracing::trace_span!("world renderer rendering")
      .in_scope(|| {
        self
          .prototype
          .update(target.ctx);
        let mut enc = target
          .ctx
          .device
          .create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
              label: Some(
                "world renderer command encoder",
              ),
            },
          );
        self
          .prototype
          .rendering(&mut enc, target);
        match target
          .ctx
          .device
          .poll(wgpu::PollType::Poll)
        {
          Ok(_) => {}
          Err(e) => tracing::warn!(
            "GPU driver polling failure. {e}"
          ),
        }
        target
          .ctx
          .queue
          .submit([enc.finish()]);
      });
  }
}
