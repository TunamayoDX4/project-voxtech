//! VoxTechの方向に関する定義

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
  /// West
  WST = 0,

  /// East
  EST = 1,

  /// South
  STH = 2,

  /// North
  NTH = 3,

  /// Bottom
  BTM = 4,

  /// Top
  TOP = 5,

  /// Undefined
  UNDEF = u8::MAX,
}
impl Dir {
  /// 方向の数
  pub const COUNT: u8 = 6;

  #[inline]
  pub fn new(value: u8) -> Self {
    match value {
      0 => Self::WST,
      1 => Self::EST,
      2 => Self::STH,
      3 => Self::NTH,
      4 => Self::BTM,
      5 => Self::TOP,
      _ => Self::UNDEF,
    }
  }
}
impl From<Axis> for Dir {
  #[inline]
  fn from(value: Axis) -> Self {
    Self::new((value as u8) << 1)
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
  UNDEF = 127,
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
      _ => Self::UNDEF,
    }
  }

  #[inline]
  pub unsafe fn from_dir_unchecked(dir: Dir) -> Self {
    unsafe { std::mem::transmute((dir as u8) >> 2) }
  }
}
impl From<Dir> for Axis {
  #[inline]
  fn from(value: Dir) -> Self {
    Self::new((value as u8) >> 1)
  }
}
