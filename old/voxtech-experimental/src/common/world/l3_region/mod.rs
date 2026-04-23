//! Level3 Region
//!
//! 256m^3 Area
//!
//! データ構造におけるグローバルとローカルの責任分界点

use crate::common::{
  l0_cell::L0CellStorage, l1_chunk::L1ChunkStorage,
  l2_sector::L2Sector,
};

pub struct L3Region {
  sector: Vec<L2Sector>,
  chunk: L1ChunkStorage,
  cell: L0CellStorage,
}
