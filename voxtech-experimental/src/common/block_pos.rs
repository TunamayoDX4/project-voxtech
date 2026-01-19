//! VoxTech内部のブロック座標関連の型の定義

use core::f64;
use std::ops::{
  Add, AddAssign, BitAnd, BitAndAssign, BitOr,
  BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
  Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign,
  Shr, ShrAssign, Sub, SubAssign,
};

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash, Pod, Zeroable,
)]
pub struct InnerBlockPos(pub u8);
impl InnerBlockPos {
  #[inline]
  pub fn new(pos: u8) -> Self {
    Self(pos & 63)
  }
  #[inline]
  pub fn new_xyz(x: u8, y: u8, z: u8) -> Self {
    Self::from([x, y, z, 0])
  }
  #[inline]
  pub fn proj_xy(&self) -> u8 {
    self.0 & 15
  }
  #[inline]
  pub fn proj_xz(&self) -> u8 {
    (self.0 & 3) | ((self.0 >> 2) & 12)
  }
  #[inline]
  pub fn proj_yz(&self) -> u8 {
    ((self.0 << 2) & 12) | ((self.0 >> 4) & 3)
  }
  #[inline]
  pub fn neigh_west(self) -> (bool, Self) {
    let neigh = (self.0 & 3).wrapping_sub(1);
    let outer = neigh == u8::MAX;
    let ret = Self((self.0 & !3) | neigh & 3);
    (outer, ret)
  }
  #[inline]
  pub fn neigh_east(self) -> (bool, Self) {
    let neigh = (self.0 | !3).wrapping_add(1);
    let outer = neigh == 0;
    let ret = Self((self.0 & !3) | neigh & 3);
    (outer, ret)
  }
  #[inline]
  pub fn neigh_south(self) -> (bool, Self) {
    let neigh = ((self.0 >> 2) | 3).wrapping_sub(1);
    let outer = neigh == u8::MAX;
    let ret = Self((self.0 & !12) | (neigh & 3) << 2);
    (outer, ret)
  }
  #[inline]
  pub fn neigh_north(self) -> (bool, Self) {
    let neigh = ((self.0 >> 2) | !3).wrapping_add(1);
    let outer = neigh == 0;
    let ret = Self((self.0 & !12) | (neigh & 3) << 2);
    (outer, ret)
  }
  #[inline]
  pub fn neigh_bottom(self) -> (bool, Self) {
    let neigh = ((self.0 >> 4) | 3).wrapping_sub(1);
    let outer = neigh == u8::MAX;
    let ret = Self((self.0 & !48) | (neigh & 3) << 4);
    (outer, ret)
  }
  #[inline]
  pub fn neigh_top(self) -> (bool, Self) {
    let neigh = ((self.0 >> 4) | !3).wrapping_add(1);
    let outer = neigh == 0;
    let ret = Self((self.0 & !48) | (neigh & 3) << 4);
    (outer, ret)
  }
}
impl From<[u8; 4]> for InnerBlockPos {
  fn from(value: [u8; 4]) -> Self {
    Self(
      value[0] & 3
        | value[1] & 3 << 2
        | value[2] & 3 << 4
        | value[3] & 3 << 6,
    )
  }
}
impl From<InnerBlockPos> for [u8; 4] {
  fn from(value: InnerBlockPos) -> Self {
    [
      value.0 & 3,
      value.0 >> 2 & 3,
      value.0 >> 4 & 3,
      value.0 >> 6 & 3,
    ]
  }
}

#[repr(C, align(32))]
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash, Pod, Zeroable,
)]
pub struct BlockPos([i64; 4]);
impl From<[i64; 4]> for BlockPos {
  #[inline]
  fn from(value: [i64; 4]) -> Self {
    Self(value)
  }
}
impl From<BlockPos> for [i64; 4] {
  #[inline]
  fn from(value: BlockPos) -> Self {
    value.0
  }
}
impl From<InnerBlockPos> for BlockPos {
  #[inline]
  fn from(value: InnerBlockPos) -> Self {
    Self([
      (value.0 & 3) as _,
      ((value.0 >> 2) & 3) as _,
      ((value.0 >> 4) & 3) as _,
      ((value.0 >> 6) & 3) as _,
    ])
  }
}
impl From<BlockPos> for InnerBlockPos {
  #[inline]
  fn from(value: BlockPos) -> Self {
    Self(
      (value.0[0] & 3) as u8
        | ((value.0[1] & 3) << 2) as u8
        | ((value.0[2] & 3) << 4) as u8
        | ((value.0[3] & 3) << 6) as u8,
    )
  }
}
impl Sub<BlockPos> for BlockPos {
  type Output = BlockDist;

