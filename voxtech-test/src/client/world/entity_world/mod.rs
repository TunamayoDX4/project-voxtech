use nalgebra::{
  Point3, UnitQuaternion, Vector3,
};
use winit::event::{
  ElementState, KeyEvent, MouseButton,
};

use super::{
  block_world::BlockWorld, WorldParam,
};
use crate::client::gfx::{
  util::CameraBundle, wgpu_ctx::WGPUCtx,
};

pub mod player;

pub struct Entity {
  pub pos: Point3<f64>,
  pub rot: UnitQuaternion<f64>,
  pub vel: Vector3<f64>,
  pub center_offset: Vector3<f64>,
  pub scale: Vector3<f64>,
  pub env_vel: Vector3<f64>,
  pub on_ground: bool,
}

pub struct EntityRef<'a> {
  pub pos: &'a Point3<f64>,
  pub rot: &'a UnitQuaternion<f64>,
  pub vel: &'a Vector3<f64>,
  pub center_offset: &'a Vector3<f64>,
  pub scale: &'a Vector3<f64>,
  pub env_vel: &'a Vector3<f64>,
  pub on_ground: &'a bool,
}

pub struct EntityWorld {
  /// エンティティの座標
  pos: Vec<Point3<f64>>,

  /// エンティティの回転角度
  rot: Vec<UnitQuaternion<f64>>,

  /// エンティティの秒速(m/s)
  vel: Vec<Vector3<f64>>,

  /// エンティティの座標と中心のオフセット
  center_offset: Vec<Vector3<f64>>,

  /// エンティティの大きさ(m)
  scale: Vec<Vector3<f64>>,

  /// エンティティの環境速度(m/s)…重力など
  env_vel: Vec<Vector3<f64>>,

  /// エンティティの接地フラグ
  on_ground: Vec<bool>,

  /// プレイヤーデータ
  player: player::Player,
}
impl EntityWorld {
  pub fn new(_: ()) -> Self {
    let mut s = Self {
      pos: Vec::new(),
      rot: Vec::new(),
      vel: Vec::new(),
      center_offset: Vec::new(),
      scale: Vec::new(),
      env_vel: Vec::new(),
      on_ground: Vec::new(),
      player: player::Player::new(0),
    };
    s.spawn_entity(Entity {
      pos: [0., 0., 0.].into(),
      rot: UnitQuaternion::identity(),
      vel: Vector3::zeros(),
      center_offset: [0., 0., 0.9].into(),
      scale: [0.5, 0.5, 1.8].into(),
      env_vel: Vector3::zeros(),
      on_ground: false,
    });
    s
  }

  pub fn get<'a>(
    &'a self,
    index: u64,
  ) -> Option<EntityRef<'a>> {
    let pos = self.pos.get(index as usize)?;
    let rot = &self.rot[index as usize];
    let vel = &self.vel[index as usize];
    let center_offset =
      &self.center_offset[index as usize];
    let scale = &self.scale[index as usize];
    let env_vel = &self.env_vel[index as usize];
    let on_ground =
      &self.on_ground[index as usize];
    Some(EntityRef {
      pos,
      rot,
      vel,
      center_offset,
      scale,
      env_vel,
      on_ground,
    })
  }

  pub fn spawn_entity(
    &mut self,
    entity: Entity,
  ) -> usize {
    let i = self.pos.len();
    self.pos.push(entity.pos);
    self.rot.push(entity.rot);
    self.vel.push(entity.vel);
    self
      .center_offset
      .push(entity.center_offset);
    self.scale.push(entity.scale);
    self.env_vel.push(entity.env_vel);
    self.on_ground.push(entity.on_ground);
    i
  }

  pub fn entity_to_block_ray_check(
    &mut self,
    index: u64,
    blocks: &BlockWorld,
    time_delta: f64,
  ) {
    self.on_ground[index as usize] = false;
    let mut remain_vel =
      self.vel[index as usize] * time_delta;
    let mut result_vel = Vector3::zeros();
    let mut position_step =
      self.pos[index as usize];
    let mut counter = 10;
    while f64::EPSILON < remain_vel.magnitude()
      && 0 < counter
      && let Some((_, raycast_result, dir)) =
        blocks.entity_motion_choose_nearest(
          &position_step,
          &remain_vel,
          &self.scale[index as usize],
          Some(
            &self.center_offset[index as usize],
          ),
        )
    {
      // 接触判定の最大回数カウンタを減らす
      counter -= 1;
      result_vel +=
        remain_vel * raycast_result.t;
      position_step +=
        remain_vel * raycast_result.t;
      let blocked_vel =
        remain_vel * (1. - raycast_result.t);
      let face_normal =
        -dir.normal().dot(&blocked_vel)
          * dir.normal();

      remain_vel = blocked_vel + face_normal;

      // 重力ベクトルに対して面のなす角が30度以下なら重力を打ち消す
      if self.env_vel[index as usize]
        .angle(&-dir.normal())
        < std::f64::consts::FRAC_PI_3
      {
        self.env_vel[index as usize] =
          Vector3::zeros();
        self.on_ground[index as usize] = true;
      }
    }

    self.vel[index as usize] =
      (remain_vel + result_vel) / time_delta;
    self.pos[index as usize] +=
      remain_vel + result_vel;
  }

  pub fn cycle_update(
    &mut self,
    time_delta: f64,
    world_param: &WorldParam,
    blocks: &mut BlockWorld,
  ) {
    self.player.cycle_update();
    self.player.motion_update(
      &mut self.pos[self.player.index as usize],
      &mut self.vel[self.player.index as usize],
      &mut self.env_vel
        [self.player.index as usize],
      &mut self.rot[self.player.index as usize],
      self.on_ground
        [self.player.index as usize],
    );
    self.player.block_break(
      blocks,
      &self.pos[self.player.index as usize],
      &self.rot[self.player.index as usize],
    );
    self.player.block_put(
      blocks,
      &self.pos[self.player.index as usize],
      &self.rot[self.player.index as usize],
    );
    for i in 0..self.pos.len() {
      self.env_vel[i] += world_param
        .calcurate_gravity_accelaration(
          time_delta,
        );
      self.vel[i] += self.env_vel[i];
      self.entity_to_block_ray_check(
        i as u64, blocks, time_delta,
      );
    }
  }

  pub fn render_update(
    &self,
    wgpu_ctx: &WGPUCtx,
    camera: &mut CameraBundle,
  ) {
    camera.update(
      wgpu_ctx,
      &(self.pos[self.player.index as usize]
        + player::Player::VIEW_POINT_OFFSET)
        .cast(),
      &self.rot[self.player.index as usize]
        .cast(),
    );
  }

  pub fn key_input(
    &mut self,
    key_event: &KeyEvent,
  ) {
    self.player.input.key_input(key_event);
  }

  pub fn mouse_motion_input(
    &mut self,
    motion: (f64, f64),
  ) {
    self
      .player
      .input
      .mouse_motion_input(motion);
  }

  pub fn mouse_button_input(
    &mut self,
    state: ElementState,
    input: MouseButton,
  ) {
    self
      .player
      .input
      .mouse_button_input(state, input);
  }
}
