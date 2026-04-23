use nalgebra::{UnitQuaternion, UnitVector3, Vector3};
use wgpu::{
  CommandEncoder, PipelineLayout, RenderPipeline,
  SurfaceTexture,
};

use super::util::*;
use crate::gfx::WGPUCtx;

pub mod tile;

pub struct Renderer {
  pub camera: CameraBundle,
  pub tile: tile::TileRdr,
}
impl Renderer {
  pub fn new(
    gfx: &WGPUCtx,
  ) -> Result<Self, Box<dyn std::error::Error>> {
    let camera = CameraBundle::new(
      gfx,
      Camera {
        fovy: std::f32::consts::FRAC_PI_3,
        znear: 0.5,
        zfar: 1000.,
      },
      0,
    );
    let tile = tile::TileRdr::new(gfx, &camera);
    Ok(Self { camera, tile })
  }

  pub fn surface_resize(
    &mut self,
    gfx: &WGPUCtx,
    new_size: winit::dpi::PhysicalSize<u32>,
  ) -> Result<(), Box<dyn std::error::Error>> {
    self
      .tile
      .surface_resize(gfx, new_size);
    Ok(())
  }

  pub fn renderer_update(
    &mut self,
    gfx: &WGPUCtx,
  ) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
  }

  pub fn rendering(
    &mut self,
    surface_texture: &SurfaceTexture,
    enc: &mut CommandEncoder,
  ) {
    let surface_texture_view = surface_texture
      .texture
      .create_view(
        &wgpu::TextureViewDescriptor::default(),
      );
    enc.begin_render_pass(
      &wgpu::RenderPassDescriptor {
        label: Some("sample render pass"),
        color_attachments: &[Some(
          wgpu::RenderPassColorAttachment {
            view: &surface_texture_view,
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
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
      },
    );
    self.tile.rendering(
      surface_texture,
      enc,
      &self.camera,
    );
  }
}
