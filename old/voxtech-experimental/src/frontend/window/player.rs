use crate::common::config::*;
use std::sync::LazyLock;

/// 物理設定のコンフィグ
static PHYSIC_CONFIG: LazyLock<PhysicsConfig> =
  LazyLock::new(|| {
    PhysicsConfig::initialize()
      .expect("physics config initialize failure.")
  });

/// キー入力モード
#[derive(
  Debug, Clone, Copy, Default, PartialEq, Eq,
)]
pub enum KeyInputMode {
  #[default]
  Released,
  Pressing,
}
impl KeyInputMode {
  pub fn press(&mut self) {
    if let KeyInputMode::Released = self {
      *self = KeyInputMode::Pressing;
    }
  }
  pub fn release(&mut self) {
    if let KeyInputMode::Pressing = self {
      *self = KeyInputMode::Released;
    }
  }
  pub fn input(
    &mut self,
    state: winit::event::ElementState,
  ) {
    match state {
      winit::event::ElementState::Pressed => {
        self.press()
      }
      winit::event::ElementState::Released => {
        self.release()
      }
    }
  }
  pub fn is_pressing(&self) -> bool {
    Self::Pressing == *self
  }
}

#[derive(Debug)]
pub struct PlayerState {
  last_update_timestamp: std::time::Instant,
  pub phys: PlayerPhysicState,
  key_input: PlayerKeyInput,
}
impl Default for PlayerState {
  fn default() -> Self {
    Self {
      last_update_timestamp: std::time::Instant::now(),
      phys: Default::default(),
      key_input: Default::default(),
    }
  }
}
impl PlayerState {
  pub fn input(
    &mut self,
    key_event: winit::event::KeyEvent,
  ) {
    self.key_input.input(key_event);
  }

  pub fn update(&mut self) {
    let now = std::time::Instant::now();
    let dur = now - self.last_update_timestamp;
    self.last_update_timestamp = now;
    let dur_sec =
      dur.as_nanos() as f64 / 1_000_000_000f64;
    self
      .phys
      .update(dur_sec, &self.key_input);
  }
}

#[derive(Debug, Default)]
pub struct PlayerPhysicState {
  pub pos: nalgebra::Point3<f64>,
  pub yaw: f64,
  pub pitch: f64,
}
impl PlayerPhysicState {
  pub fn mouse_input(&mut self, delta: (f64, f64)) {
    self.yaw = f64::rem_euclid(
      self.yaw
        - PHYSIC_CONFIG
          .calc_angle_from_mouse_motion(delta.0),
      std::f64::consts::PI * 2f64,
    );
    self.pitch = f64::clamp(
      self.pitch
        - PHYSIC_CONFIG
          .calc_angle_from_mouse_motion(delta.1),
      -std::f64::consts::FRAC_PI_2,
      std::f64::consts::FRAC_PI_2,
    );
  }

  pub fn update(
    &mut self,
    dur_sec: f64,
    key_input: &PlayerKeyInput,
  ) {
    let vel_y = if key_input
      .move_forward
      .is_pressing()
    {
      PHYSIC_CONFIG.walk_speed_mps
    } else {
      0f64
    } + if key_input
      .move_backward
      .is_pressing()
    {
      -PHYSIC_CONFIG.walk_speed_mps
    } else {
      0f64
    };
    let vel_x = if key_input
      .move_right
      .is_pressing()
    {
      PHYSIC_CONFIG.walk_speed_mps
    } else {
      0f64
    } + if key_input
      .move_left
      .is_pressing()
    {
      -PHYSIC_CONFIG.walk_speed_mps
    } else {
      0f64
    };
    let vel_z = if key_input.jump.is_pressing() {
      PHYSIC_CONFIG.walk_speed_mps
    } else {
      0f64
    } + if key_input.sneak.is_pressing() {
      -PHYSIC_CONFIG.walk_speed_mps
    } else {
      0f64
    };
    let vel =
      nalgebra::Vector3::new(vel_x, vel_y, vel_z)
        * dur_sec;
    let quat =
      nalgebra::UnitQuaternion::from_axis_angle(
        &nalgebra::UnitVector3::new_normalize(
          nalgebra::Vector3::z(),
        ),
        self.yaw,
      );
    self.pos += quat * vel;
  }
}

#[derive(Debug, Default)]
pub struct PlayerKeyInput {
  move_forward: KeyInputMode,
  move_backward: KeyInputMode,
  move_left: KeyInputMode,
  move_right: KeyInputMode,
  jump: KeyInputMode,
  sneak: KeyInputMode,
}
impl PlayerKeyInput {
  pub fn input(
    &mut self,
    key_event: winit::event::KeyEvent,
  ) {
    match key_event.physical_key {
      winit::keyboard::PhysicalKey::Code(
        winit::keyboard::KeyCode::KeyW,
      ) => self
        .move_forward
        .input(key_event.state),
      winit::keyboard::PhysicalKey::Code(
        winit::keyboard::KeyCode::KeyS,
      ) => self
        .move_backward
        .input(key_event.state),
      winit::keyboard::PhysicalKey::Code(
        winit::keyboard::KeyCode::KeyA,
      ) => self
        .move_left
        .input(key_event.state),
      winit::keyboard::PhysicalKey::Code(
        winit::keyboard::KeyCode::KeyD,
      ) => self
        .move_right
        .input(key_event.state),
      winit::keyboard::PhysicalKey::Code(
        winit::keyboard::KeyCode::Space,
      ) => self.jump.input(key_event.state),
      winit::keyboard::PhysicalKey::Code(
        winit::keyboard::KeyCode::ShiftLeft,
      ) => self
        .sneak
        .input(key_event.state),
      _ => {}
    }
  }
}

#[derive(
  serde::Serialize, serde::Deserialize, Debug,
)]
pub struct PhysicsConfig {
  pub walk_speed_mps: f64,
  pub jump_height_m: f64,
  pub mouse_sensi_pixels: f64,
  pub gravity_mps2: f64,
}
impl PhysicsConfig {
  #[inline]
  pub fn jump_velocity(&self) -> f64 {
    f64::sqrt(
      2f64 * self.gravity_mps2 * self.jump_height_m,
    )
  }
  #[inline]
  pub fn calc_angle_from_mouse_motion(
    &self,
    mouse_motion: f64,
  ) -> f64 {
    (mouse_motion / self.mouse_sensi_pixels)
      * std::f64::consts::PI
  }
}
impl AutoLoadConfig for PhysicsConfig {
  fn config_file_name() -> std::borrow::Cow<'static, str>
  {
    "physics_config.json".into()
  }

  fn allow_default() -> bool {
    true
  }
}
impl Default for PhysicsConfig {
  fn default() -> Self {
    Self {
      walk_speed_mps: 8.,
      jump_height_m: 1.2,
      mouse_sensi_pixels: 1000.0,
      gravity_mps2: 9.8,
    }
  }
}
