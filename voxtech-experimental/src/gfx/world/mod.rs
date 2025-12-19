//! ワールド描画用のレンダラ

use crate::gfx::wgpu_ctx::RenderTarget;

use super::{texture::Texture, wgpu_ctx::WGPUCtx};

pub mod chunk;
pub mod tile;

pub mod camera3d;

pub struct WorldRdr {
  tile: tile::TileShared,
  chunk: chunk::uniform::ChunkUniformLayout,
  opaque: tile::OpaqueTileRdr,
  main_camera: camera3d::Camera3DUniformInstance,
  depth_texture: Texture,
}
impl WorldRdr {
  pub fn new(
    ctx: &WGPUCtx,
    camera_config: &camera3d::Camera3DConfig,
    camera_instance: &camera3d::Camera3DInstance,
  ) -> Self {
    let depth_texture =
      Texture::new_depth(ctx, "Depth texture");
    let main_camera =
      camera3d::Camera3DUniformInstance::new(
        ctx,
        camera_config,
        camera_instance,
      );
    let tile = tile::TileShared::new(ctx);
    let chunk =
      chunk::uniform::ChunkUniformLayout::new(ctx);
    let opaque = tile::OpaqueTileRdr::new(
      ctx,
      &tile,
      &main_camera,
    );
    Self {
      tile,
      opaque,
      main_camera,
      depth_texture,
      chunk,
    }
  }

  pub fn resize(&mut self, ctx: &WGPUCtx) {
    self.depth_texture =
      Texture::new_depth(ctx, "Depth texture")
  }

  pub fn rendering(
    &mut self,
    render_target: &RenderTarget,
  ) {
  }
}
