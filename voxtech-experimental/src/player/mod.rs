use crate::control::UserControlInput;

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
      - (0.12
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
  }

  pub fn update_camera(
    &mut self,
    camera: &mut super::gfx::camera::CameraInstance,
  ) {
    let rotation =
      nalgebra::UnitQuaternion::from_axis_angle(
        &nalgebra::UnitVector3::new_normalize(
          nalgebra::Vector3::z(),
        ),
        self.yaw,
      );
    self.position =
      self.position + rotation * self.velocity;
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
