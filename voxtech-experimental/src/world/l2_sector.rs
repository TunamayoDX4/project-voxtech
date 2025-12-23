use super::{l0_cell, l1_chunk};

pub struct Sector {
	chunks: Box<[l1_chunk::Chunk; 64]>, 
}
