//! ワールド描画用のレンダラ

use crate::gfx::wgpu_ctx::RenderTarget;

use super::{texture::Texture, wgpu_ctx::WGPUCtx};

pub mod chunk;
pub mod tile;

pub mod camera3d;

pub struct WorldRdr {
  tile: tile::TileShared,
  pub chunk_layout: chunk::uniform::ChunkUniformLayout,
  opaque: tile::OpaqueTileRdr,
  main_camera: camera3d::Camera3DUniformInstance,
  depth_texture: Texture,
  pub chunk: chunk::ChunkStorage,
}
impl WorldRdr {
  pub fn new(
    ctx: &WGPUCtx,
    camera_config: &camera3d::Camera3DConfig,
    camera_instance: &camera3d::Camera3DInstance,
  ) -> Self {
    let depth_texture =
      Texture::new_depth(ctx, "World depth texture");
    let main_camera =
      camera3d::Camera3DUniformInstance::new(
        ctx,
        camera_config,
        camera_instance,
      );
    let tile = tile::TileShared::new(ctx);
    let chunk_layout =
      chunk::uniform::ChunkUniformLayout::new(ctx);
    let opaque = tile::OpaqueTileRdr::new(
      ctx,
      &main_camera,
      &chunk_layout,
    );
    let chunk = chunk::ChunkStorage::new();
    Self {
      tile,
      chunk_layout,
      opaque,
      main_camera,
      depth_texture,
      chunk,
    }
  }

  pub fn resize(&mut self, ctx: &WGPUCtx) {
    self.depth_texture =
      Texture::new_depth(ctx, "World depth texture")
  }

  pub fn update(
    &mut self,
    ctx: &WGPUCtx,
    config: &camera3d::Camera3DConfig,
    instance: &camera3d::Camera3DInstance,
  ) {
    self
      .main_camera
      .update(ctx, config, instance);
  }

  pub fn rendering(
    &mut self,
    render_target: &RenderTarget,
  ) {
    {
      let mut encoder = render_target
        .ctx
        .device
        .create_command_encoder(
          &wgpu::CommandEncoderDescriptor {
            label: Some(
              "World renderer command encoder",
            ),
          },
        );
      {
        let mut _rpass = encoder.begin_render_pass(
          &wgpu::RenderPassDescriptor {
            label: Some("World renderer pass"),
            color_attachments: &[Some(
              wgpu::RenderPassColorAttachment {
                view: &render_target.view,
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
            depth_stencil_attachment: Some(
              wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                  load: wgpu::LoadOp::Clear(1.0),
                  store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
              },
            ),
            timestamp_writes: None,
            occlusion_query_set: None,
          },
        );
      }
      render_target
        .ctx
        .queue
        .submit([encoder.finish()]);
    }
    self.opaque.rendering(
      render_target,
      &self.depth_texture,
      &self.main_camera,
      &self.tile,
      &self.chunk,
    );
  }
}
