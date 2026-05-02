use super::block::*;
use super::chunk::*;

#[repr(C)]
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub struct CellPos(pub(super) [i64; 3]);
impl CellPos {
  #[inline]
  pub fn new(value: [i64; 3]) -> Self {
    value.into()
  }
  #[inline]
  pub fn get(&self) -> &[i64; 3] {
    &self.0
  }
}
impl From<ChunkPos> for CellPos {
  #[inline]
  fn from(value: ChunkPos) -> Self {
    value.0.into()
  }
}
impl From<[i64; 3]> for CellPos {
  #[inline]
  fn from(value: [i64; 3]) -> Self {
    Self([
      value[0] & !3,
      value[1] & !3,
      value[2] & !3,
    ])
  }
}
impl From<CellPos> for [i64; 3] {
  #[inline]
  fn from(value: CellPos) -> Self {
    value.0
  }
}

pub struct AreaQueryResult<
  I: Iterator<Item = ([u8; 3], u8)>,
> {
  pub iterator: I,
}
pub struct AbstractCell;
impl AbstractCell {
  pub fn area_query(
    pos_range: [[u8; 2]; 3],
  ) -> AreaQueryResult<
    impl Iterator<Item = ([u8; 3], u8)>,
  > {
    let iterator = (pos_range[0][0]
      ..=pos_range[0][1])
      .flat_map(move |x| {
        (pos_range[1][0]..=pos_range[1][1])
          .map(move |y| [x, y])
      })
      .flat_map(move |[x, y]| {
        (pos_range[2][0]..=pos_range[2][1])
          .map(move |z| [x, y, z])
      })
      .map(|p| std::array::from_fn(|i| p[i]))
      .map(|p| {
        (p, p[0] + p[1] * 4 + p[2] * 16)
      });
    AreaQueryResult { iterator }
  }
}

#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct CellLoID(pub [BlockLoID; 64]);
impl Default for CellLoID {
  fn default() -> Self {
    Self([BlockLoID::air(); 64])
  }
}
impl CellLoID {
  pub fn air_cell() -> Self {
    Default::default()
  }

  #[inline]
  pub fn is_all_air(&self) -> bool {
    !self.0.iter().any(|b| !b.is_air())
  }

  pub fn aabb_check(
    &self,
    pos_range: [[u8; 2]; 3],
  ) -> bool {
    for (_, i) in
      AbstractCell::area_query(pos_range)
        .iterator
    {
      if !self.0[i as usize].is_air() {
        return true;
      }
    }
    false
  }
}

#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct CellHiID(pub [BlockHiID; 64]);
