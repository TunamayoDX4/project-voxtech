use crate::common::{
  BlockDist, BlockPos, Dir, InnerBlockPos,
};

use super::l1_chunk;

pub struct Sector {
  pub chunk: Option<Box<[l1_chunk::Chunk; 64]>>,
  pub chunk_halo:
    Option<Box<[l1_chunk::ChunkHaloArray; 64]>>,
}
impl Sector {
  pub fn iter_chunk(
    &self,
    pos: &BlockPos,
  ) -> Option<
    impl Iterator<
      Item = (
        InnerBlockPos,
        BlockPos,
        &l1_chunk::Chunk,
      ),
    >,
  > {
    let Some(chunk) = self.chunk.as_ref() else {
      return None;
    };
    let ret = (0..chunk.len())
      .map(|i| {
        let i = InnerBlockPos::new(i as _);
        let p = BlockDist::from(i).level_up(2);
        (i, p)
      })
      .map(|(i, p)| (i, *pos + p))
      .map(|(i, p)| (i, p, &chunk[i.0 as usize]));
    Some(ret)
  }

  pub fn update_neigh_west(&mut self, neigh: &Sector) {
    let Some(chunk) = self.chunk.as_mut() else {
      return;
    };
    let chunk_halo = self
      .chunk_halo
      .get_or_insert_with(|| {
        Box::new(std::array::from_fn(|_| {
          Default::default()
        }))
      });
    for i in (0..3).rev() {
      for j in (0..16).map(|j| j * 4) {
        chunk_halo[j + i + 1]
          .update_neigh_west(&chunk[j + i]);
      }
    }
    if let Some(neigh) = neigh.chunk.as_ref() {
      for i in (0..16).map(|i| i * 4) {
        chunk_halo[i].update_neigh_west(&neigh[i + 3]);
      }
    }
  }
  pub fn update_neigh_east(&mut self, neigh: &Sector) {
    let Some(chunk) = self.chunk.as_mut() else {
      return;
    };
    let chunk_halo = self
      .chunk_halo
      .get_or_insert_with(|| {
        Box::new(std::array::from_fn(|_| {
          Default::default()
        }))
      });
    for i in 0..3 {
      for j in (0..16).map(|j| j * 4) {
        chunk_halo[j + i]
          .update_neigh_east(&chunk[j + i + 1]);
      }
    }
    if let Some(neigh) = neigh.chunk.as_ref() {
      for i in (0..16).map(|i| i * 4) {
        chunk_halo[i + 3].update_neigh_east(&neigh[i]);
      }
    }
  }
  pub fn update_neigh_south(&mut self, neigh: &Sector) {
    let Some(chunk) = self.chunk.as_mut() else {
      return;
    };
    let chunk_halo = self
      .chunk_halo
      .get_or_insert_with(|| {
        Box::new(std::array::from_fn(|_| {
          Default::default()
        }))
      });
    for i in (0..3).rev().map(|i| i * 4) {
      for j in
        (0..16).map(|j| ((j & 12) << 2) | (j & 3))
      {
        chunk_halo[j + i + 4]
          .update_neigh_west(&chunk[j + i]);
      }
    }
    if let Some(neigh) = neigh.chunk.as_ref() {
      for i in
        (0..16).map(|j| ((j & 12) << 2) | (j & 3))
      {
        chunk_halo[i]
          .update_neigh_west(&neigh[i + 3 * 4]);
      }
    }
  }
  pub fn update_neigh_north(&mut self, neigh: &Sector) {
    let Some(chunk) = self.chunk.as_mut() else {
      return;
    };
    let chunk_halo = self
      .chunk_halo
      .get_or_insert_with(|| {
        Box::new(std::array::from_fn(|_| {
          Default::default()
        }))
      });
    for i in (0..3).map(|i| i * 4) {
      for j in
        (0..16).map(|j| ((j & 12) << 2) | (j & 3))
      {
        chunk_halo[j + i]
          .update_neigh_east(&chunk[j + i + 4]);
      }
    }
    if let Some(neigh) = neigh.chunk.as_ref() {
      for i in
        (0..16).map(|j| ((j & 12) << 2) | (j & 3))
      {
        chunk_halo[i + 3 * 4]
          .update_neigh_east(&neigh[i]);
      }
    }
  }
  pub fn update_neigh_bottom(
    &mut self,
    neigh: &Sector,
  ) {
    let Some(chunk) = self.chunk.as_mut() else {
      return;
    };
    let chunk_halo = self
      .chunk_halo
      .get_or_insert_with(|| {
        Box::new(std::array::from_fn(|_| {
          Default::default()
        }))
      });
    for i in (0..3).rev().map(|i| i * 16) {
      for j in 0..16 {
        chunk_halo[j + i + 16]
          .update_neigh_bottom(&chunk[j + i]);
      }
    }
    if let Some(neigh) = neigh.chunk.as_ref() {
      for i in 0..16 {
        chunk_halo[i]
          .update_neigh_bottom(&neigh[i + 3 * 16]);
      }
    }
  }
  pub fn update_neigh_top(&mut self, neigh: &Sector) {
    let Some(chunk) = self.chunk.as_mut() else {
      return;
    };
    let chunk_halo = self
      .chunk_halo
      .get_or_insert_with(|| {
        Box::new(std::array::from_fn(|_| {
          Default::default()
        }))
      });
    for i in (0..3).map(|i| i * 16) {
      for j in 0..16 {
        chunk_halo[j + i]
          .update_neigh_top(&chunk[j + i + 16]);
      }
    }
    if let Some(neigh) = neigh.chunk.as_ref() {
      for i in 0..16 {
        chunk_halo[i + 3 * 16]
          .update_neigh_top(&neigh[i]);
      }
    }
  }
}

pub struct SectorHaloArray([SectorHalo; 6]);
impl Default for SectorHaloArray {
  fn default() -> Self {
    Self(Default::default())
  }
}
impl SectorHaloArray {
  #[inline]
  pub fn update_neigh_west(&mut self, neigh: &Sector) {
    self.0[Dir::WST as usize] =
      SectorHalo::make_halo_east(neigh);
  }
  #[inline]
  pub fn update_neigh_east(&mut self, neigh: &Sector) {
    self.0[Dir::EST as usize] =
      SectorHalo::make_halo_west(neigh);
  }
  #[inline]
  pub fn update_neigh_south(&mut self, neigh: &Sector) {
    self.0[Dir::STH as usize] =
      SectorHalo::make_halo_north(neigh);
  }
  #[inline]
  pub fn update_neigh_north(&mut self, neigh: &Sector) {
    self.0[Dir::NTH as usize] =
      SectorHalo::make_halo_south(neigh);
  }
  #[inline]
  pub fn update_neigh_bottom(
    &mut self,
    neigh: &Sector,
  ) {
    self.0[Dir::BTM as usize] =
      SectorHalo::make_halo_top(neigh);
  }
  #[inline]
  pub fn update_neigh_top(&mut self, neigh: &Sector) {
    self.0[Dir::TOP as usize] =
      SectorHalo::make_halo_bottom(neigh);
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
