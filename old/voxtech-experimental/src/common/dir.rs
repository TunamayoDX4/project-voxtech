//! VoxTechの方向に関する定義

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
  /// West
  Wst = 0,

  /// East
  Est = 1,

  /// South
  Sth = 2,

  /// North
  Nth = 3,

  /// Bottom
  Btm = 4,

  /// Top
  Top = 5,

  /// Undefined
  Undef = u8::MAX,
}
impl Dir {
  /// 方向の数
  pub const COUNT: u8 = 6;

  #[inline]
  pub fn new(value: u8) -> Self {
    match value {
      0 => Self::Wst,
      1 => Self::Est,
      2 => Self::Sth,
      3 => Self::Nth,
      4 => Self::Btm,
      5 => Self::Top,
      _ => Self::Undef,
    }
  }

  #[inline]
  pub fn iter() -> impl Iterator<Item = Dir> {
    (0..6u8).map(|i| unsafe { std::mem::transmute(i) })
  }

  #[inline]
  pub fn is_positive(&self) -> u8 {
    *self as u8 & 1
  }

  #[inline]
  pub fn is_negative(&self) -> u8 {
    1 - self.is_positive()
  }

  #[inline]
  pub fn invert(&self) -> Self {
    unsafe {
      std::mem::transmute(
        (*self as u8 & !1) | (!(*self as u8) & 1),
      )
    }
  }
}
impl From<u8> for Dir {
  #[inline]
  fn from(value: u8) -> Self {
    Self::new(value)
  }
}
impl From<Axis> for Dir {
  #[inline]
  fn from(value: Axis) -> Self {
    Self::new((value as u8) << 1)
  }
}
impl std::fmt::Display for Dir {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.write_str(match self {
      Dir::Wst => "West",
      Dir::Est => "East",
      Dir::Sth => "South",
      Dir::Nth => "North",
      Dir::Btm => "Bottom",
      Dir::Top => "Top",
      Dir::Undef => "Undefined",
    })
  }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
  /// West-East
  WE = 0,

  /// South-North
  SN = 1,

  /// Bottom-Top
  BT = 2,

  /// Undefined
  Undef = 127,
}
impl Axis {
  /// 方向の数
  pub const COUNT: u8 = 3;

  #[inline]
  pub fn new(value: u8) -> Self {
    match value {
      0 => Self::BT,
      1 => Self::SN,
      2 => Self::BT,
      _ => Self::Undef,
    }
  }

  #[inline]
  pub fn iter() -> impl Iterator<Item = Dir> {
    (0..3u8).map(|i| unsafe { std::mem::transmute(i) })
  }

  #[inline]
  pub unsafe fn from_dir_unchecked(dir: Dir) -> Self {
    unsafe { std::mem::transmute((dir as u8) >> 1) }
  }

  /// 64分木における面を作る上でのステップ／インデックスの分布
  #[inline]
  pub fn idx_step(&self) -> (u8, u8) {
    match self {
      Self::WE => (1, 4),
      Self::SN => (4, 1),
      Self::BT => (1, 1),
      _ => (0, 0),
    }
  }
}
impl From<u8> for Axis {
  #[inline]
  fn from(value: u8) -> Self {
    Self::new(value)
  }
}
impl From<Dir> for Axis {
  #[inline]
  fn from(value: Dir) -> Self {
    Self::new((value as u8) >> 1)
  }
}
impl std::fmt::Display for Axis {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.write_str(match self {
      Axis::WE => "West->East",
      Axis::SN => "South->North",
      Axis::BT => "Bottom->Top",
      Axis::Undef => "Undefined",
    })
  }
}