  #[inline]
  fn sub(self, rhs: BlockPos) -> Self::Output {
    BlockDist([
      self.0[0] - rhs.0[0],
      self.0[1] - rhs.0[1],
      self.0[2] - rhs.0[2],
      self.0[3] - rhs.0[3],
    ])
  }
}
impl Add<BlockDist> for BlockPos {
  type Output = BlockPos;

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
    *self = *self + rhs;
  }
}
impl Sub<BlockDist> for BlockPos {
  type Output = BlockPos;

  #[inline]
  fn sub(self, rhs: BlockDist) -> Self::Output {
    Self([
      self.0[0] - rhs.0[0],
      self.0[1] - rhs.0[1],
      self.0[2] - rhs.0[2],
      self.0[3] - rhs.0[3],
    ])
  }
}
impl SubAssign<BlockDist> for BlockPos {
  #[inline]
  fn sub_assign(&mut self, rhs: BlockDist) {
    *self = *self - rhs;
  }
}

impl BitAnd<BlockPos> for BlockPos {
  type Output = BlockPos;

  #[inline]
  fn bitand(self, rhs: BlockPos) -> Self::Output {
    Self([
      self.0[0] & rhs.0[0],
      self.0[1] & rhs.0[1],
      self.0[2] & rhs.0[2],
      self.0[3] & rhs.0[3],
    ])
  }
}
impl BitAndAssign<BlockPos> for BlockPos {
  #[inline]
  fn bitand_assign(&mut self, rhs: BlockPos) {
    *self = *self & rhs
  }
}
impl BitOr<BlockPos> for BlockPos {
  type Output = BlockPos;

  #[inline]
  fn bitor(self, rhs: BlockPos) -> Self::Output {
    Self([
      self.0[0] | rhs.0[0],
      self.0[1] | rhs.0[1],
      self.0[2] | rhs.0[2],
      self.0[3] | rhs.0[3],
    ])
  }
}
impl BitOrAssign<BlockPos> for BlockPos {
  #[inline]
  fn bitor_assign(&mut self, rhs: BlockPos) {
    *self = *self | rhs
  }
}
impl BitXor<BlockPos> for BlockPos {
  type Output = BlockPos;

  #[inline]
  fn bitxor(self, rhs: BlockPos) -> Self::Output {
    Self([
      self.0[0] ^ rhs.0[0],
      self.0[1] ^ rhs.0[1],
      self.0[2] ^ rhs.0[2],
      self.0[3] ^ rhs.0[3],
    ])
  }
}
impl BitXorAssign<BlockPos> for BlockPos {
  #[inline]
  fn bitxor_assign(&mut self, rhs: BlockPos) {
    *self = *self ^ rhs
  }
}
impl Shl<BlockPos> for BlockPos {
  type Output = BlockPos;

  #[inline]
  fn shl(self, rhs: BlockPos) -> Self::Output {
    BlockPos([
      self.0[0] << rhs.0[0],
      self.0[1] << rhs.0[1],
      self.0[2] << rhs.0[2],
      self.0[3] << rhs.0[3],
    ])
  }
}
impl ShlAssign<BlockPos> for BlockPos {
  #[inline]
  fn shl_assign(&mut self, rhs: BlockPos) {
    *self = *self << rhs
  }
}
impl Shr<BlockPos> for BlockPos {
  type Output = BlockPos;

  #[inline]
  fn shr(self, rhs: BlockPos) -> Self::Output {
    BlockPos([
      self.0[0] >> rhs.0[0],
      self.0[1] >> rhs.0[1],
      self.0[2] >> rhs.0[2],
      self.0[3] >> rhs.0[3],
    ])
  }
}
impl ShrAssign<BlockPos> for BlockPos {
  #[inline]
  fn shr_assign(&mut self, rhs: BlockPos) {
    *self = *self >> rhs
  }
}

impl BitAnd<i64> for BlockPos {
  type Output = BlockPos;

  #[inline]
  fn bitand(self, rhs: i64) -> Self::Output {
    Self([
      self.0[0] & rhs,
      self.0[1] & rhs,
      self.0[2] & rhs,
      self.0[3] & rhs,
    ])
  }
}
impl BitOr<i64> for BlockPos {
  type Output = BlockPos;

