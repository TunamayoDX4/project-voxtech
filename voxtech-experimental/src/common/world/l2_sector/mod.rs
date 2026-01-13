use parking_lot::RwLock;

use crate::{
  common::{
    l1_chunk::ChunkHalo, BlockDist, BlockPos, Dir,
    InnerBlockPos,
  },
  PRwLock,
};

use super::l1_chunk;

pub struct Sector {
  pub chunk_info: RwLock<[l1_chunk::ChunkInfo; 64]>,
  pub chunk: RwLock<Option<Box<[l1_chunk::Chunk; 64]>>>,
  pub chunk_halo: RwLock<
    Option<
      [Box<[l1_chunk::ChunkHalo; 64]>;
        Dir::COUNT as usize],
    >,
  >,
}
impl Sector {
  pub fn iter_chunk(
    &self,
    pos: &BlockPos,
    mut f: impl FnMut(
      InnerBlockPos,
      BlockPos,
      &l1_chunk::Chunk,
    ) -> bool,
  ) {
    let chunk = self.chunk.read();
    let Some(chunk) = chunk.as_ref() else {
      return;
    };
    for (ipos, bpos, chunk) in (0..chunk.len())
      .map(|i| {
        let i = InnerBlockPos::new(i as _);
        let p = BlockDist::from(i).level_up(2);
        (i, p)
      })
      .map(|(i, p)| (i, *pos + p))
      .map(|(i, p)| (i, p, &chunk[i.0 as usize]))
    {
      if !f(ipos, bpos, chunk) {
        break;
      }
    }
  }

  pub fn update_halo_west(
    &self,
    neigh: Option<&SectorHalo>,
  ) {
    let mut chunk = self.chunk.write();
    let Some(chunk) = chunk.as_mut() else {
      return;
    };
    let mut chunk_halo = self.chunk_halo.write();
    let chunk_halo =
      chunk_halo.get_or_insert_with(|| {
        std::array::from_fn(|_| {
          Box::new(std::array::from_fn(|_| {
            Default::default()
          }))
        })
      });
    for x in (0..3).rev() {
      for yz in (0..16).map(|yz| yz * 4) {
        chunk_halo[Dir::WST as usize][yz + x] =
          ChunkHalo::make_halo_west(&chunk[yz + x + 1])
      }
    }
    if let Some(neigh) = neigh
      .map(|n| n.chunk.as_ref())
      .flatten()
    {
      for yz in 0..16 {
        chunk_halo[Dir::WST as usize][yz * 4 + 3] =
          neigh[yz].clone();
      }
    }
  }
  pub fn update_halo_east(
    &self,
    neigh: Option<&SectorHalo>,
  ) {
    let mut chunk = self.chunk.write();
    let Some(chunk) = chunk.as_mut() else {
      return;
    };
    let mut chunk_halo = self.chunk_halo.write();
    let chunk_halo =
      chunk_halo.get_or_insert_with(|| {
        std::array::from_fn(|_| {
          Box::new(std::array::from_fn(|_| {
            Default::default()
          }))
        })
      });
    for x in 0..3 {
      for yz in (0..16).map(|yz| yz * 4) {
        chunk_halo[Dir::EST as usize][yz + x + 1] =
          ChunkHalo::make_halo_east(&chunk[yz + x])
      }
    }
    if let Some(neigh) = neigh
      .map(|n| n.chunk.as_ref())
      .flatten()
    {
      for yz in 0..16 {
        chunk_halo[Dir::EST as usize][yz * 4] =
          neigh[yz].clone();
      }
    }
  }
  pub fn update_halo_south(
    &self,
    neigh: Option<&SectorHalo>,
  ) {
    let mut chunk = self.chunk.write();
    let Some(chunk) = chunk.as_mut() else {
      return;
    };
    let mut chunk_halo = self.chunk_halo.write();
    let chunk_halo =
      chunk_halo.get_or_insert_with(|| {
        std::array::from_fn(|_| {
          Box::new(std::array::from_fn(|_| {
            Default::default()
          }))
        })
      });
    for y in (0..3).rev().map(|y| y * 4) {
      for xz in
        (0..16).map(|xz| ((xz & 12) << 2) | (xz & 3))
      {
        chunk_halo[Dir::STH as usize][xz + y] =
          ChunkHalo::make_halo_south(&chunk[xz + y + 4])
      }
    }
    if let Some(neigh) = neigh
      .map(|n| n.chunk.as_ref())
      .flatten()
    {
      for xz in 0..16 {
        chunk_halo[Dir::STH as usize]
          [(xz & 12) << 2 | (xz & 3) + 12] =
          neigh[xz].clone();
      }
    }
  }
  pub fn update_halo_north(
    &self,
    neigh: Option<&SectorHalo>,
  ) {
    let mut chunk = self.chunk.write();
    let Some(chunk) = chunk.as_mut() else {
      return;
    };
    let mut chunk_halo = self.chunk_halo.write();
    let chunk_halo =
      chunk_halo.get_or_insert_with(|| {
        std::array::from_fn(|_| {
          Box::new(std::array::from_fn(|_| {
            Default::default()
          }))
        })
      });
    for y in (0..3).map(|y| y * 4) {
      for xz in
        (0..16).map(|xz| ((xz & 12) << 2) | (xz & 3))
      {
        chunk_halo[Dir::NTH as usize][xz + y + 4] =
          ChunkHalo::make_halo_north(&chunk[xz + y])
      }
    }
    if let Some(neigh) = neigh
      .map(|n| n.chunk.as_ref())
      .flatten()
    {
      for xz in 0..16 {
        chunk_halo[Dir::NTH as usize]
          [(xz & 12) << 2 | (xz & 3)] =
          neigh[xz].clone();
      }
    }
  }
  pub fn update_halo_bottom(
    &self,
    neigh: Option<&SectorHalo>,
  ) {
    let mut chunk = self.chunk.write();
    let Some(chunk) = chunk.as_mut() else {
      return;
    };
    let mut chunk_halo = self.chunk_halo.write();
    let chunk_halo =
      chunk_halo.get_or_insert_with(|| {
        std::array::from_fn(|_| {
          Box::new(std::array::from_fn(|_| {
            Default::default()
          }))
        })
      });
    for z in (0..3).rev().map(|z| z * 16) {
      for xy in 0..16 {
        chunk_halo[Dir::BTM as usize][xy + z] =
          ChunkHalo::make_halo_bottom(
            &chunk[xy + z + 16],
          )
      }
    }
    if let Some(neigh) = neigh
      .map(|n| n.chunk.as_ref())
      .flatten()
    {
      for xy in 0..16 {
        chunk_halo[Dir::BTM as usize][xy + 48] =
          neigh[xy].clone();
      }
    }
  }
  pub fn update_halo_top(
    &self,
    neigh: Option<&SectorHalo>,
  ) {
    let mut chunk = self.chunk.write();
    let Some(chunk) = chunk.as_mut() else {
      return;
    };
    let mut chunk_halo = self.chunk_halo.write();
    let chunk_halo =
      chunk_halo.get_or_insert_with(|| {
        std::array::from_fn(|_| {
          Box::new(std::array::from_fn(|_| {
            Default::default()
          }))
        })
      });
    for z in (0..3).map(|z| z * 16) {
      for xy in 0..16 {
        chunk_halo[Dir::TOP as usize][xy + z + 16] =
          ChunkHalo::make_halo_top(&chunk[xy + z])
      }
    }
    if let Some(neigh) = neigh
      .map(|n| n.chunk.as_ref())
      .flatten()
    {
      for xy in 0..16 {
        chunk_halo[Dir::TOP as usize][xy] =
          neigh[xy].clone();
      }
    }
  }

