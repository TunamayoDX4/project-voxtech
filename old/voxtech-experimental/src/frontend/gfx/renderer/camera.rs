use super::{WGPUCtx, util};

pub struct WRCamera {
  camera: util::camera::Camera3DInstance,
  camera_cfg: util::camera::Camera3DConfig,
  camera_uniform: util::camera::Camera3DUniformInstance,
}
impl WRCamera {
  pub fn new(ctx: &WGPUCtx) -> Self {
    let camera = util::camera::Camera3DInstance {
      position: [0., 0., 0.].into(),
      rotation:
        nalgebra::UnitQuaternion::from_axis_angle(
          &nalgebra::UnitVector3::new_normalize(
            nalgebra::Vector3::z(),
          ),
          0.,
        ),
    };
    let camera_cfg = util::camera::Camera3DConfig {
      fovy: 45. * (std::f64::consts::PI / 180.),
      near: 0.5,
      far: 5000.,
    };
    let camera_uniform =
      util::camera::Camera3DUniformInstance::new(
        ctx,
        &camera_cfg,
        &camera,
      );
    Self {
      camera,
      camera_cfg,
      camera_uniform,
    }
  }
  pub fn update(&mut self, ctx: &WGPUCtx) {
    self.camera_uniform.update(
      ctx,
      &self.camera_cfg,
      &self.camera,
    );
  }
}
