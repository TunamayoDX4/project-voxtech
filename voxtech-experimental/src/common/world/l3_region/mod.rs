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
  pub sector:
    RwLock<Option<Box<[l2_sector::Sector; 64]>>>,
  pub sector_halo: RwLock<
    Option<Box<[l2_sector::SectorHaloArray; 64]>>,
  >,
}
impl Region {
  pub fn iter_sector(
    &self,
    pos: &BlockPos,
    mut f: impl FnMut(
      InnerBlockPos,
      BlockPos,
      &l2_sector::Sector,
    ) -> bool,
  ) {
    let sector = self.sector.read();
    let Some(sector) = sector.as_ref() else {
      return;
    };
    for (ipos, bpos, chunk) in (0..sector.len())
      .map(|i| {
        let i = InnerBlockPos::new(i as _);
        let p = BlockDist::from(i).level_up(3);
        (i, p)
      })
      .map(move |(i, p)| (i, *pos + p))
      .map(|(i, p)| (i, p, &sector[i.0 as usize]))
    {
      if !f(ipos, bpos, chunk) {
        break;
      }
    }
  }

  pub fn update_neigh_west(
    &self,
    neigh: Option<&Region>,
  ) {
    let mut sector = self.sector.write();
    let Some(sector) = sector.as_mut() else {
      return;
    };
    let mut sector_halo = self.sector_halo.write();
    let sector_halo =
      sector_halo.get_or_insert_with(|| {
        Box::new(std::array::from_fn(|_| {
          Default::default()
        }))
      });
    for x in (0..3).rev() {
      for yz in (0..16).map(|yz| yz * 4) {
        //sector_halo[yz + x]
        //  .update_neigh_west(&sector[yz + x + 1]);
        // sector[yz + x].update_neigh_west(None);
      }
    }
    for p in 0..64 {
      sector[p].update_neigh_west(None);
    }
    if let Some(neigh) =
      neigh.map(|neigh| neigh.sector.read())
    {
      if let Some(neigh) = neigh.as_ref() {
        for i in (0..16).map(|i| i * 4) {
          sector_halo[i]
            .update_neigh_west(&neigh[i + 3]);
        }
      }
    }
  }
  pub fn update_neigh_east(
    &self,
    neigh: Option<&Region>,
  ) {
    let mut sector = self.sector.write();
    let Some(sector) = sector.as_mut() else {
      return;
    };
    let mut sector_halo = self.sector_halo.write();
    let sector_halo =
      sector_halo.get_or_insert_with(|| {
        Box::new(std::array::from_fn(|_| {
          Default::default()
        }))
      });
    for x in 0..3 {
      for yz in (0..16).map(|yz| yz * 4) {
        sector_halo[yz + x + 1]
          .update_neigh_east(&sector[yz + x]);
        sector[yz + x + 1].update_neigh_east(Some(
          &sector_halo[yz + x].0[Dir::EST as usize],
        ));
      }
    }
    if let Some(neigh) =
      neigh.map(|neigh| neigh.sector.read())
    {
      if let Some(neigh) = neigh.as_ref() {
        for i in (0..16).map(|i| i * 4) {
          sector_halo[i + 3]
            .update_neigh_east(&neigh[i]);
        }
      }
    }
  }
  /*
  pub fn update_neigh_south(
    &mut self,
    neigh: Option<&Region>,
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
    for y in (0..3).rev().map(|y| y * 4) {
      for xz in
        (0..16).map(|xz| ((xz & 12) << 2) | (xz & 3))
      {
        sector_halo[xz + y]
          .update_neigh_south(&sector[xz + y + 4]);
      }
    }
    if let Some(neigh) = neigh
      .as_ref()
      .map(|r| r.sector.as_ref())
      .flatten()
    {
      for i in
        (0..16).map(|j| ((j & 12) << 2) | (j & 3))
      {
        sector_halo[i]
          .update_neigh_south(&neigh[i + 3 * 4]);
      }
    }
  }
  pub fn update_neigh_north(
    &mut self,
    neigh: Option<&Region>,
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
    for y in (0..3).rev().map(|y| y * 4) {
      for xz in
        (0..16).map(|xz| ((xz & 12) << 2) | (xz & 3))
      {
        sector_halo[xz + y + 4]
          .update_neigh_north(&sector[xz + y]);
      }
    }
    if let Some(neigh) = neigh
      .as_ref()
      .map(|r| r.sector.as_ref())
      .flatten()
    {
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
    neigh: Option<&Region>,
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
    for z in (0..3).rev().map(|z| z * 16) {
      for xy in 0..16 {
        sector_halo[xy + z]
          .update_neigh_bottom(&sector[xy + z + 16]);
      }
    }
    if let Some(neigh) = neigh
      .as_ref()
      .map(|r| r.sector.as_ref())
      .flatten()
    {
      for i in 0..16 {
        sector_halo[i]
          .update_neigh_bottom(&neigh[i + 3 * 16]);
      }
    }
  }
  pub fn update_neigh_top(
    &mut self,
    neigh: Option<&Region>,
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
    for z in (0..3).rev().map(|z| z * 16) {
      for xy in 0..16 {
        sector_halo[xy + z + 16]
          .update_neigh_top(&sector[xy + z]);
      }
    }
    if let Some(neigh) = neigh
      .as_ref()
      .map(|r| r.sector.as_ref())
      .flatten()
    {
      for i in 0..16 {
        sector_halo[i + 3 * 16]
          .update_neigh_top(&neigh[i]);
      }
    }
  } */

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
      nalgebra::Point3::new(256., 128., 128.);
    let vp = relative_camera_pos - origin;
    vp.x <= f64::EPSILON
  }

  /// 対象のリージョンの東面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_east_face(
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin = nalgebra::Point3::new(0., 128., 128.);
    let vp = relative_camera_pos - origin;
    -f64::EPSILON <= vp.x
  }

  /// 対象のリージョンの南面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_south_face(
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin =
      nalgebra::Point3::new(128., 256., 128.);
    let vp = relative_camera_pos - origin;
    vp.y <= f64::EPSILON
  }

  /// 対象のリージョンの北面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_north_face(
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin = nalgebra::Point3::new(128., 0., 128.);
    let vp = relative_camera_pos - origin;
    -f64::EPSILON <= vp.y
  }

  /// 対象のリージョンの下面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_bottom_face(
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin =
      nalgebra::Point3::new(128., 128., 256.);
    let vp = relative_camera_pos - origin;
    vp.z <= f64::EPSILON
  }

  /// 対象のリージョンの上面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_top_face(
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin = nalgebra::Point3::new(128., 128., 0.);
    let vp = relative_camera_pos - origin;
    -f64::EPSILON <= vp.z
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
        .read()
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
        .read()
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
        .read()
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
        .read()
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
        .read()
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
        .read()
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
