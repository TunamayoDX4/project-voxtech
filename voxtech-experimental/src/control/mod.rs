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
  pub input: [f64; 2],
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
  pub move_key: UserMoveControl,
  pub mouse_velocity: UserControlMouseVelocity,
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

  /// エスケープキーの入力処理
  fn press_escape(
    &mut self,
    key_event: &winit::event::KeyEvent,
    window: &winit::window::Window,
  ) {
    match match key_event.state {
      // Escキーが入力された際のカーソルの再表示・グラブの解除
      winit::event::ElementState::Pressed => {
        self.open_menu = true;
        window.set_cursor_visible(true);
        window
          .set_cursor_grab(winit::window::CursorGrabMode::None)
      }

      // Escキーが離された際のカーソルの非表示・グラブの有効化
      winit::event::ElementState::Released => {
        self.open_menu = false;
        window.set_cursor_visible(false);
        window.set_cursor_grab(
          winit::window::CursorGrabMode::Confined,
        )
      }
    } {
      Ok(()) => {}

      // グラブ処理失敗時のメッセージ表示
      Err(e) => {
        eprintln!("cursor grabmode change error: {e}")
      }
    }
  }

  /// キー入力
  pub fn key_input(
    &mut self,
    key_event: &winit::event::KeyEvent,
    window: &winit::window::Window,
  ) {
    match key_event.physical_key {
      // Escapeキーを押したときにカーソルを表示する
      winit::keyboard::PhysicalKey::Code(
        winit::keyboard::KeyCode::Escape,
      ) if !key_event.repeat => {
        self.press_escape(key_event, window)
      }

      // 移動キーの入力処理
      _ if !self.open_menu => self.move_key.input(key_event),

      // メニューが開かれてるときの処理
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
