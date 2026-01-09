//! Level1 Chunk
//!
//! 16m^3 Area

use crate::common::l0_cell::L0Cell;

#[repr(C, align(8))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct L1Chunk {
  idx_key: u32,
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
  cell: Vec<L1ChunkBody>,
}
