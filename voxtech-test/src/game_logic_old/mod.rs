use std::ops::Range;

use nalgebra::{
  Point3, Quaternion, UnitQuaternion, UnitVector3,
  Vector3,
};
use parking_lot::Mutex;
use winit::{
  event::{ElementState, MouseButton},
  keyboard::KeyCode,
};

pub mod block;

#[derive(Debug, Clone)]
pub struct Player {
  pub position: Point3<f64>,
  pub rotation: UnitQuaternion<f64>,
  pub velocity: Vector3<f64>,
  pub input: PlayerInput,
}
impl Player {
  pub fn new(
    pos: impl Into<Point3<f64>>,
    rot: impl Into<UnitQuaternion<f64>>,
  ) -> Self {
    Self {
      position: pos.into(),
      rotation: rot.into(),
      velocity: Vector3::zeros(),
      input: Default::default(),
    }
  }

  pub fn update(&mut self, time_delta: f64) {
    self.input.apply_velocity(
      &mut self.velocity,
      &self.rotation,
    );
    self
      .input
      .apply_rotation(&mut self.rotation);
    let motion = self.velocity * time_delta;
    self.position += motion;
  }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct PlayerInput {
  move_flont: bool,
  move_back: bool,
  move_left: bool,
  move_right: bool,
  move_up: bool,
  move_down: bool,
  attack: bool,
  using: bool,
  yaw: f64,
  pitch: f64,

  on_ground: bool,
}
impl PlayerInput {
  pub fn keyboard_input(
    &mut self,
    key_event: winit::event::KeyEvent,
  ) {
    let flag = key_event.state
      == winit::event::ElementState::Pressed;
    let winit::keyboard::PhysicalKey::Code(key_code) =
      key_event.physical_key
    else {
      return;
    };
    match key_code {
      KeyCode::KeyW => self.move_flont = flag,
      KeyCode::KeyS => self.move_back = flag,
      KeyCode::KeyA => self.move_left = flag,
      KeyCode::KeyD => self.move_right = flag,
      KeyCode::Space => self.move_up = flag,
      KeyCode::ShiftLeft | KeyCode::ShiftRight => {
        self.move_down = flag
      }
      _ => {}
    }
  }

  pub fn mouse_button_input(
    &mut self,
    button: MouseButton,
    state: ElementState,
  ) {
    let flag = state == ElementState::Pressed;
    match button {
      MouseButton::Left => self.attack = flag,
      MouseButton::Right => self.using = flag,
      _ => {}
    }
  }

  pub fn mouse_motion_input(
    &mut self,
    delta: (f64, f64),
  ) {
    self.yaw = (self.yaw + delta.0)
      .rem_euclid(std::f64::consts::PI * 2.);
    self.pitch = (self.pitch + delta.1).clamp(
      -std::f64::consts::FRAC_PI_2,
      std::f64::consts::FRAC_PI_2,
    );
  }

  pub fn apply_velocity(
    &mut self,
    velocity: &mut Vector3<f64>,
    rotation: &UnitQuaternion<f64>,
  ) {
    *velocity = Vector3::zeros();
    if self.move_flont {
      *velocity += Vector3::x() * 5.;
    }
    if self.move_back {
      *velocity -= Vector3::x() * 5.;
    }
    if self.move_left {
      *velocity -= Vector3::y() * 5.;
    }
    if self.move_right {
      *velocity += Vector3::y() * 5.;
    }
    if self.move_up {
      *velocity += Vector3::z() * 5.;
    }
    if self.move_down {
      *velocity -= Vector3::z() * 5.;
    }
    *velocity = *rotation * *velocity;
  }

