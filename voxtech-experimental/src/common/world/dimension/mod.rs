use hashbrown::HashMap;

use crate::common::*;

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
  pub fn iter(
    &self,
  ) -> impl Iterator<Item = (&BlockPos, &l3_region::Region)>
  {
    self.map.iter()
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
  pub fn update_neigh_west(
    &mut self,
    target_region_pos: &BlockPos,
  ) -> bool {
    let target_region_pos = target_region_pos.cut_up(4);
    let neigh_pos = target_region_pos
      + BlockDist::new(-1, 0, 0).level_up(4);
    let target_halo = self
      .halo
      .get_mut(&target_region_pos);
    let neigh = self.map.get(&neigh_pos);
    match (target_halo, neigh) {
      (Some(target_halo), Some(neigh)) => {
        target_halo.update_neigh_west(neigh);
        true
      }
      _ => false,
    }
  }
  pub fn update_neigh_east(
    &mut self,
    target_region_pos: &BlockPos,
  ) -> bool {
    let target_region_pos = target_region_pos.cut_up(4);
    let neigh_pos = target_region_pos
      + BlockDist::new(1, 0, 0).level_up(4);
    let target_halo = self
      .halo
      .get_mut(&target_region_pos);
    let neigh = self.map.get(&neigh_pos);
    match (target_halo, neigh) {
      (Some(target_halo), Some(neigh)) => {
        target_halo.update_neigh_east(neigh);
        true
      }
      _ => false,
    }
  }
  pub fn update_neigh_south(
    &mut self,
    target_region_pos: &BlockPos,
  ) -> bool {
    let target_region_pos = target_region_pos.cut_up(4);
    let neigh_pos = target_region_pos
      + BlockDist::new(0, -1, 0).level_up(4);
    let target_halo = self
      .halo
      .get_mut(&target_region_pos);
    let neigh = self.map.get(&neigh_pos);
    match (target_halo, neigh) {
      (Some(target_halo), Some(neigh)) => {
        target_halo.update_neigh_south(neigh);
        true
      }
      _ => false,
    }
  }
  pub fn update_neigh_north(
    &mut self,
    target_region_pos: &BlockPos,
  ) -> bool {
    let target_region_pos = target_region_pos.cut_up(4);
    let neigh_pos = target_region_pos
      + BlockDist::new(0, 1, 0).level_up(4);
    let target_halo = self
      .halo
      .get_mut(&target_region_pos);
    let neigh = self.map.get(&neigh_pos);
    match (target_halo, neigh) {
      (Some(target_halo), Some(neigh)) => {
        target_halo.update_neigh_north(neigh);
        true
      }
      _ => false,
    }
  }
  pub fn update_neigh_bottom(
    &mut self,
    target_region_pos: &BlockPos,
  ) -> bool {
    let target_region_pos = target_region_pos.cut_up(4);
    let neigh_pos = target_region_pos
      + BlockDist::new(0, 0, -1).level_up(4);
    let target_halo = self
      .halo
      .get_mut(&target_region_pos);
    let neigh = self.map.get(&neigh_pos);
    match (target_halo, neigh) {
      (Some(target_halo), Some(neigh)) => {
        target_halo.update_neigh_bottom(neigh);
        true
      }
      _ => false,
    }
  }
  pub fn update_neigh_top(
    &mut self,
    target_region_pos: &BlockPos,
  ) -> bool {
    let target_region_pos = target_region_pos.cut_up(4);
    let neigh_pos = target_region_pos
      + BlockDist::new(0, 0, 1).level_up(4);
    let target_halo = self
      .halo
      .get_mut(&target_region_pos);
    let neigh = self.map.get(&neigh_pos);
    match (target_halo, neigh) {
      (Some(target_halo), Some(neigh)) => {
        target_halo.update_neigh_top(neigh);
        true
      }
      _ => false,
    }
  }
}
