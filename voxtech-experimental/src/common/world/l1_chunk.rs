use crate::common::Dir;

use super::l0_cell;

#[derive(Debug, Clone)]
pub struct Chunk {
  pub cell: Option<Box<[l0_cell::Cell; 64]>>,
}
impl Chunk {}

pub struct ChunkHaloArray([ChunkHalo; 6]);
impl Default for ChunkHaloArray {
  fn default() -> Self {
    Self(Default::default())
  }
}
impl ChunkHaloArray {
  #[inline]
  pub fn update_neigh_west(&mut self, neigh: &Chunk) {
    self.0[Dir::WST as usize] =
      ChunkHalo::make_halo_east(neigh);
  }
  #[inline]
  pub fn update_neigh_east(&mut self, neigh: &Chunk) {
    self.0[Dir::EST as usize] =
      ChunkHalo::make_halo_west(neigh);
  }
  #[inline]
  pub fn update_neigh_south(&mut self, neigh: &Chunk) {
    self.0[Dir::STH as usize] =
      ChunkHalo::make_halo_north(neigh);
  }
  #[inline]
  pub fn update_neigh_north(&mut self, neigh: &Chunk) {
    self.0[Dir::NTH as usize] =
      ChunkHalo::make_halo_south(neigh);
  }
  #[inline]
  pub fn update_neigh_bottom(&mut self, neigh: &Chunk) {
    self.0[Dir::BTM as usize] =
      ChunkHalo::make_halo_top(neigh);
  }
  #[inline]
  pub fn update_neigh_top(&mut self, neigh: &Chunk) {
    self.0[Dir::TOP as usize] =
      ChunkHalo::make_halo_bottom(neigh);
  }
}

#[derive(Debug, Clone)]
pub struct ChunkHalo {
  pub cell: Option<[l0_cell::CellHalo; 16]>,
}
impl Default for ChunkHalo {
  fn default() -> Self {
    Self { cell: None }
  }
}
impl ChunkHalo {
  #[inline]
  pub fn make_halo_west(chunk: &Chunk) -> Self {
    Self {
      cell: chunk.cell.as_ref().map(|c| {
        std::array::from_fn(|i| {
          l0_cell::CellHalo::make_halo_west(&c[i * 4])
        })
      }),
    }
  }
  #[inline]
  pub fn make_halo_east(chunk: &Chunk) -> Self {
    Self {
      cell: chunk.cell.as_ref().map(|c| {
        std::array::from_fn(|i| {
          l0_cell::CellHalo::make_halo_east(
            &c[i * 4 + 3],
          )
        })
      }),
    }
  }
  #[inline]
  pub fn make_halo_south(chunk: &Chunk) -> Self {
    Self {
      cell: chunk.cell.as_ref().map(|c| {
        std::array::from_fn(|i| {
          let broad = (i & !3) * 4;
          let narrow = i % 4;
          l0_cell::CellHalo::make_halo_south(
            &c[broad + narrow],
          )
        })
      }),
    }
  }
  #[inline]
  pub fn make_halo_north(chunk: &Chunk) -> Self {
    Self {
      cell: chunk.cell.as_ref().map(|c| {
        std::array::from_fn(|i| {
          let broad = (i & !3) * 4;
          let narrow = i % 4;
          l0_cell::CellHalo::make_halo_north(
            &c[broad + narrow + 12],
          )
        })
      }),
    }
  }
  #[inline]
  pub fn make_halo_bottom(chunk: &Chunk) -> Self {
    Self {
      cell: chunk.cell.as_ref().map(|c| {
        std::array::from_fn(|i| {
          l0_cell::CellHalo::make_halo_bottom(&c[i])
        })
      }),
    }
  }
  #[inline]
  pub fn make_halo_top(chunk: &Chunk) -> Self {
    Self {
      cell: chunk.cell.as_ref().map(|c| {
        std::array::from_fn(|i| {
          l0_cell::CellHalo::make_halo_top(&c[i + 48])
        })
      }),
    }
  }
}
