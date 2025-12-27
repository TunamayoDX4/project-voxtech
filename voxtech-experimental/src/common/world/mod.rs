use hashbrown::HashMap;

use crate::common::*;

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

pub struct Dimension {
  map: HashMap<BlockPos, l3_region::Region>,
  halo: HashMap<BlockPos, l3_region::RegionHaloArray>,
}
impl Dimension {
  pub fn new() -> Self {
    Self {
      map: HashMap::new(),
      halo: HashMap::new(),
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
    self
      .halo
      .entry(region_pos)
      .insert(Default::default());
  }
  #[inline]
  pub fn get(
    &self,
    region_pos: &BlockPos,
  ) -> Option<(
    &l3_region::Region,
    &l3_region::RegionHaloArray,
  )> {
    let r = self
      .map
      .get(&region_pos.cut_up(4));
    let rha = self
      .halo
      .get(&region_pos.cut_up(4));
    let (Some(r), Some(rha)) = (r, rha) else {
      return None;
    };
    Some((r, rha))
  }
  #[inline]
  pub fn get_mut(
    &mut self,
    region_pos: &BlockPos,
  ) -> Option<(
    &mut l3_region::Region,
    &mut l3_region::RegionHaloArray,
  )> {
    let r = self
      .map
      .get_mut(&region_pos.cut_up(4));
    let rha = self
      .halo
      .get_mut(&region_pos.cut_up(4));
    let (Some(r), Some(rha)) = (r, rha) else {
      return None;
    };
    Some((r, rha))
  }
}
