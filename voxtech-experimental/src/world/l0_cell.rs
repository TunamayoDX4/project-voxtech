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
    let non_air_bit =
      u64::from_be_bytes(std::array::from_fn(|i| {
        cell.non_air_bits(i)
      }));

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
  pub fn non_air_bits(&self, idx: usize) -> u8 {
    let head = (idx % 8) * 8;
    let r = ((self.0[head + 0] != 0) as u8) << 7
      | ((self.0[head + 1] != 0) as u8) << 6
      | ((self.0[head + 2] != 0) as u8) << 5
      | ((self.0[head + 3] != 0) as u8) << 4
      | ((self.0[head + 4] != 0) as u8) << 3
      | ((self.0[head + 5] != 0) as u8) << 2
      | ((self.0[head + 6] != 0) as u8) << 1
      | ((self.0[head + 7] != 0) as u8) << 0;
    r
  }
}

#[repr(C, align(16))]
#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash, Pod, Zeroable,
)]
pub struct CellHalo(pub [u8; 16]);
impl CellHalo {
  pub fn make_halo(
    cell: &Cell,
    face: crate::common::Dir,
  ) -> Self {
    let step = crate::common::Step::from(face);
    let neg = face.is_negative();
    Self(std::array::from_fn(|i| {
      cell.0[i * step as usize]
    }))
  }
}
