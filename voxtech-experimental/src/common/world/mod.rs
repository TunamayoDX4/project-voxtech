use hashbrown::HashMap;

pub mod player;

pub mod l0_cell;
pub mod l1_chunk;
pub mod l2_sector;
pub mod l3_region;

pub mod dim;

/// # World
/// 世界
pub struct World {
  dims: HashMap<String, dim::Dim>,
}