  #[inline]
  pub fn chk_visible_face(
    ipos: InnerBlockPos,
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> [bool; Dir::COUNT as usize] {
    let stride = nalgebra::Vector3::new(
      ((ipos.0 >> 0) & 3) as f64 * 64.,
      ((ipos.0 >> 2) & 3) as f64 * 64.,
      ((ipos.0 >> 4) & 3) as f64 * 64.,
    );
    [
      Self::chk_visible_west_face(
        stride,
        relative_camera_pos,
      ),
      Self::chk_visible_east_face(
        stride,
        relative_camera_pos,
      ),
      Self::chk_visible_south_face(
        stride,
        relative_camera_pos,
      ),
      Self::chk_visible_north_face(
        stride,
        relative_camera_pos,
      ),
      Self::chk_visible_bottom_face(
        stride,
        relative_camera_pos,
      ),
      Self::chk_visible_top_face(
        stride,
        relative_camera_pos,
      ),
    ]
  }

  /// 対象のリージョンの西面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_west_face(
    stride: nalgebra::Vector3<f64>,
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin =
      nalgebra::Point3::new(64., 32., 32.) + stride;
    let vp = relative_camera_pos - origin;
    vp.x <= f64::EPSILON
  }

  /// 対象のリージョンの東面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_east_face(
    stride: nalgebra::Vector3<f64>,
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin =
      nalgebra::Point3::new(0., 32., 32.) + stride;
    let vp = relative_camera_pos - origin;
    -f64::EPSILON <= vp.x
  }

  /// 対象のリージョンの南面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_south_face(
    stride: nalgebra::Vector3<f64>,
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin =
      nalgebra::Point3::new(32., 64., 32.) + stride;
    let vp = relative_camera_pos - origin;
    vp.y <= f64::EPSILON
  }

  /// 対象のリージョンの北面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_north_face(
    stride: nalgebra::Vector3<f64>,
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin =
      nalgebra::Point3::new(32., 0., 32.) + stride;
    let vp = relative_camera_pos - origin;
    -f64::EPSILON <= vp.y
  }

  /// 対象のリージョンの下面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_bottom_face(
    stride: nalgebra::Vector3<f64>,
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin =
      nalgebra::Point3::new(32., 32., 64.) + stride;
    let vp = relative_camera_pos - origin;
    vp.z <= f64::EPSILON
  }

  /// 対象のリージョンの上面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_top_face(
    stride: nalgebra::Vector3<f64>,
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin =
      nalgebra::Point3::new(32., 32., 0.) + stride;
    let vp = relative_camera_pos - origin;
    -f64::EPSILON <= vp.z
  }
}

