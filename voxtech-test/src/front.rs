use std::sync::Arc;

use winit::window::Window;

use super::gfx::GfxCtx;

pub struct FrontEnd {
  pub gfx: Option<GfxCtx>,
}
impl FrontEnd {
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    Self { gfx: None }
  }

  pub fn window_create(
    &mut self,
    window: Arc<Window>,
  ) -> Result<(), Box<dyn std::error::Error>> {
    self.gfx = Some(pollster::block_on(
      GfxCtx::new(window),
    )?);
    Ok(())
  }

  pub fn window_resize(
    &mut self,
    new_size: winit::dpi::PhysicalSize<u32>,
  ) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(gfx) = self.gfx.as_mut() {
      gfx.resize(new_size)?;
    }

    Ok(())
  }

  pub fn rendering(
    &mut self,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let Some(gfx) = self.gfx.as_mut() else {
      return Ok(());
    };

    gfx.rendering()
  }
}