  pub fn apply_rotation(
    &mut self,
    rotation: &mut UnitQuaternion<f64>,
  ) {
    *rotation = UnitQuaternion::from_axis_angle(
      &UnitVector3::new_normalize(Vector3::z()),
      self.yaw,
    ) * UnitQuaternion::from_axis_angle(
      &UnitVector3::new_normalize(Vector3::y()),
      self.pitch,
    );
  }
}

pub struct World {
  pub player: Player,
  pub blocks: BlockStorage,
}
impl World {
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    let blocks =
      BlockStorage::new((256, 256, 32), |(x, y, z)| {
        if z == 0
          || x == 0
          || x == 255
          || y == 0
          || y == 255
        {
          return Block(0b101011);
        }

        let flip =
          ((z as u8 >> 4) & 1).wrapping_sub(1) & 63;

        let mask0 =
          ((x as u8 >> 2) & 1).wrapping_sub(1) & 63;
        let mask1 =
          ((y as u8 >> 2) & 1).wrapping_sub(1) & 63;
        let mask2 =
          ((z as u8 >> 2) & 1).wrapping_sub(1) & 63;

        let id = (x as u8 & 3
          | (y as u8 & 3) << 2
          | (z as u8 & 3) << 4)
          & ((mask0 ^ mask1 ^ mask2) ^ flip);

        let id = if (z >> 2) & 1 == 0
          && (x & 3 == 2 || y & 3 == 2 || z & 3 == 2)
        {
          0
        } else {
          id
        };

        Block(id)
      });
    let player = Player::new(
      [0., 0., 0.],
      UnitQuaternion::from_axis_angle(
        &UnitVector3::new_normalize(Vector3::z()),
        0.,
      ),
    );
    Self { blocks, player }
  }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block(u8);
impl Block {
  #[inline]
  pub fn new(id: u8) -> Self {
    Self(id)
  }

  #[inline]
  pub fn get(&self) -> u8 {
    self.0
  }
}

pub struct BlockStorage {
  scale: (u32, u32, u32),
  blocks: Vec<Block>,
  update_flg: Mutex<bool>,
}
impl BlockStorage {
  pub fn new(
    scale: (u32, u32, u32),
    f: impl FnMut((u32, u32, u32)) -> Block,
  ) -> Self {
    let blocks = (0..scale.2)
      .flat_map(|z| {
        (0..scale.1).flat_map(move |y| {
          (0..scale.0).map(move |x| (x, y, z))
        })
      })
      .map(f)
      .collect::<Vec<_>>();

    Self {
      scale,
      blocks,
      update_flg: Mutex::new(true),
    }
  }

  pub fn render_instances_update(
    &self,
    instances: &mut Vec<
      crate::gfx::renderer::tile::vertex::Instance,
    >,
  ) {
    let mut update_flg = self.update_flg.lock();
    if *update_flg {
      *update_flg = false;
      instances.clear();
      for (pos, block) in self
        .blocks
        .iter()
        .enumerate()
        .filter(|(_, b)| b.0 != 0)
        .map(|(i, b)| {
          (
            (
              i % self.scale.0 as usize,
              (i / self.scale.0 as usize)
                % self.scale.1 as usize,
              (i / (self.scale.0 * self.scale.1)
                as usize),
            ),
            b,
          )
        })
        .map(|(p, b)| {
          (
            (
              p.0 as f32, p.1 as f32, p.2 as f32,
            ),
            b,
          )
        })
      {
        instances.push(
        crate::gfx::renderer::tile::vertex::Instance {
          position: [pos.0, pos.1, pos.2, 0.],
          color: [
            (block.0 & 3) as f32 / 3.,
            ((block.0 >> 2) & 3) as f32 / 3.,
            ((block.0 >> 4) & 3) as f32 / 3.,
            1.,
          ],
        },
      )
      }
    }
  }

  pub fn calcurate_entity_range(
    &self,
    position: &Point3<f64>,
    scale: &Vector3<f64>,
  ) -> [Range<u32>; 3] {
    let half_scale = scale * 0.5;
    let p = [
      position - half_scale, //
      position + half_scale,
    ];
    let min = [
      p[0]
        .x
        .min(p[1].x)
        .max(0.)
        .floor() as u32, //
      p[0]
        .y
        .min(p[1].y)
        .max(0.)
        .floor() as u32, //
      p[0]
        .z
        .min(p[1].z)
        .max(0.)
        .floor() as u32,
    ];
    let max = [
      p[0]
        .x
        .max(p[1].x)
        .ceil()
        .min(self.scale.0 as f64) as u32,
      p[0]
        .y
        .max(p[1].y)
        .ceil()
        .min(self.scale.1 as f64) as u32,
      p[0]
        .z
        .max(p[1].z)
        .ceil()
        .min(self.scale.2 as f64) as u32,
    ];
    std::array::from_fn(|i| min[i]..max[i])
  }

