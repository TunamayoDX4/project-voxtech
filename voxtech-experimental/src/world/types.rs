use std::ops::{Add, Div, Mul, Rem, Sub};

#[repr(C, align(32))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos([i64; 3]);
impl BlockPos {
  #[inline]
  pub fn new(x: i64, y: i64, z: i64) -> Self {
    Self([x, y, z])
  }
  #[inline]
  pub fn as_64index(&self) -> u8 {
    (self.get_x() << 0) as u8
      | (self.get_y() << 2) as u8
      | (self.get_z() << 4) as u8
  }
  #[inline]
  pub fn from_64index(pos: u8) -> Self {
    Self::new(
      (pos >> 0 & 0b11) as i64,
      (pos >> 2 & 0b11) as i64,
      (pos >> 4 & 0b11) as i64,
    )
  }
  #[inline]
  pub fn get_x(&self) -> i64 {
    self.0[0]
  }
  #[inline]
  pub fn get_y(&self) -> i64 {
    self.0[1]
  }
  #[inline]
  pub fn get_z(&self) -> i64 {
    self.0[2]
  }
  #[inline]
  pub fn set_x(&mut self, x: i64) {
    self.0[0] = x
  }
  #[inline]
  pub fn set_y(&mut self, y: i64) {
    self.0[1] = y
  }
  #[inline]
  pub fn set_z(&mut self, z: i64) {
    self.0[2] = z
  }
  #[inline]
  pub fn split_inner(&self) -> (BlockPos, BlockPos) {
    let to = Self::new(
      self.0[0] >> 2,
      self.0[1] >> 2,
      self.0[2] >> 2,
    );
    let inner = Self::new(
      self.0[0].abs() & 0b11,
      self.0[1].abs() & 0b11,
      self.0[2].abs() & 0b11,
    );
    (to, inner)
  }
  #[inline]
  pub fn merge_inner(
    &self,
    inner: BlockPos,
  ) -> BlockPos {
    Self::new(
      self.0[0] << 2 & inner.get_x(),
      self.0[1] << 2 & inner.get_y(),
      self.0[2] << 2 & inner.get_z(),
    )
  }
  #[inline]
  pub fn up_level(&self, shift: u8) -> Self {
    Self::new(
      self.0[0] << 2 * shift,
      self.0[1] << 2 * shift,
      self.0[2] << 2 * shift,
    )
  }
  #[inline]
  pub fn down_level(&self, shift: u8) -> Self {
    Self::new(
      self.0[0] >> 2 * shift,
      self.0[1] >> 2 * shift,
      self.0[2] >> 2 * shift,
    )
  }
}
impl Sub<BlockPos> for BlockPos {
  type Output = BlockDist;

  fn sub(self, rhs: BlockPos) -> Self::Output {
    BlockDist([
      self.0[0] - rhs.0[0],
      self.0[1] - rhs.0[1],
      self.0[2] - rhs.0[2],
    ])
  }
}
impl Add<BlockDist> for BlockPos {
  type Output = BlockPos;

  fn add(self, rhs: BlockDist) -> Self::Output {
    BlockPos([
      self.0[0] + rhs.0[0],
      self.0[1] + rhs.0[1],
      self.0[2] + rhs.0[2],
    ])
  }
}

#[repr(C, align(32))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockDist([i64; 3]);
impl BlockDist {
  #[inline]
  pub fn new(x: i64, y: i64, z: i64) -> Self {
    Self([x, y, z])
  }
  #[inline]
  pub fn get_x(&self) -> i64 {
    self.0[0]
  }
  #[inline]
  pub fn get_y(&self) -> i64 {
    self.0[1]
  }
  #[inline]
  pub fn get_z(&self) -> i64 {
    self.0[2]
  }
  #[inline]
  pub fn set_x(&mut self, x: i64) {
    self.0[0] = x
  }
  #[inline]
  pub fn set_y(&mut self, y: i64) {
    self.0[1] = y
  }
  #[inline]
  pub fn set_z(&mut self, z: i64) {
    self.0[2] = z
  }
  #[inline]
  pub fn up_level(&self, shift: u8) -> Self {
    Self::new(
      self.0[0] << 2 * shift,
      self.0[1] << 2 * shift,
      self.0[2] << 2 * shift,
    )
  }
  #[inline]
  pub fn down_level(&self, shift: u8) -> Self {
    Self::new(
      self.0[0] >> 2 * shift,
      self.0[1] >> 2 * shift,
      self.0[2] >> 2 * shift,
    )
  }
}
impl Add<BlockDist> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn add(self, rhs: BlockDist) -> Self::Output {
    BlockDist([
      self.0[0] + rhs.0[0],
      self.0[1] + rhs.0[1],
      self.0[2] + rhs.0[2],
    ])
  }
}
impl Sub<BlockDist> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn sub(self, rhs: BlockDist) -> Self::Output {
    BlockDist([
      self.0[0] - rhs.0[0],
      self.0[1] - rhs.0[1],
      self.0[2] - rhs.0[2],
    ])
  }
}
impl Mul<BlockDist> for BlockDist {
  type Output = BlockDist;

  fn mul(self, rhs: BlockDist) -> Self::Output {
    BlockDist([
      self.0[0] * rhs.0[0],
      self.0[1] * rhs.0[1],
      self.0[2] * rhs.0[2],
    ])
  }
}
impl Div<BlockDist> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn div(self, rhs: BlockDist) -> Self::Output {
    BlockDist([
      self.0[0] / rhs.0[0],
      self.0[1] / rhs.0[1],
      self.0[2] / rhs.0[2],
    ])
  }
}
impl Rem<BlockDist> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn rem(self, rhs: BlockDist) -> Self::Output {
    BlockDist([
      self.0[0] % rhs.0[0],
      self.0[1] % rhs.0[1],
      self.0[2] % rhs.0[2],
    ])
  }
}
impl Mul<i64> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn mul(self, rhs: i64) -> Self::Output {
    BlockDist([
      self.0[0] * rhs,
      self.0[1] * rhs,
      self.0[2] * rhs,
    ])
  }
}
impl Div<i64> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn div(self, rhs: i64) -> Self::Output {
    BlockDist([
      self.0[0] / rhs,
      self.0[1] / rhs,
      self.0[2] / rhs,
    ])
  }
}
impl Rem<i64> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn rem(self, rhs: i64) -> Self::Output {
    BlockDist([
      self.0[0] % rhs,
      self.0[1] % rhs,
      self.0[2] % rhs,
    ])
  }
}
