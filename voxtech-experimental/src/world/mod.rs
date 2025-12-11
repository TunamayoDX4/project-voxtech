use std::collections::VecDeque;

use hashbrown::HashMap;

/// Worldはプログラム上における空間インスタンスのバインダ
/// World is the binder for dimension instances in the program.
pub struct World {
  dim: Vec<Option<Dimension>>,
  dim_map: HashMap<String, usize>,
  dim_clr: VecDeque<usize>,
}

/// Dimensionはワールドの中のインスタンス
/// Dimension is instance within the world
pub struct Dimension {
  region: Vec<Option<Region>>,
  region_map: HashMap<[i64; 3], usize>,
  region_clr: VecDeque<usize>,
}

/// Regionは256m立方サイズのワールド断片
/// Region is 256m^3 scale world fragment
#[derive(Clone)]
pub struct Region {
  sector: Box<[Sector; 64]>,
}

/// Sectorは64m立方サイズのワールド断片
/// Sector is 64m^3 scale world fragment
#[derive(Clone)]
pub struct Sector {
  chunk: [Box<Chunk>; 64],
}
impl std::fmt::Debug for Sector {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("Sector")
      .field("chunk", &self.chunk)
      .finish()
  }
}

/// Chunkは16m立方サイズのワールド断片
/// Chunk is 16m^3 scale world fragment
#[repr(C)]
#[derive(Clone)]
pub struct Chunk {
  cell: [Cell; 64],
}
impl std::fmt::Debug for Chunk {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("Chunk")
      .field("cell", &self.cell)
      .finish()
  }
}

/// Cellは4m立方サイズのワールド断片
/// Cell is 4m^3 scale world fragment
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
  blocks: [Block; 64],
}

/// Blockはワールドにおけるもっとも小さな断片
/// Block is the smallest fragment in the world
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block(u8);
impl Default for Block {
  #[inline]
  fn default() -> Self {
    Self::air()
  }
}
impl Block {
  #[inline]
  pub const fn air() -> Self {
    Self(0)
  }
  #[inline]
  pub fn is_air(&self) -> bool {
    *self == Self::air()
  }
  #[inline]
  pub fn not_air(&self) -> bool {
    *self != Self::air()
  }
}
