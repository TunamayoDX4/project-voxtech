use std::{collections::VecDeque, u16};

use crate::common::cell::{CellMem, CellMemType};

#[repr(C, align(8))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellDataId {
  /// 索引データ
  idx: u32,

  /// 世代データ
  generation: u8,
}

/// Region中のCellデータを格納して管理するためのストレージ
/// Regionは256^3サイズであり、Cellの数は最大でも64^3(256Ki Cells)である。
pub struct CellStorageInRegion {
  generation: Vec<u8>,
  cell_memtype: Vec<CellMemType>,
  cell_mem: Vec<CellMem>,
  remove_queue: VecDeque<u32>,
}
impl CellStorageInRegion {
  pub fn new() -> Self {
    Self {
      generation: Vec::new(),
      cell_memtype: Vec::new(),
      cell_mem: Vec::new(),
      remove_queue: VecDeque::new(),
    }
  }
  pub fn allocate(
    &mut self,
    ty: CellMemType,
    f: impl FnOnce() -> CellMem,
  ) -> CellDataId {
  }
}