  #[inline]
  fn bitor(self, rhs: i64) -> Self::Output {
    Self([
      self.0[0] | rhs,
      self.0[1] | rhs,
      self.0[2] | rhs,
      self.0[3] | rhs,
    ])
  }
}
impl BitXor<i64> for BlockPos {
  type Output = BlockPos;

  #[inline]
  fn bitxor(self, rhs: i64) -> Self::Output {
    Self([
      self.0[0] ^ rhs,
      self.0[1] ^ rhs,
      self.0[2] ^ rhs,
      self.0[3] ^ rhs,
    ])
  }
}
impl Shl<i64> for BlockPos {
  type Output = BlockPos;

  #[inline]
  fn shl(self, rhs: i64) -> Self::Output {
    Self([
      self.0[0] << rhs,
      self.0[1] << rhs,
      self.0[2] << rhs,
      self.0[3] << rhs,
    ])
  }
}
impl Shr<i64> for BlockPos {
  type Output = BlockPos;

  #[inline]
  fn shr(self, rhs: i64) -> Self::Output {
    Self([
      self.0[0] >> rhs,
      self.0[1] >> rhs,
      self.0[2] >> rhs,
      self.0[3] >> rhs,
    ])
  }
}
impl BlockPos {
  #[inline]
  pub fn new(x: i64, y: i64, z: i64) -> Self {
    Self([x, y, z, 0])
  }

  #[inline]
  pub fn raw(&self) -> &[i64; 3] {
    &self.0.as_chunks().0[0]
  }

  #[inline]
  pub fn raw_mut(&mut self) -> &mut [i64; 3] {
    &mut self.0.as_chunks_mut().0[0]
  }

  #[inline]
  pub fn x(&self) -> &i64 {
    &self.0[0]
  }

  #[inline]
  pub fn y(&self) -> &i64 {
    &self.0[1]
  }

  #[inline]
  pub fn z(&self) -> &i64 {
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
  pub fn abs(&self) -> Self {
    Self([
      self.0[0].abs(),
      self.0[1].abs(),
      self.0[2].abs(),
      self.0[3].abs(),
    ])
  }

  /// 2*levelビット分上げる
  #[inline]
  pub fn level_up(&self, level: u8) -> Self {
    *self << (2 * level) as i64
  }

  /// 2*levelビット分下げる
  #[inline]
  pub fn level_down(&self, level: u8) -> Self {
    *self >> (2 * level) as i64
  }

  /// 下位2*levelビットを切り上げる
  #[inline]
  pub fn cut_up(&self, level: u8) -> Self {
    *self & i64::MAX << (2 * level)
  }

  /// 上位2*levelビットを切り下げる
  #[inline]
  pub fn cut_down(&self, level: u8) -> Self {
    let right = !i64::MAX << (2 * level);
    *self & right
  }

  /// 下位2ビットを切り上げる
  #[inline]
  pub fn cut_up_1(&self) -> Self {
    *self & !3
  }

  /// 上位2ビットを切り下げる
  #[inline]
  pub fn cut_down_1(&self) -> Self {
    *self & 3
  }

  /// 2*levelビットで切り分ける
  #[inline]
  pub fn split(&self, level: u8) -> [Self; 2] {
    [
      self.cut_up(level),
      self.cut_down(level),
    ]
  }

  /// 2*levelビットで結合する
  #[inline]
  pub fn merge(&self, right: &Self, level: u8) -> Self {
    let left = self.cut_up(level);
    let right = right.cut_down(level);
    Self([
      left.0[0] + right.0[0],
      left.0[1] + right.0[1],
      left.0[2] + right.0[2],
      left.0[3] + right.0[3],
    ])
  }

  /// 2ビットで切り分ける
  #[inline]
  pub fn split_1(&self) -> [Self; 2] {
    [
      self.cut_up_1(),
      self.cut_down_1(),
    ]
  }

  /// 2ビットで結合する
  #[inline]
  pub fn merge_1(&self, right: &Self) -> Self {
    let left = self.cut_up_1();
    let right = right.cut_down_1();
    Self([
      left.0[0] + right.0[0],
      left.0[1] + right.0[1],
      left.0[2] + right.0[2],
      left.0[3] + right.0[3],
    ])
  }
}

#[repr(C, align(32))]
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash, Pod, Zeroable,
)]
pub struct BlockDist([i64; 4]);
impl From<[i64; 4]> for BlockDist {
  fn from(value: [i64; 4]) -> Self {
    Self(value)
  }
}
impl From<BlockDist> for [i64; 4] {
  fn from(value: BlockDist) -> Self {
    value.0
  }
}
impl From<InnerBlockPos> for BlockDist {
  #[inline]
  fn from(value: InnerBlockPos) -> Self {
    Self([
      (value.0 & 3) as _,
      ((value.0 >> 2) & 3) as _,
      ((value.0 >> 4) & 3) as _,
      ((value.0 >> 6) & 3) as _,
    ])
  }
}
impl From<BlockDist> for InnerBlockPos {
  #[inline]
  fn from(value: BlockDist) -> Self {
    Self(
      (value.0[0] & 3) as u8
        | ((value.0[1] & 3) << 2) as u8
        | ((value.0[2] & 3) << 4) as u8
        | ((value.0[3] & 3) << 6) as u8,
    )
  }
}
impl BlockDist {
  #[inline]
  pub fn new(x: i64, y: i64, z: i64) -> Self {
    Self([x, y, z, 0])
  }

