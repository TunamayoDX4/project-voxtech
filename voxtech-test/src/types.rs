use std::ops::{
  Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub,
  SubAssign,
};

/// ブロックの座標を示す構造体
#[repr(C, align(32))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos([i64; 4]);
impl BlockPos {
  #[inline]
  pub fn new(xyz: [i64; 3]) -> Self {
    Self([xyz[0], xyz[1], xyz[2], 0])
  }
  #[inline]
  pub fn x(&self) -> i64 {
    self.0[0]
  }
  #[inline]
  pub fn y(&self) -> i64 {
    self.0[1]
  }
  #[inline]
  pub fn z(&self) -> i64 {
    self.0[2]
  }
  #[inline]
  pub fn x_ref(&self) -> &i64 {
    &self.0[0]
  }
  #[inline]
  pub fn y_ref(&self) -> &i64 {
    &self.0[1]
  }
  #[inline]
  pub fn z_ref(&self) -> &i64 {
    &self.0[2]
  }
  #[inline]
  pub fn x_mut(&mut self) -> &mut i64 {
    &mut self.0[0]
  }
  #[inline]
  pub fn y_mut(&mut self) -> &mut i64 {
    &mut self.0[1]
  }
  #[inline]
  pub fn z_mut(&mut self) -> &mut i64 {
    &mut self.0[2]
  }
  #[inline]
  pub fn xyz(&self) -> [i64; 3] {
    self.0.as_chunks().0[0]
  }
  #[inline]
  pub fn xyz_ref(&self) -> &[i64; 3] {
    &self.0.as_chunks().0[0]
  }
  #[inline]
  pub fn xyz_mut(&mut self) -> &mut [i64; 3] {
    &mut self.0.as_chunks_mut().0[0]
  }
  #[inline]
  pub fn as_64tree_index(&self) -> u8 {
    (self.0[0] & 0b11) as u8
      | ((self.0[1] & 0b11) as u8) << 2
      | ((self.0[2] & 0b11) as u8) << 4
      | ((self.0[3] & 0b11) as u8) << 6
  }
  #[inline]
  pub fn down_level(&self) -> Self {
    Self([
      self.0[0] >> 2,
      self.0[1] >> 2,
      self.0[2] >> 2,
      self.0[3] >> 2,
    ])
  }
  #[inline]
  pub fn up_level(&self) -> Self {
    Self([
      self.0[0] << 2,
      self.0[1] << 2,
      self.0[2] << 2,
      self.0[3] << 2,
    ])
  }
}
impl From<[i64; 3]> for BlockPos {
  #[inline]
  fn from(value: [i64; 3]) -> Self {
    Self::new(value)
  }
}
impl Sub<Self> for BlockPos {
  type Output = BlockDist;

  #[inline]
  fn sub(self, rhs: Self) -> Self::Output {
    BlockDist([
      self.0[0] - rhs.0[0],
      self.0[1] - rhs.0[1],
      self.0[2] - rhs.0[2],
      self.0[3] - rhs.0[3],
    ])
  }
}
impl Add<BlockDist> for BlockPos {
  type Output = Self;

  #[inline]
  fn add(self, rhs: BlockDist) -> Self::Output {
    Self([
      self.0[0] + rhs.0[0],
      self.0[1] + rhs.0[1],
      self.0[2] + rhs.0[2],
      self.0[3] + rhs.0[3],
    ])
  }
}
impl AddAssign<BlockDist> for BlockPos {
  #[inline]
  fn add_assign(&mut self, rhs: BlockDist) {
    *self = *self + rhs
  }
}

