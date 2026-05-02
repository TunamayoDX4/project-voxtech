use nalgebra::{
  Point3, UnitQuaternion, UnitVector3, Vector3,
};
use winit::{
  event::{
    ElementState, KeyEvent, MouseButton,
  },
  keyboard::{KeyCode, PhysicalKey},
};
use super::super::block_world::block::ray_check::Dir;

pub struct Player {
  pub index: u64,
  pub input: PlayerInput,
}
impl Player {
  pub const VIEW_POINT_OFFSET: Vector3<f64> =
    Vector3::new(0., 0., 1.5);
  pub fn new(index: u64) -> Self {
    Self {
      index,
      input: PlayerInput::default(),
    }
  }

  pub fn cycle_update(&mut self) {
    self.input.cycle_update();
  }

  pub fn block_break(
    &self,
    blocks: &mut super::BlockWorld,
    pos: &Point3<f64>,
    rot: &UnitQuaternion<f64>,
  ) {
    if self.input.mouse_left
      && self.input.mouse_left_count == 1
    {
      let ray = Vector3::x() * 3.5;
      let vec = rot * ray;
      if let Some((pos, _, _)) = blocks
        .choose_raycheck_nearest(
          *pos + Self::VIEW_POINT_OFFSET,
          vec,
        )
        && let Some(lo_id) =
          blocks.remove_block(pos)
      {
        println!(
          "\x1b[33m{pos:?}, {lo_id:?}\x1b[37m"
        );
      }
    }
  }

  pub fn block_put(
    &self,
    blocks: &mut super::BlockWorld,
    pos: &Point3<f64>,
    rot: &UnitQuaternion<f64>,
  ) {
    if self.input.mouse_right
      && self.input.mouse_right_count == 1
    {
      let ray = Vector3::x() * 3.5;
      let vec = rot * ray;
      if let Some((pos, _, dir)) = blocks
        .choose_raycheck_nearest(
          *pos + Self::VIEW_POINT_OFFSET,
          vec,
        )
      {
        let pos_stride = match dir {
          Dir::EAST => [-1, 0, 0],
          Dir::WEST => [1, 0, 0],
          Dir::SOUTH => [0, -1, 0],
          Dir::NORTH => [0, 1, 0],
          Dir::BOTTOM => [0, 0, -1],
          Dir::TOP => [0, 0, 1],
        };
        let pos = std::array::from_fn(|i| {
          pos[i] + pos_stride[i]
        });
        blocks.put_block(pos);
      }
    }
  }

  pub fn motion_update(
    &self,
    _pos: &mut Point3<f64>,
    vel: &mut Vector3<f64>,
    env_vel: &mut Vector3<f64>,
    rot: &mut UnitQuaternion<f64>,
    on_ground: bool,
  ) {
    // ヨー回転角の入力
    *rot = UnitQuaternion::from_axis_angle(
      &UnitVector3::new_normalize(Vector3::z()),
      self.input.yaw,
    );

    // 速度の入力
    *vel = Vector3::zeros();
    if self.input.move_front {
      vel.x = 1.;
    } else if self.input.move_back {
      vel.x = -1.;
    }
    if self.input.move_left {
      vel.y = -1.;
    } else if self.input.move_right {
      vel.y = 1.;
    }
    if self.input.move_up && on_ground {
      env_vel.z = 8.;
    }
    if f64::EPSILON < vel.magnitude() {
      *vel = vel.normalize();
    }
    *vel *= 5.;
    *vel = *rot * *vel;

    // ピッチ回転角の入力
    *rot *= UnitQuaternion::from_axis_angle(
      &UnitVector3::new_normalize(Vector3::y()),
      self.input.pitch,
    );
  }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct PlayerInput {
  pub move_front: bool,
  pub move_back: bool,
  pub move_left: bool,
  pub move_right: bool,
  pub move_up: bool,
  pub move_down: bool,
  pub yaw: f64,
  pub pitch: f64,
  pub mouse_left: bool,
  pub mouse_left_count: u64,
  pub mouse_right: bool,
  pub mouse_right_count: u64,
}
impl PlayerInput {
  pub fn cycle_update(&mut self) {
    if self.mouse_left {
      self.mouse_left_count += 1;
    } else {
      self.mouse_left_count = 0;
    }

    if self.mouse_right {
      self.mouse_right_count += 1;
    } else {
      self.mouse_right_count = 0;
    }
  }

  pub fn key_input(
    &mut self,
    key_event: &KeyEvent,
  ) {
    let pressed =
      key_event.state == ElementState::Pressed;
    match key_event.physical_key {
      PhysicalKey::Code(KeyCode::KeyW) => {
        self.move_front = pressed
      }
      PhysicalKey::Code(KeyCode::KeyS) => {
        self.move_back = pressed
      }
      PhysicalKey::Code(KeyCode::KeyA) => {
        self.move_left = pressed
      }
      PhysicalKey::Code(KeyCode::KeyD) => {
        self.move_right = pressed
      }
      PhysicalKey::Code(KeyCode::Space) => {
        self.move_up = pressed
      }
      PhysicalKey::Code(
        KeyCode::ShiftLeft
        | KeyCode::ShiftRight,
      ) => self.move_down = pressed,
      _ => {}
    }
  }
  pub fn mouse_motion_input(
    &mut self,
    motion: (f64, f64),
  ) {
    self.yaw = (self.yaw + motion.0 * 0.002)
      .rem_euclid(std::f64::consts::PI * 2.);
    self.pitch =
      (self.pitch + motion.1 * 0.002).clamp(
        -std::f64::consts::FRAC_PI_2,
        std::f64::consts::FRAC_PI_2,
      );
  }
  pub fn mouse_button_input(
    &mut self,
    state: ElementState,
    input: MouseButton,
  ) {
    let pressed =
      state == ElementState::Pressed;
    match input {
      MouseButton::Left => {
        self.mouse_left = pressed
      }
      MouseButton::Right => {
        self.mouse_right = pressed
      }
      _ => {}
    }
  }
}