  #[inline]
  pub fn raw(&self) -> &[i64; 3] {
    &self.0.as_chunks().0[0]
  }

  #[inline]
  pub fn raw_mut(&mut self) -> &mut [i64; 3] {
    &mut self.0.as_chunks_mut().0[0]
  }

  #[inline]
  pub fn x(&self) -> &i64 {
    &self.0[0]
  }

  #[inline]
  pub fn y(&self) -> &i64 {
    &self.0[1]
  }

  #[inline]
  pub fn z(&self) -> &i64 {
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
  pub fn abs(&self) -> Self {
    Self([
      self.0[0].abs(),
      self.0[1].abs(),
      self.0[2].abs(),
      self.0[3].abs(),
    ])
  }

  /// 2*levelビット分上げる
  #[inline]
  pub fn level_up(&self, level: u8) -> Self {
    *self << (2 * level) as i64
  }

  /// 2*levelビット分下げる
  #[inline]
  pub fn level_down(&self, level: u8) -> Self {
    *self >> (2 * level) as i64
  }

  /// 下位2*levelビットを切り上げる
  #[inline]
  pub fn cut_up(&self, level: u8) -> Self {
    *self & i64::MAX << (2 * level)
  }

  /// 上位2*levelビットを切り下げる
  #[inline]
  pub fn cut_down(&self, level: u8) -> Self {
    let right = !i64::MAX << (2 * level);
    *self & right
  }

  /// 下位2ビットを切り上げる
  #[inline]
  pub fn cut_up_1(&self) -> Self {
    *self & !3
  }

  /// 上位2ビットを切り下げる
  #[inline]
  pub fn cut_down_1(&self) -> Self {
    *self & 3
  }

  /// 2*levelビットで切り分ける
  #[inline]
  pub fn split(&self, level: u8) -> [Self; 2] {
    [
      self.cut_up(level),
      self.cut_down(level),
    ]
  }

  /// 2*levelビットで結合する
  #[inline]
  pub fn merge(&self, right: &Self, level: u8) -> Self {
    let left = self.cut_up(level);
    let right = right.cut_down(level);
    left | right
  }

  /// 2ビットで切り分ける
  #[inline]
  pub fn split_1(&self) -> [Self; 2] {
    [
      self.cut_up_1(),
      self.cut_down_1(),
    ]
  }

  /// 2ビットで結合する
  #[inline]
  pub fn merge_1(&self, right: &Self) -> Self {
    let left = self.cut_up_1();
    let right = right.cut_down_1();
    Self([
      left.0[0] + right.0[0],
      left.0[1] + right.0[1],
      left.0[2] + right.0[2],
      left.0[3] + right.0[3],
    ])
  }
}
impl Not for BlockDist {
  type Output = Self;

