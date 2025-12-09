use winit::keyboard::{KeyCode, PhysicalKey};

/// プレイヤー移動用のキー入力
pub struct UserMoveControl {
  pub l: bool,
  pub r: bool,
  pub fw: bool,
  pub bw: bool,
  pub up: bool,
  pub dn: bool,

  pub rot_l: bool,
  pub rot_r: bool,
  pub rot_dn: bool,
  pub rot_up: bool,
}
impl UserMoveControl {
  pub fn new() -> Self {
    Self {
      l: false,
      r: false,
      fw: false,
      bw: false,
      up: false,
      dn: false,
      //
      rot_l: false,
      rot_r: false,
      rot_dn: false,
      rot_up: false,
    }
  }

  /// キー入力
  pub fn input(&mut self, key_event: &winit::event::KeyEvent) {
    let k = key_event.physical_key;
    let i =
      key_event.state == winit::event::ElementState::Pressed;
    if !key_event.repeat {
      match k {
        PhysicalKey::Code(KeyCode::KeyW) => self.fw = i,
        PhysicalKey::Code(KeyCode::KeyS) => self.bw = i,
        PhysicalKey::Code(KeyCode::KeyA) => self.l = i,
        PhysicalKey::Code(KeyCode::KeyD) => self.r = i,
        PhysicalKey::Code(KeyCode::Space) => self.up = i,
        PhysicalKey::Code(KeyCode::ShiftLeft) => self.dn = i,
        PhysicalKey::Code(KeyCode::KeyZ) => self.rot_l = i,
        PhysicalKey::Code(KeyCode::KeyC) => self.rot_r = i,
        PhysicalKey::Code(KeyCode::KeyV) => self.rot_dn = i,
        PhysicalKey::Code(KeyCode::KeyR) => self.rot_up = i,
        _ => {}
      }
    }
  }
}

/// プレイヤー入力の内マウス移動速度
pub struct UserControlMouseVelocity {
  input: [f64; 2],
}
impl UserControlMouseVelocity {
  pub fn new() -> Self {
    Self { input: [0., 0.] }
  }

  #[inline]
  pub fn input(&mut self, velocity: [f64; 2]) {
    self.input[0] += velocity[0];
    self.input[1] += velocity[1];
  }

  #[inline]
  pub fn reset(&mut self) {
    self.input[0] = 0.;
    self.input[1] = 0.;
  }
}

/// プレイヤー制御に関わる入力
pub struct UserControlInput {
  move_key: UserMoveControl,
  mouse_velocity: UserControlMouseVelocity,
  open_menu: bool,
}
impl UserControlInput {
  pub fn new() -> Self {
    Self {
      move_key: UserMoveControl::new(),
      mouse_velocity: UserControlMouseVelocity::new(),
      open_menu: false,
    }
  }

  /// キー入力
  pub fn key_input(
    &mut self,
    key_event: &winit::event::KeyEvent,
    window: &winit::window::Window,
  ) {
    match key_event.physical_key {
      winit::keyboard::PhysicalKey::Code(
        winit::keyboard::KeyCode::Escape,
      ) if !key_event.repeat => match match key_event.state {
        winit::event::ElementState::Pressed => {
          self.open_menu = true;
          window.set_cursor_visible(true);
          window.set_cursor_grab(
            winit::window::CursorGrabMode::None,
          )
        }
        winit::event::ElementState::Released => {
          self.open_menu = false;
          window.set_cursor_visible(false);
          window.set_cursor_grab(
            winit::window::CursorGrabMode::Confined,
          )
        }
      } {
        Ok(()) => {}
        Err(e) => {
          eprintln!("cursor grabmode change error: {e}")
        }
      },
      _ if !self.open_menu => self.move_key.input(key_event),
      _ => {}
    }
  }

  /// マウス入力
  #[inline]
  pub fn mouse_input(&mut self, velocity: [f64; 2]) {
    if !self.open_menu {
      self
        .mouse_velocity
        .input(velocity);
    }
  }

  /// 定期更新
  pub fn update(&mut self) {
    self.mouse_velocity.reset();
  }
}

pub struct Player {
  position: nalgebra::Point3<f64>,
  velocity: nalgebra::Vector3<f64>,
  yaw: f64,
  pitch: f64,
  roll: f64,
}
impl Player {
  pub fn new() -> Self {
    Self {
      position: [0., 0., 0.].into(),
      velocity: [0., 0., 0.].into(),
      yaw: 0.,
      pitch: 0.,
      roll: 0.,
    }
  }

  pub fn update(&mut self, input: &UserControlInput) {
    if input.move_key.l {
      self.velocity.x -= 5. / 60.
    }
    if input.move_key.r {
      self.velocity.x += 5. / 60.
    }
    if input.move_key.dn {
      self.velocity.z -= 5. / 60.
    }
    if input.move_key.up {
      self.velocity.z += 5. / 60.
    }
    if input.move_key.bw {
      self.velocity.y -= 5. / 60.
    }
    if input.move_key.fw {
      self.velocity.y += 5. / 60.
    }
    self.yaw = (self.yaw
      + (0.12
        * input.mouse_velocity.input[0]
        * std::f64::consts::PI
        / 180.))
      .rem_euclid(std::f64::consts::PI * 2.);
    self.pitch = (self.pitch
      + (0.08
        * -input.mouse_velocity.input[1]
        * std::f64::consts::PI
        / 180.))
      .clamp(
        -std::f64::consts::FRAC_PI_2,
        std::f64::consts::FRAC_PI_2,
      );
    if input.move_key.rot_l {
      self.yaw = (self.yaw
        + (90. * std::f64::consts::PI / 180.) / 60.)
        .rem_euclid(std::f64::consts::PI * 2.)
    }
    if input.move_key.rot_r {
      self.yaw = (self.yaw
        - (90. * std::f64::consts::PI / 180.) / 60.)
        .rem_euclid(std::f64::consts::PI * 2.)
    }
    if input.move_key.rot_dn {
      self.pitch = (self.pitch
        - (90. * std::f64::consts::PI / 180.) / 60.)
        .max(-std::f64::consts::FRAC_PI_2);
    }
    if input.move_key.rot_up {
      self.pitch = (self.pitch
        + (90. * std::f64::consts::PI / 180.) / 60.)
        .min(std::f64::consts::FRAC_PI_2);
    }
  }

  pub fn update_camera(
    &mut self,
    camera: &mut super::gfx::camera::CameraInstance,
  ) {
    let rotation = nalgebra::UnitQuaternion::from_axis_angle(
      &nalgebra::UnitVector3::new_normalize(
        nalgebra::Vector3::z(),
      ),
      self.yaw,
    );
    self.position = self.position + rotation * self.velocity;
    self.velocity = [0., 0., 0.].into();
    camera.position = self.position;
    camera.rotation = rotation
      * nalgebra::UnitQuaternion::from_axis_angle(
        &nalgebra::UnitVector3::new_normalize(
          nalgebra::Vector3::x(),
        ),
        self.pitch,
      )
      * nalgebra::UnitQuaternion::from_axis_angle(
        &nalgebra::UnitVector3::new_normalize(
          nalgebra::Vector3::y(),
        ),
        self.roll,
      );
  }
}
