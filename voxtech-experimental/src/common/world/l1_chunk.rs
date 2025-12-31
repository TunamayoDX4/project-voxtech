use crate::common::Dir;

use super::l0_cell;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ChunkInfo {
  /// 不透明タイルが更新されているか？
  pub dirty_opq_tile: bool,
}
impl Default for ChunkInfo {
  fn default() -> Self {
    Self {
      dirty_opq_tile: false,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Chunk {
  pub cell: Option<Box<[l0_cell::Cell; 64]>>,
}
impl Chunk {
  /*
  /// 1ブロック西(-X)にずらす
  #[inline]
  pub fn stride_west(&mut self) -> bool {
    let Some(cell) = self.cell.as_mut() else {
      return false;
    };
    for i in 0..64u8 {
      cell[i as usize].stride_west();
    }
    true
  }
  /// 1ブロック東(+X)にずらす
  #[inline]
  pub fn stride_east(&mut self) -> bool {
    let Some(cell) = self.cell.as_mut() else {
      return false;
    };
    for i in 0..64u8 {
      cell[i as usize].stride_east();
    }
    true
  }
  /// 1ブロック南(-Y)にずらす
  #[inline]
  pub fn stride_south(&mut self) -> bool {
    let Some(cell) = self.cell.as_mut() else {
      return false;
    };
    for i in 0..64u8 {
      cell[i as usize].stride_south();
    }
    true
  }
  /// 1ブロック北(+Y)にずらす
  #[inline]
  pub fn stride_north(&mut self) -> bool {
    let Some(cell) = self.cell.as_mut() else {
      return false;
    };
    for i in 0..64u8 {
      cell[i as usize].stride_north();
    }
    true
  }
  /// 1ブロック下(-Z)にずらす
  #[inline]
  pub fn stride_bottom(&mut self) -> bool {
    let Some(cell) = self.cell.as_mut() else {
      return false;
    };
    for i in 0..64u8 {
      cell[i as usize].stride_bottom();
    }
    true
  }
  /// 1ブロック上(+Z)にずらす
  #[inline]
  pub fn stride_top(&mut self) -> bool {
    let Some(cell) = self.cell.as_mut() else {
      return false;
    };
    for i in 0..64u8 {
      cell[i as usize].stride_top();
    }
    true
  }
   */
}

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
