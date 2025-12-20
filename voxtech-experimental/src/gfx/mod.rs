//! グラフィクスモジュール

pub mod wgpu_ctx;

pub mod texture;
pub mod world;

pub struct GfxBundle {
  wgpu_ctx: wgpu_ctx::WGPUCtx,
  world: Option<world::WorldRdr>,
}
impl GfxBundle {
  pub async fn new(
    window: std::sync::Arc<winit::window::Window>,
  ) -> crate::StdResult<Self> {
    let wgpu_ctx =
      wgpu_ctx::WGPUCtx::new(window).await?;
    let world = None;

    Ok(Self { wgpu_ctx, world })
  }

  pub fn reconfigure(&self) {
    self.wgpu_ctx.reconfigure();
  }

  pub fn resize(&mut self) {
    self.wgpu_ctx.resize();
  }

  pub fn world_init(
    &mut self,
    camera_config: &world::camera3d::Camera3DConfig,
    camera_instance: &world::camera3d::Camera3DInstance,
  ) {
    let world = world::WorldRdr::new(
      &self.wgpu_ctx,
      camera_config,
      camera_instance,
    );
    self.world = Some(world);
  }

  pub fn world(
    &mut self,
    f: impl FnOnce(&wgpu_ctx::WGPUCtx, &mut world::WorldRdr),
  ) {
    let Some(world) = self.world.as_mut() else {
      return;
    };
    f(&self.wgpu_ctx, world)
  }

  pub fn rendering(
    &mut self,
  ) -> Result<(), wgpu::SurfaceError> {
    let target = self.wgpu_ctx.rendering()?;
    if let Some(world) = self.world.as_mut() {
      world.rendering(&target);
    }
    target.present();

    Ok(())
  }
}
