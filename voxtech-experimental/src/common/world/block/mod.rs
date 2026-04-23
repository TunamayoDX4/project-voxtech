pub mod additional;
pub mod basic;

use std::ops::{Deref, DerefMut};

#[repr(u8)]
#[derive(
  Default, Debug, Clone, Copy, PartialEq, Eq,
)]
pub enum BlockType {
  Air = 0b00,

  #[default]
  Basic = 0b01,

  Additional = 0b10,

  EntityType = 0b11,
}

/// ブロックID下位バイト
///
/// 128 <= LoIDの場合…つまり最上位ビットが立っている場合には、
/// ブロックIDが2バイトであることを示す。
#[repr(C)]
#[derive(
  Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod,
)]
pub struct LoID(u8);
impl Deref for LoID {
  type Target = u8;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl DerefMut for LoID {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
impl LoID {
  /// ID下位バイトの値を取り出すためのビットフィールド
  pub const LO_ID_BITS: u8 = 0x7F;

  /// ID上位バイトの存在有無を示すフラグ
  pub const BIG_ID_FLAG: u8 = 0x80;

  /// ID下位バイトの初期化
  #[inline]
  pub fn new(id: u16) -> Self {
    let is_big = ((id <= (Self::LO_ID_BITS as u16))
      as u8)
      .wrapping_sub(1)
      & Self::BIG_ID_FLAG;
    let id = (id as u8) & Self::LO_ID_BITS;
    Self(id | is_big)
  }

  #[inline]
  pub fn is_big(&self) -> u8 {
    ((self.0 <= Self::LO_ID_BITS) as u8).wrapping_sub(1)
  }

  #[inline]
  pub fn id_body(&self) -> u16 {
    (self.0 & Self::LO_ID_BITS) as u16
  }
}

#[repr(C)]
#[derive(
  Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod,
)]
pub struct HiID(u8);
impl Deref for HiID {
  type Target = u8;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl DerefMut for HiID {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
impl HiID {
  /// ID上位バイトの値を取り出すためのビットフィールド
  pub const HI_ID_BITS: u16 = 0x7F80;

  /// ID上位バイトの初期化
  #[inline]
  pub fn new(id: u16) -> Self {
    let hi_id = id & Self::HI_ID_BITS >> 7;
    Self(hi_id as u8)
  }

  /// LoIDがbig_idなら有効化する
  #[inline]
  pub fn mask(self, lo_id: &LoID) -> Self {
    Self(self.0 & lo_id.is_big())
  }

  #[inline]
  pub fn id_body(&self) -> u16 {
    (self.0 as u16) << 7
  }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GenericTag(u8);
impl Deref for GenericTag {
  type Target = u8;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl DerefMut for GenericTag {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BlockID(u16);
impl Deref for BlockID {
  type Target = u16;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl BlockID {
  #[inline]
  pub fn new(lo_id: LoID, hi_id: HiID) -> Self {
    let hi_id = hi_id.mask(&lo_id).id_body();
    let lo_id = lo_id.id_body();
    Self(hi_id | lo_id)
  }

  #[inline]
  pub fn hi_lo(&self) -> (LoID, HiID) {
    (
      LoID::new(self.0),
      HiID::new(self.0),
    )
  }

  #[inline]
  pub fn is_big(&self) -> bool {
    (LoID::LO_ID_BITS as u16) < self.0
  }
}