  pub fn calcurate_velocity_applied_entity_range(
    &self,
    position: &Point3<f64>,
    scale: &Vector3<f64>,
    velocity: &Vector3<f64>,
  ) -> [Range<u32>; 3] {
    let half_scale = scale * 0.5;
    let p = [
      Point3::origin(),
      Point3::origin() + velocity,
    ];
    let p: [Vector3<f64>; 2] = [
      [
        p[0].x.min(p[1].x),
        p[0].y.min(p[1].y),
        p[0].z.min(p[1].z),
      ]
      .into(),
      [
        p[0].x.max(p[1].x),
        p[0].y.max(p[1].y),
        p[0].z.max(p[1].z),
      ]
      .into(),
    ];
    let p: [Point3<f64>; 2] = [
      position - half_scale + p[0],
      position + half_scale + p[1],
    ];
    let min = [
      p[0]
        .x
        .min(p[1].x)
        .max(0.)
        .floor() as u32, //
      p[0]
        .y
        .min(p[1].y)
        .max(0.)
        .floor() as u32, //
      p[0]
        .z
        .min(p[1].z)
        .max(0.)
        .floor() as u32,
    ];
    let max = [
      p[0]
        .x
        .max(p[1].x)
        .ceil()
        .min(self.scale.0 as f64) as u32,
      p[0]
        .y
        .max(p[1].y)
        .ceil()
        .min(self.scale.1 as f64) as u32,
      p[0]
        .z
        .max(p[1].z)
        .ceil()
        .min(self.scale.2 as f64) as u32,
    ];
    std::array::from_fn(|i| min[i]..max[i])
  }

  pub fn aabb_check(
    &self,
    position: &Point3<f64>,
    scale: &Vector3<f64>,
  ) -> bool {
    let range =
      self.calcurate_entity_range(position, scale);
    for x in range[0].clone() {
      for y in range[1].clone() {
        for z in range[2].clone() {
          let p = x as usize
            + (y * self.scale.0) as usize
            + (z * (self.scale.0 * self.scale.1))
              as usize;
          if self.blocks[p].0 != 0 {
            return true;
          }
        }
      }
    }
    false
  }

  pub fn raycast_check(
    &self,
    position: &Point3<f64>,
    scale: &Vector3<f64>,
    velocity: &Vector3<f64>,
  ) -> Option<(block::RayCastResult, block::Dir)> {
    let range = self
      .calcurate_velocity_applied_entity_range(
        position, scale, velocity,
      );
    let mut best_hit: Option<(
      block::RayCastResult,
      block::Dir,
    )> = None;
    for x in range[0].clone() {
      for y in range[1].clone() {
        for z in range[2].clone() {
          let p = x as usize
            + (y * self.scale.0) as usize
            + (z * (self.scale.0 * self.scale.1))
              as usize;

          // ID == 0 なら空気なのでスキップ
          if self.blocks[p].0 == 0 {
            continue;
          }

          let p =
            block::AbstractBlock::ray_block_normalize(
              &Point3::from([x as _, y as _, z as _]),
              &[*position, position + velocity],
              scale,
            );

          if let Some((rcr, dir)) =
            block::AbstractBlock::ray_check_opposing(&p)
          {
            let hit =
              block::RayCastResult::new(rcr, &p);
            if let Some(best_hit) = best_hit.as_mut() {
              if hit.t < best_hit.0.t {
                *best_hit = (hit, dir);
              }
            } else {
              best_hit = Some((hit, dir));
            }
          }
        }
      }
    }

    best_hit
  }
}
