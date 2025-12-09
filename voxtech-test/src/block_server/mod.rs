use std::{
  collections::{HashMap, VecDeque},
  ops::{Index, IndexMut},
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CellBlkIDLo([u8; 64]);
impl Default for CellBlkIDLo {
  fn default() -> Self {
    Self([0; 64])
  }
}
impl Index<crate::types::BlockPos> for CellBlkIDLo {
  type Output = u8;

  fn index(
    &self,
    index: crate::types::BlockPos,
  ) -> &Self::Output {
    &self.0[index.as_64tree_index() as usize]
  }
}
impl IndexMut<crate::types::BlockPos> for CellBlkIDLo {
  fn index_mut(
    &mut self,
    index: crate::types::BlockPos,
  ) -> &mut Self::Output {
    &mut self.0[index.as_64tree_index() as usize]
  }
}
impl CellBlkIDLo {
  /// 占有率を集計する
  #[inline]
  pub fn occupancy(&self) -> u64 {
    self
      .0
      .iter()
      .map(|i| {
        if *i != 0 {
          1
        } else {
          0
        }
      })
      .enumerate()
      .fold(0, |v, (i, b)| v | (b << i))
  }

  /// ブロックの個数を計算する
  #[inline]
  pub fn sum_block(&self) -> usize {
    self
      .0
      .iter()
      .map(|b| {
        if *b != 0 {
          1
        } else {
          0
        }
      })
      .sum()
  }

  #[inline]
  pub fn iter(&self) -> impl Iterator<Item = &u8> {
    self.0.iter()
  }

  #[inline]
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut u8> {
    self.0.iter_mut()
  }

  #[inline]
  pub fn into_iter(self) -> impl Iterator<Item = u8> {
    self.0.into_iter()
  }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CellInfo {
  occupancy: u64,
}
impl Default for CellInfo {
  fn default() -> Self {
    Self { occupancy: 0u64 }
  }
}
impl CellInfo {
  /// セルのブロック数を計算する
  pub fn calc_blocks(&self) -> u8 {
    self.occupancy.count_ones() as u8
  }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ChunkBlkIDLo([CellBlkIDLo; 64]);
impl Default for ChunkBlkIDLo {
  fn default() -> Self {
    Self([Default::default(); 64])
  }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ChunkInfo {
  cell_info: [CellInfo; 64],
}
impl Default for ChunkInfo {
  fn default() -> Self {
    Self {
      cell_info: [Default::default(); 64],
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Chunk {
  info: ChunkInfo,
  blk_loid: Option<Box<ChunkBlkIDLo>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SectorPos([i64; 4]);

#[derive(Debug, Clone)]
pub struct Sector {
  chunks: [Chunk; 64],
}
impl Default for Sector {
  fn default() -> Self {
    Self {
      chunks: std::array::from_fn(|_| Default::default()),
    }
  }
}
impl Index<usize> for Sector {
  type Output = Chunk;

  fn index(&self, index: usize) -> &Self::Output {
    &self.chunks[index]
  }
}
impl IndexMut<usize> for Sector {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.chunks[index]
  }
}
impl Sector {
  pub fn get(&self, index: u8) -> &Chunk {
    &self.chunks[(index % 64) as usize]
  }
  pub fn get_mut(&mut self, index: u8) -> &mut Chunk {
    &mut self.chunks[(index % 64) as usize]
  }
  pub fn iter(
    &self,
  ) -> impl DoubleEndedIterator<Item = &Chunk> {
    self.chunks.iter()
  }
  pub fn iter_mut(
    &mut self,
  ) -> impl DoubleEndedIterator<Item = &mut Chunk> {
    self.chunks.iter_mut()
  }
}

pub struct World {
  sector_map: HashMap<SectorPos, usize>,
  sector_mem: Vec<Option<Sector>>,
  sector_rev: Vec<SectorPos>,
  remove_que: VecDeque<usize>,
}
impl World {
  pub fn insert(&mut self, pos: SectorPos, sector: Sector) {
    let idx = if let Some(idx) = self.remove_que.pop_front() {
      self.sector_mem[idx] = Some(sector);
      self.sector_rev[idx] = pos;
      idx
    } else {
      let idx = self.sector_mem.len();
      self
        .sector_mem
        .push(Some(sector));
      self.sector_rev.push(pos);
      idx
    };
    self.sector_map.insert(pos, idx);
  }
  pub fn remove(&mut self, pos: SectorPos) -> Option<Sector> {
    let idx = self.sector_map.remove(&pos)?;
    self.sector_mem[idx].take()
  }
  pub fn remove_idx(
    &mut self,
    idx: usize,
  ) -> Option<(SectorPos, Sector)> {
    let pos = self.sector_rev[idx];
    self.sector_map.remove(&pos)?;
    let sector = self.sector_mem[idx].take();
    sector.map(|s| (pos, s))
  }
  pub fn get(&self, idx: usize) -> Option<&Sector> {
    self.sector_mem[idx].as_ref()
  }
  pub fn get_mut(&mut self, idx: usize) -> Option<&mut Sector> {
    self.sector_mem[idx].as_mut()
  }
  pub fn get_by_key(&self, key: &SectorPos) -> Option<&Sector> {
    let idx = self
      .sector_map
      .get(key)
      .copied()?;
    self.sector_mem[idx].as_ref()
  }
  pub fn get_by_key_mut(
    &mut self,
    key: &SectorPos,
  ) -> Option<&mut Sector> {
    let idx = self
      .sector_map
      .get(key)
      .copied()?;
    self.sector_mem[idx].as_mut()
  }
  pub fn idx_get(&self, key: &SectorPos) -> Option<usize> {
    self
      .sector_map
      .get(key)
      .copied()
  }
  pub fn iter(
    &self,
  ) -> impl DoubleEndedIterator<Item = &Sector> {
    self
      .sector_mem
      .iter()
      .filter_map(|f| f.as_ref())
  }
  pub fn iter_mut(
    &mut self,
  ) -> impl DoubleEndedIterator<Item = &mut Sector> {
    self
      .sector_mem
      .iter_mut()
      .filter_map(|f| f.as_mut())
  }
}
impl Index<usize> for World {
  type Output = Sector;

  fn index(&self, index: usize) -> &Self::Output {
    self.sector_mem[index]
      .as_ref()
      .unwrap()
  }
}
impl IndexMut<usize> for World {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    self.sector_mem[index]
      .as_mut()
      .unwrap()
  }
}
