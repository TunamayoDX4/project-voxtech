use parking_lot::RwLock;

use crate::common::{
  BlockDist, BlockPos, Dir, InnerBlockPos,
};

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
impl Region {
  pub fn iter_sector(
    &self,
    pos: &BlockPos,
  ) -> Option<
    impl Iterator<
      Item = (
        InnerBlockPos,
        BlockPos,
        &l2_sector::Sector,
      ),
    >,
  > {
    let Some(sector) = self.sector.as_ref() else {
      return None;
    };
    let ret = (0..sector.len())
      .map(|i| {
        let i = InnerBlockPos::new(i as _);
        let p = BlockDist::from(i).level_up(3);
        (i, p)
      })
      .map(move |(i, p)| (i, *pos + p))
      .map(|(i, p)| (i, p, &sector[i.0 as usize]));
    Some(ret)
  }

  pub fn update_neigh_west(&mut self, neigh: &Region) {
    let Some(sector) = self.sector.as_mut() else {
      return;
    };
    let sector_halo = self
      .sector_halo
      .get_or_insert_with(|| {
        Box::new(std::array::from_fn(|_| {
          Default::default()
        }))
      });
    for i in (0..3).rev() {
      for j in (0..16).map(|j| j * 4) {
        sector_halo[j + i + 1]
          .update_neigh_west(&sector[j + i]);
      }
    }
    if let Some(neigh) = neigh.sector.as_ref() {
      for i in (0..16).map(|i| i * 4) {
        sector_halo[i].update_neigh_west(&neigh[i + 3]);
      }
    }
  }
  pub fn update_neigh_east(&mut self, neigh: &Region) {
    let Some(sector) = self.sector.as_mut() else {
      return;
    };
    let sector_halo = self
      .sector_halo
      .get_or_insert_with(|| {
        Box::new(std::array::from_fn(|_| {
          Default::default()
        }))
      });
    for i in 0..3 {
      for j in (0..16).map(|j| j * 4) {
        sector_halo[j + i]
          .update_neigh_east(&sector[j + i + 1]);
      }
    }
    if let Some(neigh) = neigh.sector.as_ref() {
      for i in (0..16).map(|i| i * 4) {
        sector_halo[i + 3].update_neigh_east(&neigh[i]);
      }
    }
  }
  pub fn update_neigh_south(&mut self, neigh: &Region) {
    let Some(sector) = self.sector.as_mut() else {
      return;
    };
    let sector_halo = self
      .sector_halo
      .get_or_insert_with(|| {
        Box::new(std::array::from_fn(|_| {
          Default::default()
        }))
      });
    for i in (0..3).rev().map(|i| i * 4) {
      for j in
        (0..16).map(|j| ((j & 12) << 2) | (j & 3))
      {
        sector_halo[j + i + 4]
          .update_neigh_south(&sector[j + i]);
      }
    }
    if let Some(neigh) = neigh.sector.as_ref() {
      for i in
        (0..16).map(|j| ((j & 12) << 2) | (j & 3))
      {
        sector_halo[i]
          .update_neigh_south(&neigh[i + 3 * 4]);
      }
    }
  }
  pub fn update_neigh_north(&mut self, neigh: &Region) {
    let Some(sector) = self.sector.as_mut() else {
      return;
    };
    let sector_halo = self
      .sector_halo
      .get_or_insert_with(|| {
        Box::new(std::array::from_fn(|_| {
          Default::default()
        }))
      });
    for i in (0..3).map(|i| i * 4) {
      for j in
        (0..16).map(|j| ((j & 12) << 2) | (j & 3))
      {
        sector_halo[j + i]
          .update_neigh_north(&sector[j + i + 4]);
      }
    }
    if let Some(neigh) = neigh.sector.as_ref() {
      for i in
        (0..16).map(|j| ((j & 12) << 2) | (j & 3))
      {
        sector_halo[i + 3 * 4]
          .update_neigh_north(&neigh[i]);
      }
    }
  }
  pub fn update_neigh_bottom(
    &mut self,
    neigh: &Region,
  ) {
    let Some(sector) = self.sector.as_mut() else {
      return;
    };
    let sector_halo = self
      .sector_halo
      .get_or_insert_with(|| {
        Box::new(std::array::from_fn(|_| {
          Default::default()
        }))
      });
    for i in (0..3).rev().map(|i| i * 16) {
      for j in 0..16 {
        sector_halo[j + i + 16]
          .update_neigh_bottom(&sector[j + i]);
      }
    }
    if let Some(neigh) = neigh.sector.as_ref() {
      for i in 0..16 {
        sector_halo[i]
          .update_neigh_bottom(&neigh[i + 3 * 16]);
      }
    }
  }
  pub fn update_neigh_top(&mut self, neigh: &Region) {
    let Some(sector) = self.sector.as_mut() else {
      return;
    };
    let sector_halo = self
      .sector_halo
      .get_or_insert_with(|| {
        Box::new(std::array::from_fn(|_| {
          Default::default()
        }))
      });
    for i in (0..3).map(|i| i * 16) {
      for j in 0..16 {
        sector_halo[j + i]
          .update_neigh_top(&sector[j + i + 16]);
      }
    }
    if let Some(neigh) = neigh.sector.as_ref() {
      for i in 0..16 {
        sector_halo[i + 3 * 16]
          .update_neigh_top(&neigh[i]);
      }
    }
  }

