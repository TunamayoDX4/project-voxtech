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

  /// 64分木の子要素を西方向(-X)へローテート
  #[inline]
  pub fn rotate_west(&self) -> Self {
    Self(std::array::from_fn(|i| {
      let no_axis = i & !3;
      let axis = (i + 1) & 3;
      self.0[no_axis | axis]
    }))
  }

  /// 64分木の子要素を西方向(-X)へストライド
  #[inline]
  pub fn stride_west(&self) -> Self {
    let mut rotated = self.rotate_west();
    // 不要な範囲外に追い出されたブロックを消去する
    for i in 0..16 {
      rotated.0[i * 4 + 3] = 0;
    }
    rotated
  }

  /// 64分木の子要素を西方向(-X)へストライド(隣セルの考慮)
  #[inline]
  pub fn stride_west_neigh(
    &self,
    neigh: &CellHalo,
  ) -> Self {
    let mut rotated = self.rotate_west();
    // 不要な範囲外に追い出されたブロックをお隣にする
    for i in 0..16 {
      rotated.0[i * 4 + 3] = neigh.0[i];
    }
    rotated
  }

  /// 64分木の子要素を東方向(+X)へローテート
  #[inline]
  pub fn rotate_east(&self) -> Self {
    Self(std::array::from_fn(|i| {
      let no_axis = i & !3;
      let axis = (i + 3) & 3;
      self.0[no_axis + axis]
    }))
  }

  /// 64分木の子要素を東方向(+X)へストライド
  #[inline]
  pub fn stride_east(&self) -> Self {
    let mut rotated = self.rotate_east();
    // 不要な範囲外に追い出されたブロックを消去する
    for i in 0..16 {
      rotated.0[i * 4] = 0;
    }
    rotated
  }

  /// 64分木の子要素を東方向(+X)へストライド(隣セルの考慮)
  #[inline]
  pub fn stride_east_neigh(
    &self,
    neigh: &CellHalo,
  ) -> Self {
    let mut rotated = self.rotate_east();
    // 不要な範囲外に追い出されたブロックをお隣にする
    for i in 0..16 {
      rotated.0[i * 4] = neigh.0[i];
    }
    rotated
  }

  /// 64分木の子要素を南方向(-Y)へローテート
  #[inline]
  pub fn rotate_south(&self) -> Self {
    Self(std::array::from_fn(|i| {
      let no_axis = i & !12;
      let axis = (i + 4) & 12;
      self.0[no_axis + axis]
    }))
  }

  /// 64分木の子要素を南方向(-Y)へストライド
  #[inline]
  pub fn stride_south(&self) -> Self {
    let mut rotated = self.rotate_south();
    // 不要な範囲外に追い出されたブロックを消去する
    for i in 0..16 {
      let broad = (i & !3) * 4;
      let narrow = i % 4;
      rotated.0[broad + narrow + 12] = 0;
    }
    rotated
  }

  /// 64分木の子要素を南方向(-Y)へストライド(隣セルの考慮)
  #[inline]
  pub fn stride_south_neigh(
    &self,
    neigh: &CellHalo,
  ) -> Self {
    let mut rotated = self.rotate_south();
    // 不要な範囲外に追い出されたブロックをお隣にする
    for i in 0..16 {
      let broad = (i & !3) * 4;
      let narrow = i % 4;
      rotated.0[broad + narrow + 12] = neigh.0[i];
    }
    rotated
  }

  /// 64分木の子要素を北方向(+Y)へローテート
  #[inline]
  pub fn rotate_north(&self) -> Self {
    Self(std::array::from_fn(|i| {
      let no_axis = i & !12;
      let axis = (i + 12) & 12;
      self.0[no_axis + axis]
    }))
  }

  /// 64分木の子要素を北方向(+Y)へストライド
  #[inline]
  pub fn stride_north(&self) -> Self {
    let mut rotated = self.rotate_north();
    // 不要な範囲外に追い出されたブロックを消去する
    for i in 0..16 {
      let broad = (i & !3) * 4;
      let narrow = i % 4;
      rotated.0[broad + narrow] = 0;
    }
    rotated
  }

  /// 64分木の子要素を北方向(+Y)へストライド(隣セルの考慮)
  #[inline]
  pub fn stride_north_neigh(
    &self,
    neigh: &CellHalo,
  ) -> Self {
    let mut rotated = self.rotate_north();
    // 不要な範囲外に追い出されたブロックをお隣にする
    for i in 0..16 {
      let broad = (i & !3) * 4;
      let narrow = i % 4;
      rotated.0[broad + narrow] = neigh.0[i];
    }
    rotated
  }

  /// 64分木の子要素を下方向(-Z)へローテート
  #[inline]
  pub fn rotate_bottom(&self) -> Self {
    Self(std::array::from_fn(|i| {
      let no_axis = i & !48;
      let axis = (i + 16) & 48;
      self.0[no_axis + axis]
    }))
  }

  /// 64分木の子要素を下方向(-Z)へストライド
  #[inline]
  pub fn stride_bottom(&self) -> Self {
    let mut rotated = self.rotate_bottom();
    // 不要な範囲外に追い出されたブロックを消去する
    for i in 0..16 {
      rotated.0[i + 48] = 0;
    }
    rotated
  }

  /// 64分木の子要素を北方向(+Y)へストライド(隣セルの考慮)
  #[inline]
  pub fn stride_bottom_neigh(
    &self,
    neigh: &CellHalo,
  ) -> Self {
    let mut rotated = self.rotate_bottom();
    // 不要な範囲外に追い出されたブロックをお隣にする
    for i in 0..16 {
      rotated.0[i + 48] = neigh.0[i];
    }
    rotated
  }

  /// 64分木の子要素を上方向(+Z)へローテート
  #[inline]
  pub fn rotate_top(&self) -> Self {
    Self(std::array::from_fn(|i| {
      let no_axis = i & !48;
      let axis = (i + 48) & 48;
      self.0[no_axis + axis]
    }))
  }

  /// 64分木の子要素を上方向(+Z)へストライド
  #[inline]
  pub fn stride_top(&self) -> Self {
    let mut rotated = self.rotate_top();
    // 不要な範囲外に追い出されたブロックを消去する
    for i in 0..16 {
      rotated.0[i] = 0;
    }
    rotated
  }

  /// 64分木の子要素を北方向(+Y)へストライド(隣セルの考慮)
  #[inline]
  pub fn stride_top_neigh(
    &self,
    neigh: &CellHalo,
  ) -> Self {
    let mut rotated = self.rotate_top();
    // 不要な範囲外に追い出されたブロックをお隣にする
    for i in 0..16 {
      rotated.0[i] = neigh.0[i];
    }
    rotated
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
