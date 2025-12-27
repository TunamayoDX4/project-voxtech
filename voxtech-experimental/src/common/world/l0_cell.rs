use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable,
)]
pub struct CellInfo {
  /// 空気じゃないブロックのビットボード
  pub non_air_bit: u64,
}
impl CellInfo {
  pub fn new(cell: &Cell) -> Self {
    let non_air_bit = cell.non_air_bits();

    Self { non_air_bit }
  }

  #[inline]
  pub fn write_block(
    &mut self,
    pos: u8,
    non_air: bool,
  ) {
    let mut nab = self.non_air_bit.to_be_bytes();
    let blk_pos = (pos / 8) % 8;
    let bit = (non_air as u8) << pos % 8;
    let nab_byte = nab[blk_pos as usize] & !bit;
    nab[blk_pos as usize] = nab_byte | bit;
    self.non_air_bit = u64::from_be_bytes(nab);
  }
}

#[repr(C, align(64))]
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash, Pod, Zeroable,
)]
pub struct Cell(pub [u8; 64]);
impl Cell {
  #[inline]
  pub fn non_air_bits(&self) -> u64 {
    u64::from_be_bytes(std::array::from_fn(|i| {
      let head = (i % 8) * 8;
      let r = ((self.0[head + 0] != 0) as u8) << 7
        | ((self.0[head + 1] != 0) as u8) << 6
        | ((self.0[head + 2] != 0) as u8) << 5
        | ((self.0[head + 3] != 0) as u8) << 4
        | ((self.0[head + 4] != 0) as u8) << 3
        | ((self.0[head + 5] != 0) as u8) << 2
        | ((self.0[head + 6] != 0) as u8) << 1
        | ((self.0[head + 7] != 0) as u8) << 0;
      r
    }))
  }
}

#[repr(C, align(16))]
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash, Pod, Zeroable,
)]
pub struct CellHalo(pub [u8; 16]);
impl CellHalo {
  #[inline]
  pub fn make_halo_west(cell: &Cell) -> Self {
    Self(std::array::from_fn(|i| {
      cell.0[i * 4]
    }))
  }
  #[inline]
  pub fn make_halo_east(cell: &Cell) -> Self {
    Self(std::array::from_fn(|i| {
      cell.0[i * 4 + 3]
    }))
  }
  #[inline]
  pub fn make_halo_south(cell: &Cell) -> Self {
    Self(std::array::from_fn(|i| {
      let broad = (i & !3) * 4;
      let narrow = i % 4;
      cell.0[broad + narrow]
    }))
  }
  #[inline]
  pub fn make_halo_north(cell: &Cell) -> Self {
    Self(std::array::from_fn(|i| {
      let broad = (i & !3) * 4;
      let narrow = i % 4;
      cell.0[broad + narrow + 12]
    }))
  }
  #[inline]
  pub fn make_halo_bottom(cell: &Cell) -> Self {
    Self(std::array::from_fn(|i| {
      cell.0[i]
    }))
  }
  #[inline]
  pub fn make_halo_top(cell: &Cell) -> Self {
    Self(std::array::from_fn(|i| {
      cell.0[i + 48]
    }))
  }
}