  #[inline]
  fn not(self) -> Self::Output {
    Self([
      !self.0[0], !self.0[1], !self.0[2], !self.0[3],
    ])
  }
}
impl Add<BlockDist> for BlockDist {
  type Output = BlockDist;

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
impl AddAssign<BlockDist> for BlockDist {
  #[inline]
  fn add_assign(&mut self, rhs: BlockDist) {
    *self = *self + rhs
  }
}
impl Sub<BlockDist> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn sub(self, rhs: BlockDist) -> Self::Output {
    Self([
      self.0[0] - rhs.0[0],
      self.0[1] - rhs.0[1],
      self.0[2] - rhs.0[2],
      self.0[3] - rhs.0[3],
    ])
  }
}
impl SubAssign<BlockDist> for BlockDist {
  #[inline]
  fn sub_assign(&mut self, rhs: BlockDist) {
    *self = *self - rhs
  }
}
impl Mul<BlockDist> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn mul(self, rhs: BlockDist) -> Self::Output {
    Self([
      self.0[0] * rhs.0[0],
      self.0[1] * rhs.0[1],
      self.0[2] * rhs.0[2],
      self.0[3] * rhs.0[3],
    ])
  }
}
impl MulAssign<BlockDist> for BlockDist {
  #[inline]
  fn mul_assign(&mut self, rhs: BlockDist) {
    *self = *self * rhs
  }
}
impl Div<BlockDist> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn div(self, rhs: BlockDist) -> Self::Output {
    Self([
      self.0[0] / rhs.0[0],
      self.0[1] / rhs.0[1],
      self.0[2] / rhs.0[2],
      self.0[3] / rhs.0[3],
    ])
  }
}
impl DivAssign<BlockDist> for BlockDist {
  #[inline]
  fn div_assign(&mut self, rhs: BlockDist) {
    *self = *self / rhs
  }
}
impl Rem<BlockDist> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn rem(self, rhs: BlockDist) -> Self::Output {
    Self([
      self.0[0] % rhs.0[0],
      self.0[1] % rhs.0[1],
      self.0[2] % rhs.0[2],
      self.0[3] % rhs.0[3],
    ])
  }
}
impl RemAssign<BlockDist> for BlockDist {
  #[inline]
  fn rem_assign(&mut self, rhs: BlockDist) {
    *self = *self % rhs
  }
}
impl BitAnd<BlockDist> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn bitand(self, rhs: BlockDist) -> Self::Output {
    Self([
      self.0[0] & rhs.0[0],
      self.0[1] & rhs.0[1],
      self.0[2] & rhs.0[2],
      self.0[3] & rhs.0[3],
    ])
  }
}
impl BitAndAssign<BlockDist> for BlockDist {
  #[inline]
  fn bitand_assign(&mut self, rhs: BlockDist) {
    *self = *self & rhs
  }
}
impl BitOr<BlockDist> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn bitor(self, rhs: BlockDist) -> Self::Output {
    Self([
      self.0[0] | rhs.0[0],
      self.0[1] | rhs.0[1],
      self.0[2] | rhs.0[2],
      self.0[3] | rhs.0[3],
    ])
  }
}
impl BitOrAssign<BlockDist> for BlockDist {
  #[inline]
  fn bitor_assign(&mut self, rhs: BlockDist) {
    *self = *self | rhs
  }
}
impl BitXor<BlockDist> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn bitxor(self, rhs: BlockDist) -> Self::Output {
    Self([
      self.0[0] ^ rhs.0[0],
      self.0[1] ^ rhs.0[1],
      self.0[2] ^ rhs.0[2],
      self.0[3] ^ rhs.0[3],
    ])
  }
}
impl BitXorAssign<BlockDist> for BlockDist {
  #[inline]
  fn bitxor_assign(&mut self, rhs: BlockDist) {
    *self = *self ^ rhs
  }
}
impl Shl<BlockDist> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn shl(self, rhs: BlockDist) -> Self::Output {
    BlockDist([
      self.0[0] << rhs.0[0],
      self.0[1] << rhs.0[1],
      self.0[2] << rhs.0[2],
      self.0[3] << rhs.0[3],
    ])
  }
}
impl ShlAssign<BlockDist> for BlockDist {
  #[inline]
  fn shl_assign(&mut self, rhs: BlockDist) {
    *self = *self << rhs
  }
}
impl Shr<BlockDist> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn shr(self, rhs: BlockDist) -> Self::Output {
    BlockDist([
      self.0[0] >> rhs.0[0],
      self.0[1] >> rhs.0[1],
      self.0[2] >> rhs.0[2],
      self.0[3] >> rhs.0[3],
    ])
  }
}
impl ShrAssign<BlockDist> for BlockDist {
  #[inline]
  fn shr_assign(&mut self, rhs: BlockDist) {
    *self = *self >> rhs
  }
}

