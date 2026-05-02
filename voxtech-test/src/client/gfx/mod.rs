//! mod.rs(gfx)
//!

use std::sync::Arc;

use winit::window::Window;

pub mod renderer;
pub mod util;
pub mod wgpu_ctx;

pub struct GfxCtx {
  wgpu: wgpu_ctx::WGPUCtx,
  renderer: renderer::Renderer,
}
impl GfxCtx {
  pub async fn new(
    window: Arc<Window>,
  ) -> crate::util::StdResult<Self> {
    let wgpu =
      wgpu_ctx::WGPUCtx::new(window).await?;
    let renderer =
      renderer::Renderer::new(&wgpu);
    Ok(Self { wgpu, renderer })
  }

  pub fn resize(
    &mut self,
    new_size: winit::dpi::PhysicalSize<u32>,
  ) {
    self.wgpu.resize();
    self.renderer.resize(&self.wgpu, new_size);
  }

  pub fn update(
    &mut self,
    world: &mut super::world::World,
  ) {
    world.renderer_update(
      &self.wgpu,
      &mut self.renderer,
    );
    world.camera_update(
      &self.wgpu,
      &mut self.renderer.camera,
    );
  }

  pub fn rendering(
    &mut self,
  ) -> crate::util::StdResult<()> {
    self.wgpu.rendering(&mut self.renderer)
  }
}
