pub mod world {
  pub struct EntityStorage {}

  pub struct BlockStorage {}

  pub struct World {}
}

pub mod entity {
  use nalgebra::{Point3, UnitQuaternion, Vector3};

  #[derive(Debug, Clone, Copy)]
  pub struct EntityPhysics {
    pub position: Point3<f64>,
    pub scale: Vector3<f64>,
    pub rotation: UnitQuaternion<f64>,
    pub velocity: Vector3<f64>,
  }
  impl EntityPhysics {
    pub fn update(&mut self, time_delta: f64) {
      self.position += self.velocity * time_delta;
    }
  }
}

pub mod block {

  #[repr(C)]
  #[derive(Debug, Clone, Copy, Default)]
  pub struct BlockID(u8);
  impl BlockID {}
}

pub struct GameLogic {}
