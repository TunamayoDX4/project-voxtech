use super::util::*;
use wgpu::RenderPassColorAttachment;

pub mod tile;

pub struct Renderer {
  main_depth: Texture,
  pub camera: CameraBundle,
  pub tile: tile::TileRdr,
}
impl Renderer {
  pub fn new(
    wgpu_ctx: &super::wgpu_ctx::WGPUCtx,
  ) -> Self {
    let main_depth =
      Texture::create_depth_texture(wgpu_ctx);
    let camera = CameraBundle::new(
      wgpu_ctx,
      Camera {
        fovy: std::f32::consts::FRAC_PI_3,
        znear: 0.125,
        zfar: 1000.,
      },
      0,
    );
    let tile =
      tile::TileRdr::new(wgpu_ctx, &camera);
    Self {
      main_depth,
      camera,
      tile,
    }
  }

  pub fn rendering(
    &mut self,
    texture: &wgpu::SurfaceTexture,
    encoder: &mut wgpu::CommandEncoder,
  ) {
    encoder.begin_render_pass(
      &wgpu::RenderPassDescriptor {
        label: Some("render pass"),
        color_attachments: &[Some(
          RenderPassColorAttachment {
            view: &texture.texture.create_view(
              &wgpu::TextureViewDescriptor::default(),
            ),
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
              load: wgpu::LoadOp::Clear(wgpu::Color {
                r: 0.6,
                g: 0.8,
                b: 1.0,
                a: 1.0,
              }),
              store: wgpu::StoreOp::Store,
            },
          },
        )],
        depth_stencil_attachment: Some(
          wgpu::RenderPassDepthStencilAttachment {
            view: &self.main_depth.view,
            depth_ops: Some(wgpu::Operations {
              load: wgpu::LoadOp::Clear(1.0),
              store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
          },
        ),
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
      },
    );
    // 描画処理
    self.tile.rendering(
      encoder,
      texture,
      &self.main_depth,
      &self.camera,
    );
  }

  pub fn resize(
    &mut self,
    wgpu_ctx: &super::wgpu_ctx::WGPUCtx,
    _new_size: winit::dpi::PhysicalSize<u32>,
  ) {
    self.main_depth =
      Texture::create_depth_texture(wgpu_ctx)
  }
}
