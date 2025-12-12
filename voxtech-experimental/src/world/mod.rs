use std::collections::VecDeque;

use hashbrown::HashMap;

pub mod types;

/// Worldはプログラム上における空間インスタンスのバインダ
/// World is the binder for dimension instances in the program.
pub struct World {
  map: HashMap<types::BlockPos, Chunk>,
}
impl World {
  pub fn spawn_chunk(
    &mut self,
    chunk_pos: types::BlockPos,
    f: impl Fn(types::BlockPos) -> u8 + Clone,
  ) {
    self
      .map
      .entry(chunk_pos)
      .insert(Chunk::new(chunk_pos, f));
  }
}

pub struct Chunk {
  cell: Option<Box<[Cell; 64]>>,
}
impl Chunk {
  pub fn new(
    chunk_pos: types::BlockPos,
    f: impl Fn(types::BlockPos) -> u8 + Clone,
  ) -> Self {
    Self {
      cell: Some(Box::new(std::array::from_fn(
        |i| {
          let pos = chunk_pos.merge_inner(
            types::BlockPos::from_64index(i as u8),
          );
          Cell::new(pos, f.clone())
        },
      ))),
    }
  }
}

#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct Cell([u8; 64]);
impl Cell {
  pub fn new(
    cell_pos: types::BlockPos,
    f: impl Fn(types::BlockPos) -> u8,
  ) -> Self {
    Self(std::array::from_fn(|i| {
      let pos = cell_pos.merge_inner(
        types::BlockPos::from_64index(i as u8),
      );
      f(pos)
    }))
  }
}
