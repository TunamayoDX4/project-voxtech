use hashbrown::HashMap;

use crate::common::*;

pub mod l0_cell;
pub mod l1_chunk;
pub mod l2_sector;
pub mod l3_region;

/// Worldはプログラム上における空間インスタンスのバインダ
/// World is the binder for dimension instances in the program.
pub struct World {
  dim: Dimension, 
}
impl World {
  pub fn new() -> Self {
    Self {
      dim: Dimension::new(), 
    }
  }
}

pub struct Dimension {
  map: HashMap<BlockPos, l3_region::Region>, 
}
impl Dimension {
  pub fn new() -> Self {
    Self {
      map: HashMap::new(), 
    }
  }
  pub fn spawn_region(
    &mut self, 
    region_pos: BlockPos, 
    f: impl FnOnce(BlockPos) -> l3_region::Region, 
  ) {
    // 座標を256m単位にするために、下位8bitを切り上げる
    let region_pos = region_pos.cut_up(4);
    self
      .map
      .entry(region_pos)
      .insert(f(region_pos));
  }
}