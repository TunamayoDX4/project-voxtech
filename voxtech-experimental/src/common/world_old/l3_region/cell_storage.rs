use std::collections::VecDeque;

use super::l0_cell;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellKey(u32);
impl CellKey {
  #[inline]
  pub fn new(idx: u32, generation: u8) -> Self {
    let generation = (generation as u32) << 24;
    let idx = idx & 0x00FFFFFF;
    Self(generation | idx)
  }
  #[inline]
  pub fn idx(&self) -> u32 {
    self.0 & 0x00FFFFFF
  }
  #[inline]
  pub fn generation(&self) -> u8 {
    (self.0 >> 24) as u8
  }
}
impl std::fmt::Debug for CellKey {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.write_fmt(format_args!(
      "CellKey {{ idx: {idx}, generation: {generation} }}", 
      idx = self.idx(), 
      generation = self.generation()
    ))
  }
}

/// Cellを最大で256Ki個格納するためのストレージ。
pub struct CellStorage {
  generation: Vec<u8>,
  memory: Vec<l0_cell::Cell>,
  remque: VecDeque<u32>, 
}