  #[inline]
  pub fn chk_visible_face(
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> [bool; Dir::COUNT as usize] {
    [
      Self::chk_visible_west_face(relative_camera_pos),
      Self::chk_visible_east_face(relative_camera_pos),
      Self::chk_visible_south_face(relative_camera_pos),
      Self::chk_visible_north_face(relative_camera_pos),
      Self::chk_visible_bottom_face(
        relative_camera_pos,
      ),
      Self::chk_visible_top_face(relative_camera_pos),
    ]
  }

  /// 対象のリージョンの西面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_west_face(
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin =
      nalgebra::Point3::new(128., 128., 128.);
    let vn = -nalgebra::Vector3::x();
    let vp = relative_camera_pos - origin;
    println!("{relative_camera_pos:?}, {vn:?}, {vp:?}");
    vn.angle(&vp) < std::f64::consts::FRAC_PI_2
  }

  /// 対象のリージョンの東面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_east_face(
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin = nalgebra::Point3::new(0., 128., 128.);
    let vn = nalgebra::Vector3::x();
    let vp = relative_camera_pos - origin;
    vn.angle(&vp) < std::f64::consts::FRAC_PI_2
  }

  /// 対象のリージョンの南面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_south_face(
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin =
      nalgebra::Point3::new(128., 256., 128.);
    let vn = -nalgebra::Vector3::y();
    let vp = relative_camera_pos - origin;
    vn.angle(&vp) < std::f64::consts::FRAC_PI_2
  }

  /// 対象のリージョンの北面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_north_face(
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin = nalgebra::Point3::new(128., 0., 128.);
    let vn = nalgebra::Vector3::y();
    let vp = relative_camera_pos - origin;
    vn.angle(&vp) < std::f64::consts::FRAC_PI_2
  }

  /// 対象のリージョンの下面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_bottom_face(
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin =
      nalgebra::Point3::new(128., 128., 256.);
    let vn = -nalgebra::Vector3::z();
    let vp = relative_camera_pos - origin;
    vn.angle(&vp) < std::f64::consts::FRAC_PI_2
  }

  /// 対象のリージョンの上面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_top_face(
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin = nalgebra::Point3::new(128., 128., 0.);
    let vn = nalgebra::Vector3::z();
    let vp = relative_camera_pos - origin;
    vn.angle(&vp) < std::f64::consts::FRAC_PI_2
  }
}

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
