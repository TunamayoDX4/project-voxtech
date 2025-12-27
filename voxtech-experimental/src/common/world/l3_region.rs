use crate::common::{Dir, InnerBlockPos};

use super::{
  l0_cell,   //
  l1_chunk,  //
  l2_sector, //
};

pub mod cell_storage;

pub struct Region {
  pub sector: Option<Box<[l2_sector::Sector; 64]>>,
  pub sector_halo:
    Option<Box<[l2_sector::SectorHaloArray; 64]>>,
}
impl Region {}

pub struct RegionHaloArray([RegionHalo; 6]);
impl Default for RegionHaloArray {
  fn default() -> Self {
    Self(Default::default())
  }
}
impl RegionHaloArray {
  #[inline]
  pub fn update_neigh_west(&mut self, neigh: &Region) {
    self.0[Dir::WST as usize] =
      RegionHalo::make_halo_east(neigh);
  }
  #[inline]
  pub fn update_neigh_east(&mut self, neigh: &Region) {
    self.0[Dir::EST as usize] =
      RegionHalo::make_halo_west(neigh);
  }
  #[inline]
  pub fn update_neigh_south(&mut self, neigh: &Region) {
    self.0[Dir::STH as usize] =
      RegionHalo::make_halo_north(neigh);
  }
  #[inline]
  pub fn update_neigh_north(&mut self, neigh: &Region) {
    self.0[Dir::NTH as usize] =
      RegionHalo::make_halo_south(neigh);
  }
  #[inline]
  pub fn update_neigh_bottom(
    &mut self,
    neigh: &Region,
  ) {
    self.0[Dir::BTM as usize] =
      RegionHalo::make_halo_top(neigh);
  }
  #[inline]
  pub fn update_neigh_top(&mut self, neigh: &Region) {
    self.0[Dir::TOP as usize] =
      RegionHalo::make_halo_bottom(neigh);
  }
}

pub struct RegionHalo {
  pub sector: Option<Box<[l2_sector::SectorHalo; 16]>>,
}
impl Default for RegionHalo {
  fn default() -> Self {
    Self { sector: None }
  }
}
impl RegionHalo {
  #[inline]
  pub fn make_halo_west(region: &Region) -> Self {
    Self {
      sector: region
        .sector
        .as_ref()
        .map(|s| {
          std::array::from_fn(|i| {
            l2_sector::SectorHalo::make_halo_west(
              &s[i * 4],
            )
          })
        })
        .map(|s| Box::new(s)),
    }
  }
  #[inline]
  pub fn make_halo_east(region: &Region) -> Self {
    Self {
      sector: region
        .sector
        .as_ref()
        .map(|s| {
          std::array::from_fn(|i| {
            l2_sector::SectorHalo::make_halo_east(
              &s[i * 4 + 3],
            )
          })
        })
        .map(|s| Box::new(s)),
    }
  }
  #[inline]
  pub fn make_halo_south(region: &Region) -> Self {
    Self {
      sector: region
        .sector
        .as_ref()
        .map(|s| {
          std::array::from_fn(|i| {
            let broad = (i & !3) * 4;
            let narrow = i % 4;
            l2_sector::SectorHalo::make_halo_south(
              &s[broad + narrow],
            )
          })
        })
        .map(|s| Box::new(s)),
    }
  }
  #[inline]
  pub fn make_halo_north(region: &Region) -> Self {
    Self {
      sector: region
        .sector
        .as_ref()
        .map(|s| {
          std::array::from_fn(|i| {
            let broad = (i & !3) * 4;
            let narrow = i % 4;
            l2_sector::SectorHalo::make_halo_north(
              &s[broad + narrow + 12],
            )
          })
        })
        .map(|s| Box::new(s)),
    }
  }
  #[inline]
  pub fn make_halo_bottom(region: &Region) -> Self {
    Self {
      sector: region
        .sector
        .as_ref()
        .map(|s| {
          std::array::from_fn(|i| {
            l2_sector::SectorHalo::make_halo_bottom(
              &s[i],
            )
          })
        })
        .map(|s| Box::new(s)),
    }
  }
  #[inline]
  pub fn make_halo_top(region: &Region) -> Self {
    Self {
      sector: region
        .sector
        .as_ref()
        .map(|s| {
          std::array::from_fn(|i| {
            l2_sector::SectorHalo::make_halo_top(
              &s[i + 48],
            )
          })
        })
        .map(|s| Box::new(s)),
    }
  }
}
