//! Level0 Cell
//!
//! 4m^3 Area

#[repr(C, align(8))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct L0Cell {
  idx_key: u32,
  gen_key: u8,
}

#[repr(C, align(64))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct L0CellBody([u8; 64]);

pub struct L0CellStorage {
  bump: Vec<bool>,
  gen_key: Vec<u8>,
  cell: Vec<L0CellBody>,
}
