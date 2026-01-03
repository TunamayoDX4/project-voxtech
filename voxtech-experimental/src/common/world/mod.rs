pub mod dimension;
use dimension::Dimension;

pub mod l0_cell;
pub mod l1_chunk;
pub mod l2_sector;
pub mod l3_region;

/// Worldはプログラム上における空間インスタンスのバインダ
/// World is the binder for dimension instances in the program.
pub struct World {
  pub dim: Dimension,
}
impl World {
  pub fn new() -> Self {
    Self {
      dim: Dimension::new(),
    }
  }
}

