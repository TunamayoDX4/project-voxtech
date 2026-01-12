use super::wgpu_ctx::*;

pub mod util;

pub mod block;

pub struct WorldRenderer {
  prototype: block::PrototypeRenderer,
}
impl WorldRenderer {
  pub fn new(ctx: &super::wgpu_ctx::WGPUCtx) -> Self {
    let prototype = block::PrototypeRenderer::new(ctx);
    Self { prototype }
  }

  pub fn rendering(&self, target: &RenderTarget) {
    tracing::trace_span!("world renderer rendering")
      .in_scope(|| {
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
        target
          .ctx
          .queue
          .submit([enc.finish()].into_iter());
      });
  }
}
