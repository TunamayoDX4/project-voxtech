use super::l0_cell;

pub struct Chunk {
  info: [l0_cell::CellInfo; 64], 
  cell: Option<Box<[l0_cell::Cell; 64]>>,
}
