use super::{
  l0_cell,   //
  l1_chunk,  //
  l2_sector, //
};

pub mod cell_storage;

pub struct Region {
  sectors: Box<[l2_sector::Sector; 64]>, 
}