pub struct SectorHaloArray(pub [SectorHalo; 6]);
impl Default for SectorHaloArray {
  fn default() -> Self {
    Self(Default::default())
  }
}
impl SectorHaloArray {
  #[inline]
  pub fn make_halo_west(&mut self, neigh: &Sector) {
    self.0[Dir::WST as usize] =
      SectorHalo::make_halo_west(neigh);
  }
  #[inline]
  pub fn make_halo_east(&mut self, neigh: &Sector) {
    self.0[Dir::EST as usize] =
      SectorHalo::make_halo_east(neigh);
  }
  #[inline]
  pub fn make_halo_south(&mut self, neigh: &Sector) {
    self.0[Dir::STH as usize] =
      SectorHalo::make_halo_south(neigh);
  }
  #[inline]
  pub fn make_halo_north(&mut self, neigh: &Sector) {
    self.0[Dir::NTH as usize] =
      SectorHalo::make_halo_north(neigh);
  }
  #[inline]
  pub fn make_halo_bottom(&mut self, neigh: &Sector) {
    self.0[Dir::BTM as usize] =
      SectorHalo::make_halo_bottom(neigh);
  }
  #[inline]
  pub fn make_halo_top(&mut self, neigh: &Sector) {
    self.0[Dir::TOP as usize] =
      SectorHalo::make_halo_top(neigh);
  }
}

pub struct SectorHalo {
  pub chunk: Option<Box<[l1_chunk::ChunkHalo; 16]>>,
}
impl Default for SectorHalo {
  fn default() -> Self {
    Self { chunk: None }
  }
}
impl SectorHalo {
  #[inline]
  pub fn make_halo_west(sector: &Sector) -> Self {
    Self {
      chunk: sector
        .chunk
        .read()
        .as_ref()
        .map(|c| {
          std::array::from_fn(|i| {
            l1_chunk::ChunkHalo::make_halo_west(
              &c[i * 4],
            )
          })
        })
        .map(|c| Box::new(c)),
    }
  }
  #[inline]
  pub fn make_halo_east(sector: &Sector) -> Self {
    Self {
      chunk: sector
        .chunk
        .read()
        .as_ref()
        .map(|c| {
          std::array::from_fn(|i| {
            l1_chunk::ChunkHalo::make_halo_east(
              &c[i * 4 + 3],
            )
          })
        })
        .map(|c| Box::new(c)),
    }
  }
  #[inline]
  pub fn make_halo_south(sector: &Sector) -> Self {
    Self {
      chunk: sector
        .chunk
        .read()
        .as_ref()
        .map(|c| {
          std::array::from_fn(|i| {
            let broad = (i & !3) * 4;
            let narrow = i % 4;
            l1_chunk::ChunkHalo::make_halo_south(
              &c[broad + narrow],
            )
          })
        })
        .map(|c| Box::new(c)),
    }
  }
  #[inline]
  pub fn make_halo_north(sector: &Sector) -> Self {
    Self {
      chunk: sector
        .chunk
        .read()
        .as_ref()
        .map(|c| {
          std::array::from_fn(|i| {
            let broad = (i & !3) * 4;
            let narrow = i % 4;
            l1_chunk::ChunkHalo::make_halo_north(
              &c[broad + narrow + 12],
            )
          })
        })
        .map(|c| Box::new(c)),
    }
  }
  #[inline]
  pub fn make_halo_bottom(sector: &Sector) -> Self {
    Self {
      chunk: sector
        .chunk
        .read()
        .as_ref()
        .map(|c| {
          std::array::from_fn(|i| {
            l1_chunk::ChunkHalo::make_halo_bottom(&c[i])
          })
        })
        .map(|c| Box::new(c)),
    }
  }
  #[inline]
  pub fn make_halo_top(sector: &Sector) -> Self {
    Self {
      chunk: sector
        .chunk
        .read()
        .as_ref()
        .map(|c| {
          std::array::from_fn(|i| {
            l1_chunk::ChunkHalo::make_halo_top(
              &c[i + 48],
            )
          })
        })
        .map(|c| Box::new(c)),
    }
  }
}
