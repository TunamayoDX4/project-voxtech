use std::time::Instant;

use nalgebra::Vector3;
use winit::event::{
  ElementState, KeyEvent, MouseButton,
};

pub mod block_world;
pub mod entity_world;

pub struct WorldParam {
  pub gravity: Vector3<f64>,
}
impl WorldParam {
  pub fn calcurate_gravity_accelaration(
    &self,
    time_delta: f64,
  ) -> Vector3<f64> {
    self.gravity * time_delta
  }
}
impl Default for WorldParam {
  fn default() -> Self {
    Self {
      gravity: -Vector3::z() * 9.8 * 2.5,
    }
  }
}

pub struct World {
  last_update: Instant,
  param: WorldParam,
  block_world: block_world::BlockWorld,
  entity_world: entity_world::EntityWorld,
}
impl Default for World {
  fn default() -> Self {
    let param = WorldParam::default();
    let block_world =
      block_world::BlockWorld::new(());
    let entity_world =
      entity_world::EntityWorld::new(());
    Self {
      param,
      last_update: Instant::now(),
      block_world,
      entity_world,
    }
  }
}
impl World {
  pub fn cycle_update(&mut self) {
    let now = Instant::now();
    let dur = now - self.last_update;
    self.last_update = now;
    let delta = dur.as_nanos() as f64 / 1.0e9;
    self.entity_world.cycle_update(
      delta,
      &self.param,
      &mut self.block_world,
    );
  }

  pub fn camera_update(
    &self,
    wgpu_ctx: &super::gfx::wgpu_ctx::WGPUCtx,
    camera: &mut super::gfx::util::CameraBundle,
  ) {
    self
      .entity_world
      .render_update(wgpu_ctx, camera);
  }

  pub fn renderer_update(
    &mut self,
    wgpu_ctx: &super::gfx::wgpu_ctx::WGPUCtx,
    renderer: &mut super::gfx::renderer::Renderer,
  ) {
    self.block_world.render_update(
      wgpu_ctx,
      &mut renderer.tile.instances,
    );
  }

  pub fn key_input(
    &mut self,
    key_event: &KeyEvent,
  ) {
    self.entity_world.key_input(key_event);
  }

  pub fn mouse_motion_input(
    &mut self,
    mouse_motion: (f64, f64),
  ) {
    self
      .entity_world
      .mouse_motion_input(mouse_motion);
  }

  pub fn mouse_button_input(
    &mut self,
    state: ElementState,
    input: MouseButton,
  ) {
    self
      .entity_world
      .mouse_button_input(state, input);
  }
}