impl Add<i64> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn add(self, rhs: i64) -> Self::Output {
    Self([
      self.0[0] + rhs,
      self.0[1] + rhs,
      self.0[2] + rhs,
      self.0[3] + rhs,
    ])
  }
}
impl AddAssign<i64> for BlockDist {
  #[inline]
  fn add_assign(&mut self, rhs: i64) {
    *self = *self + rhs
  }
}
impl Sub<i64> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn sub(self, rhs: i64) -> Self::Output {
    Self([
      self.0[0] - rhs,
      self.0[1] - rhs,
      self.0[2] - rhs,
      self.0[3] - rhs,
    ])
  }
}
impl SubAssign<i64> for BlockDist {
  #[inline]
  fn sub_assign(&mut self, rhs: i64) {
    *self = *self - rhs
  }
}
impl Mul<i64> for BlockDist {
  type Output = BlockDist;

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
  type Output = BlockDist;

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
impl Rem<i64> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn rem(self, rhs: i64) -> Self::Output {
    Self([
      self.0[0] % rhs,
      self.0[1] % rhs,
      self.0[2] % rhs,
      self.0[3] % rhs,
    ])
  }
}
impl RemAssign<i64> for BlockDist {
  #[inline]
  fn rem_assign(&mut self, rhs: i64) {
    *self = *self % rhs
  }
}
impl BitAnd<i64> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn bitand(self, rhs: i64) -> Self::Output {
    Self([
      self.0[0] & rhs,
      self.0[1] & rhs,
      self.0[2] & rhs,
      self.0[3] & rhs,
    ])
  }
}
impl BitAndAssign<i64> for BlockDist {
  #[inline]
  fn bitand_assign(&mut self, rhs: i64) {
    *self = *self & rhs
  }
}
impl BitOr<i64> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn bitor(self, rhs: i64) -> Self::Output {
    Self([
      self.0[0] | rhs,
      self.0[1] | rhs,
      self.0[2] | rhs,
      self.0[3] | rhs,
    ])
  }
}
impl BitOrAssign<i64> for BlockDist {
  #[inline]
  fn bitor_assign(&mut self, rhs: i64) {
    *self = *self | rhs
  }
}
impl BitXor<i64> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn bitxor(self, rhs: i64) -> Self::Output {
    Self([
      self.0[0] ^ rhs,
      self.0[1] ^ rhs,
      self.0[2] ^ rhs,
      self.0[3] ^ rhs,
    ])
  }
}
impl BitXorAssign<i64> for BlockDist {
  #[inline]
  fn bitxor_assign(&mut self, rhs: i64) {
    *self = *self ^ rhs
  }
}
impl Shl<i64> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn shl(self, rhs: i64) -> Self::Output {
    BlockDist([
      self.0[0] << rhs,
      self.0[1] << rhs,
      self.0[2] << rhs,
      self.0[3] << rhs,
    ])
  }
}
impl ShlAssign<i64> for BlockDist {
  #[inline]
  fn shl_assign(&mut self, rhs: i64) {
    *self = *self << rhs
  }
}
impl Shr<i64> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn shr(self, rhs: i64) -> Self::Output {
    BlockDist([
      self.0[0] >> rhs,
      self.0[1] >> rhs,
      self.0[2] >> rhs,
      self.0[3] >> rhs,
    ])
  }
}
impl ShrAssign<i64> for BlockDist {
  #[inline]
  fn shr_assign(&mut self, rhs: i64) {
    *self = *self >> rhs
  }
}
impl Mul<f64> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn mul(self, rhs: f64) -> Self::Output {
    Self([
      ((self.0[0] as f64 * rhs).floor() + f64::EPSILON)
        as i64,
      ((self.0[0] as f64 * rhs).floor() + f64::EPSILON)
        as i64,
      ((self.0[0] as f64 * rhs).floor() + f64::EPSILON)
        as i64,
      ((self.0[0] as f64 * rhs).floor() + f64::EPSILON)
        as i64,
    ])
  }
}
impl MulAssign<f64> for BlockDist {
  #[inline]
  fn mul_assign(&mut self, rhs: f64) {
    *self = *self * rhs
  }
}
impl Div<f64> for BlockDist {
  type Output = BlockDist;

  #[inline]
  fn div(self, rhs: f64) -> Self::Output {
    Self([
      ((self.0[0] as f64 / rhs).floor() + f64::EPSILON)
        as i64,
      ((self.0[0] as f64 / rhs).floor() + f64::EPSILON)
        as i64,
      ((self.0[0] as f64 / rhs).floor() + f64::EPSILON)
        as i64,
      ((self.0[0] as f64 / rhs).floor() + f64::EPSILON)
        as i64,
    ])
  }
}
impl DivAssign<f64> for BlockDist {
  #[inline]
  fn div_assign(&mut self, rhs: f64) {
    *self = *self / rhs
  }
}
