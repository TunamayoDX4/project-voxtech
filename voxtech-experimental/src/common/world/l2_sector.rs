use crate::common::Dir;

use super::{l0_cell, l1_chunk};

pub struct Sector {
  pub chunk: Option<Box<[l1_chunk::Chunk; 64]>>,
  pub chunk_halo:
    Option<Box<[l1_chunk::ChunkHaloArray; 64]>>,
}
impl Sector {
  pub fn update_neigh_west(
    &mut self,
    neigh: &Sector,
  ) {
    let neigh = neigh.chunk.as_ref();
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
    for i in (0..chunk.len()).map(|i| i * 4) {
      if let Some(neigh) = neigh {
        chunk_halo[i].update_neigh_west(&neigh[i + 3]);
      }
      chunk_halo[i + 1].update_neigh_west(&chunk[i]);
      chunk_halo[i + 2].update_neigh_west(&chunk[i + 1]);
      chunk_halo[i + 3].update_neigh_west(&chunk[i + 2]);
    }
  }
  pub fn update_neigh_west(
    &mut self,
    neigh: &Sector,
  ) {
    let neigh = neigh.chunk.as_ref();
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
    for i in (0..chunk.len()).map(|i| i * 4) {
      if let Some(neigh) = neigh {
        chunk_halo[i].update_neigh_west(&neigh[i + 3]);
      }
      chunk_halo[i + 1].update_neigh_west(&chunk[i]);
      chunk_halo[i + 2].update_neigh_west(&chunk[i + 1]);
      chunk_halo[i + 3].update_neigh_west(&chunk[i + 2]);
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
