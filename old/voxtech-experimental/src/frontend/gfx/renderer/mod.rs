use super::wgpu_ctx::*;

pub mod util;

pub mod prototype;
pub mod tile;

pub mod camera;

pub struct WorldRendererHandler {
  pub channel: crossbeam::channel::Sender<(
    [f64; 2],
    nalgebra::Point3<f64>,
  )>,
}

pub struct WorldRenderer {
  camera: camera::WRCamera,
  depth: util::texture::DepthTexture,
  prototype: prototype::PrototypeRenderer,
}
impl WorldRenderer {
  pub fn new(
    ctx: &super::wgpu_ctx::WGPUCtx,
  ) -> (Self, WorldRendererHandler) {
    let (send, recv) = crossbeam::channel::unbounded();
    let depth = util::texture::DepthTexture::new_depth(
      ctx,
      "depth texture",
    );
    let prototype = prototype::PrototypeRenderer::new(
      ctx, recv, &depth,
    );
    let handler =
      WorldRendererHandler { channel: send };
    let camera = camera::WRCamera::new(ctx);
    (
      Self {
        camera,
        depth,
        prototype,
      },
      handler,
    )
  }

  /// レンダラの各変数の更新処理
  pub fn update(&mut self, ctx: &WGPUCtx) {
    self.camera.update(ctx);
  }

  /// 画面のリサイズ処理
  pub fn resize(&mut self, ctx: &WGPUCtx) {
    self.depth = util::texture::DepthTexture::new_depth(
      ctx,
      "depth texture",
    );
  }

  /// レンダラの描画処理本体
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
        self.prototype.rendering(
          &mut enc,
          target,
          &self.depth,
        );
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
