use std::time::Instant;

use winit::event::{ElementState, MouseButton};

use crate::game_logic_old::World;

pub struct BackEnd {
  time_stamp: Instant,
  loaded_world: Option<World>,
}
impl BackEnd {
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    let time_stamp = Instant::now();
    let world = Some(World::new());
    Self {
      time_stamp,
      loaded_world: world,
    }
  }

  pub fn update(&mut self) {
    let now = Instant::now();
    let dur = now - self.time_stamp;
    self.time_stamp = now;
    // 1秒(1_000_000_000ナノ秒)で割り、
    // 前回のサイクルからの経過時間(秒)を得る
    let time_delta = dur.as_nanos() as f64 / 1.0e9;

    if let Some(world) = self.loaded_world.as_mut() {
      world.player.update(time_delta);
    }
  }

  pub fn frontend_update(
    &self,
    frontend: &mut crate::front::FrontEnd,
  ) {
    if let Some((gfx, world)) = frontend
      .gfx
      .as_mut()
      .zip(self.loaded_world.as_ref())
    {
      gfx.update_camera(
        &world.player.position.cast(),
        &world.player.rotation.cast(),
      );
      gfx.update_tile_instances(|instances| {
        world
          .blocks
          .render_instances_update(instances);
      });
    }
  }

  pub fn keyboard_input(
    &mut self,
    key_event: winit::event::KeyEvent,
  ) {
    let Some(player) = self
      .loaded_world
      .as_mut()
      .map(|w| &mut w.player)
    else {
      return;
    };
    player
      .input
      .keyboard_input(key_event);
  }

  pub fn mouse_motion_input(
    &mut self,
    delta: (f64, f64),
  ) {
    let Some(player) = self
      .loaded_world
      .as_mut()
      .map(|w| &mut w.player)
    else {
      return;
    };
    let delta = (delta.0 * 0.002, delta.1 * 0.002);
    player
      .input
      .mouse_motion_input(delta);
  }

  pub fn mouse_button_input(
    &mut self,
    button: MouseButton,
    state: ElementState,
  ) {
    let Some(world) = self.loaded_world.as_mut() else {
      return;
    };

    world
      .player
      .input
      .mouse_button_input(button, state);
  }
}
