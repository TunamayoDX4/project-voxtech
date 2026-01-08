pub mod storage;

/// セルの大まかな種別を表記するためのデータ型
#[repr(u8)]
#[derive(
  Debug, Clone, Copy, Default, PartialEq, Eq, Hash,
)]
pub enum CellType {
  #[default]
  AllAir = 0,
  HasBasicBlock = 1,
  HasGenericBlock = 2,
  HasNormalBlock = 3,
  HasExtendBlock = 4,

  /// Undefined(ERROR)
  UNDEF = u8::MAX,
}
impl From<u8> for CellType {
  fn from(value: u8) -> Self {
    match value {
      0 => Self::AllAir,
      1 => Self::HasBasicBlock,
      2 => Self::HasGenericBlock,
      3 => Self::HasExtendBlock,
      _ => Self::UNDEF,
    }
  }
}
impl PartialOrd for CellType {
  fn partial_cmp(
    &self,
    other: &Self,
  ) -> Option<std::cmp::Ordering> {
    match (self, other) {
      (Self::UNDEF, _) | (_, Self::UNDEF) => None,
      (a, b) => Some(std::cmp::Ord::cmp(
        &(*a as u8),
        &(*b as u8),
      )),
    }
  }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CellInfo {
  ty: CellType,
  non_air_bits: u64,
}

/// ブロックを構成するバイトデータを64個格納するためのメモリ
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct CellMem([u8; 64]);
impl Default for CellMem {
  fn default() -> Self {
    Self([0; 64])
  }
}

/// ブロックを構成するバイト列の列レベルでの種類識別情報
#[repr(u8)]
#[derive(
  Default, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum CellMemType {
  /// 未使用
  #[default]
  NotUsing = 0,
  /// 低位ID
  LoID,
  /// システムタグ
  SysTag,
  /// 高位ID or Generic Tag
  HiIDorGT,
  /// ユーザタグ
  UserTag,
}
