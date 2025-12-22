use super::l0_cell::Cell;

pub struct Chunk {
  cell: Box<[Cell; 64]>,
}
