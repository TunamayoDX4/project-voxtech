/// 64分木の内部座標
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tree64InnerPos(u8);
impl From<BlockPos> for Tree64InnerPos {
  #[inline]
  fn from(value: BlockPos) -> Self {
    Self(
      (*value.x() & 3) as u8
        | (*value.y() & 3 << 2) as u8
        | (*value.z() & 3 << 4) as u8
        | (*value.w() & 3 << 6) as u8,
    )
  }
}
impl Tree64InnerPos {
  #[inline]
  pub fn new(pos: u8) -> Self {
    Self(pos & 63)
  }
  #[inline]
  pub fn invalid() -> Self {
    Self(u8::MAX)
  }

  #[inline]
  pub fn is_valid(&self) -> bool {
    self.0 < 64
  }
}

/// 64分木の内部距離
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tree64InnerDist(u8);

/// ブロック用の座標構造体
#[repr(C, align(32))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos(pub [i64; 4]);
impl From<[i64; 3]> for BlockPos {
  fn from(value: [i64; 3]) -> Self {
    Self([value[0], value[1], value[2], 0])
  }
}
impl BlockPos {
  pub fn new(x: i64, y: i64, z: i64) -> Self {
    Self([x, y, z, 0])
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
  pub fn w(&self) -> &i64 {
    &self.0[3]
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
  pub fn w_mut(&mut self) -> &mut i64 {
    &mut self.0[3]
  }

  #[inline]
  pub fn insert_innerpos(
    &self,
    inner_pos: Tree64InnerPos,
  ) -> Self {
    Self([
      self.0[0] << 2 | (inner_pos.0 & 3) as i64,
      self.0[1] << 2 | (inner_pos.0 >> 2 & 3) as i64,
      self.0[2] << 2 | (inner_pos.0 >> 4 & 3) as i64,
      self.0[3] << 2 | (inner_pos.0 >> 6 & 3) as i64,
    ])
  }

  #[inline]
  pub fn split_innerpos(&self) -> (Tree64InnerPos, Self) {
    (
      Tree64InnerPos::new(
        (self.0[0] & 3) as u8
          | (self.0[1] & 3 << 2) as u8
          | (self.0[2] & 3 << 4) as u8
          | (self.0[3] & 3 << 6) as u8,
      ),
      Self([
        self.0[0] >> 2,
        self.0[1] >> 2,
        self.0[2] >> 2,
        self.0[3] >> 2,
      ]),
    )
  }

  #[inline]
  pub fn level_up(&self, level: u8) -> Self {
    Self([
      self.0[0] << level * 2,
      self.0[1] << level * 2,
      self.0[2] << level * 2,
      self.0[3] << level * 2,
    ])
  }
  #[inline]
  pub fn level_down(&self, level: u8) -> Self {
    Self([
      self.0[0] >> level * 2,
      self.0[1] >> level * 2,
      self.0[2] >> level * 2,
      self.0[3] >> level * 2,
    ])
  }
}

#[repr(C, align(32))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockDist(pub [i64; 4]);
