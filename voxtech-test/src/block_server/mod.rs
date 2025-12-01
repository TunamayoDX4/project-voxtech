use std::{
  collections::HashMap,
  ops::{Index, IndexMut},
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CellBlkIDLo([u8; 64]);
impl Default for CellBlkIDLo {
  fn default() -> Self {
    Self([0; 64])
  }
}
impl Index<crate::types::BlockPos> for CellBlkIDLo {
  type Output = u8;

  fn index(
    &self,
    index: crate::types::BlockPos,
  ) -> &Self::Output {
    &self.0[index.as_64tree_index() as usize]
  }
}
impl IndexMut<crate::types::BlockPos> for CellBlkIDLo {
  fn index_mut(
    &mut self,
    index: crate::types::BlockPos,
  ) -> &mut Self::Output {
    &mut self.0[index.as_64tree_index() as usize]
  }
}
impl CellBlkIDLo {
  /// 占有率を集計する
  #[inline]
  pub fn occupancy(&self) -> u64 {
    self
      .0
      .iter()
      .map(|i| {
        if *i != 0 {
          1
        } else {
          0
        }
      })
      .enumerate()
      .fold(0, |v, (i, b)| v | (b << i))
  }

  /// ブロックの個数を計算する
  #[inline]
  pub fn sum_block(&self) -> usize {
    self
      .0
      .iter()
      .map(|b| {
        if *b != 0 {
          1
        } else {
          0
        }
      })
      .sum()
  }

  #[inline]
  pub fn iter(&self) -> impl Iterator<Item = &u8> {
    self.0.iter()
  }

  #[inline]
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut u8> {
    self.0.iter_mut()
  }

  #[inline]
  pub fn into_iter(self) -> impl Iterator<Item = u8> {
    self.0.into_iter()
  }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CellInfo {
  occupancy: u64,
}
impl Default for CellInfo {
  fn default() -> Self {
    Self { occupancy: 0u64 }
  }
}
impl CellInfo {
  /// セルのブロック数を計算する
  pub fn calc_blocks(&self) -> u8 {
    self.occupancy.count_ones() as u8
  }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ChunkBlkIDLo([CellBlkIDLo; 64]);
impl Default for ChunkBlkIDLo {
  fn default() -> Self {
    Self([Default::default(); 64])
  }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ChunkInfo {
  cell_info: [CellInfo; 64],
}
impl Default for ChunkInfo {
  fn default() -> Self {
    Self {
      cell_info: [Default::default(); 64],
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Chunk {
  info: ChunkInfo,
  blk_loid: Option<Box<ChunkBlkIDLo>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SectorPos([i64; 4]);

#[derive(Debug, Clone)]
pub struct Sector {
  chunks: [Chunk; 64],
}
impl Default for Sector {
  fn default() -> Self {
    Self {
      chunks: std::array::from_fn(|_| Default::default()),
    }
  }
}

pub struct World {
  sectors: HashMap<SectorPos, Sector>,
}
