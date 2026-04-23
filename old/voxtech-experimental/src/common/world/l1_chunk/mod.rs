//! Level1 Chunk
//!
//! 16m^3 Area

use crate::common::l0_cell::L0Cell;

#[repr(C, align(4))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct L1Chunk {
  idx_key: u16,
  gen_key: u8,
}

#[repr(C, align(64))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct L1ChunkBody {
  cells: [L0Cell; 64],
}

pub struct L1ChunkStorage {
  bump: Vec<bool>,
  gen_key: Vec<u8>,
  chunk: Vec<L1ChunkBody>,

  /// 直近に削除されたチャンクが再確保される可能性もあるためLIFOとする
  remove_stack: Vec<u32>,
}
