use crate::common::*;
use std::sync::Arc;
use winit::window::Window;

pub mod wgpu_ctx;

pub struct GfxInstance {
  wgpu_ctx: wgpu_ctx::WGPUCtx,
}
impl GfxInstance {
  pub async fn new(
    window: Arc<Window>,
  ) -> StdResult<Self> {
    let wgpu_ctx =
      wgpu_ctx::WGPUCtx::new(window).await?;
    Ok(Self { wgpu_ctx })
  }
}
