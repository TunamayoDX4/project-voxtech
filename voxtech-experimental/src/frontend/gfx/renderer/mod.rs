use super::wgpu_ctx::*;

pub struct WorldRenderer {}
impl WorldRenderer {
  pub fn new() -> Self {
    Self {}
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
        enc.begin_render_pass(
          &wgpu::RenderPassDescriptor {
            label: Some(
              "world renderer main render pass",
            ),
            color_attachments: &[Some(
              wgpu::RenderPassColorAttachment {
                view: &target.view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                  load: wgpu::LoadOp::Clear(
                    wgpu::Color {
                      r: 0.1,
                      g: 0.2,
                      b: 0.3,
                      a: 1.0,
                    },
                  ),
                  store: wgpu::StoreOp::Store,
                },
              },
            )],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
          },
        );
        target
          .ctx
          .queue
          .submit([enc.finish()].into_iter());
      });
  }
}
