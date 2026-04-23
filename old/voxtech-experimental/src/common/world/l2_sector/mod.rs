//! Level2 Sector
//!
//! 64m^3 Area

use crate::common::l1_chunk::L1Chunk;

#[repr(C, align(64))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct L2SectorChunks([L1Chunk; 64]);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct L2SectorInfo {
  blocks: u32,
}

#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct L2Sector {
  info: L2SectorInfo,
  sector_chunks: L2SectorChunks,
}
