pub mod ray_check;

pub struct AbstractBlock;

#[repr(C)]
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub struct BlockID(pub u16);
impl BlockID {
  pub fn new(
    lo: BlockLoID,
    hi: BlockHiID,
  ) -> Self {
    Self(
      lo.0 as u16 & !0x7F
        | ((hi.0 as u16) << 7) & 0x7F80,
    )
  }
}

/// ブロックの持つID(下位バイト)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BlockLoID(pub u8);
impl BlockLoID {
  #[inline]
  pub fn air() -> Self {
    Self(0)
  }
  #[inline]
  pub fn is_air(&self) -> bool {
    self.0 == 0
  }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BlockHiID(pub u8);