/// ブロック間の距離を示す構造体
#[repr(C, align(32))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockDist([i64; 4]);
impl BlockDist {
  #[inline]
  pub fn new(xyz: [i64; 3]) -> Self {
    Self([xyz[0], xyz[1], xyz[2], 0])
  }
  #[inline]
  pub fn x(&self) -> i64 {
    self.0[0]
  }
  #[inline]
  pub fn y(&self) -> i64 {
    self.0[1]
  }
  #[inline]
  pub fn z(&self) -> i64 {
    self.0[2]
  }
  #[inline]
  pub fn x_ref(&self) -> &i64 {
    &self.0[0]
  }
  #[inline]
  pub fn y_ref(&self) -> &i64 {
    &self.0[1]
  }
  #[inline]
  pub fn z_ref(&self) -> &i64 {
    &self.0[2]
  }
  #[inline]
  pub fn x_mut(&mut self) -> &mut i64 {
    &mut self.0[0]
  }
  #[inline]
  pub fn y_mut(&mut self) -> &mut i64 {
    &mut self.0[1]
  }
  #[inline]
  pub fn z_mut(&mut self) -> &mut i64 {
    &mut self.0[2]
  }
  #[inline]
  pub fn xyz(&self) -> [i64; 3] {
    self.0.as_chunks().0[0]
  }
  #[inline]
  pub fn xyz_ref(&self) -> &[i64; 3] {
    &self.0.as_chunks().0[0]
  }
  #[inline]
  pub fn xyz_mut(&mut self) -> &mut [i64; 3] {
    &mut self.0.as_chunks_mut().0[0]
  }
  #[inline]
  pub fn dist(&self) -> u64 {
    (self.x().abs() as u64)
      + (self.y().abs() as u64)
      + (self.z().abs() as u64)
  }
}
impl From<[i64; 3]> for BlockDist {
  #[inline]
  fn from(value: [i64; 3]) -> Self {
    Self::new(value)
  }
}
impl Add<Self> for BlockDist {
  type Output = Self;

  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    Self([
      self.0[0] + rhs.0[0],
      self.0[1] + rhs.0[1],
      self.0[2] + rhs.0[2],
      self.0[3] + rhs.0[3],
    ])
  }
}
impl AddAssign<Self> for BlockDist {
  #[inline]
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs
  }
}
impl Sub<Self> for BlockDist {
  type Output = Self;

  #[inline]
  fn sub(self, rhs: Self) -> Self::Output {
    Self([
      self.0[0] + rhs.0[0],
      self.0[1] + rhs.0[1],
      self.0[2] + rhs.0[2],
      self.0[3] + rhs.0[3],
    ])
  }
}
impl SubAssign<Self> for BlockDist {
  #[inline]
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs
  }
}
impl Mul<Self> for BlockDist {
  type Output = Self;

  #[inline]
  fn mul(self, rhs: Self) -> Self::Output {
    Self([
      self.0[0] * rhs.0[0],
      self.0[1] * rhs.0[1],
      self.0[2] * rhs.0[2],
      self.0[3] * rhs.0[3],
    ])
  }
}
impl MulAssign<Self> for BlockDist {
  #[inline]
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs
  }
}
impl Div<Self> for BlockDist {
  type Output = Self;

  #[inline]
  fn div(self, rhs: Self) -> Self::Output {
    Self([
      self.0[0] / rhs.0[0],
      self.0[1] / rhs.0[1],
      self.0[2] / rhs.0[2],
      self.0[3] / rhs.0[3],
    ])
  }
}
impl DivAssign<Self> for BlockDist {
  #[inline]
  fn div_assign(&mut self, rhs: Self) {
    *self = *self / rhs
  }
}
impl Mul<i64> for BlockDist {
  type Output = Self;

  #[inline]
  fn mul(self, rhs: i64) -> Self::Output {
    Self([
      self.0[0] * rhs,
      self.0[1] * rhs,
      self.0[2] * rhs,
      self.0[3] * rhs,
    ])
  }
}
impl MulAssign<i64> for BlockDist {
  #[inline]
  fn mul_assign(&mut self, rhs: i64) {
    *self = *self * rhs
  }
}
impl Div<i64> for BlockDist {
  type Output = Self;

  #[inline]
  fn div(self, rhs: i64) -> Self::Output {
    Self([
      self.0[0] / rhs,
      self.0[1] / rhs,
      self.0[2] / rhs,
      self.0[3] / rhs,
    ])
  }
}
impl DivAssign<i64> for BlockDist {
  #[inline]
  fn div_assign(&mut self, rhs: i64) {
    *self = *self / rhs
  }
}
