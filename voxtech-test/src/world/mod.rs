use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorldInitError {
  BadWorldName,
}
impl std::fmt::Display for WorldInitError {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      WorldInitError::BadWorldName => {
        f.write_str("Bad world name inputted")
      }
    }
  }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlkIDLo(u8);

/// ブロックの下位バイトを格納するCell構造体
/// 4メートル立方単位のセル、64要素
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellBlkIDLo(pub [BlkIDLo; 64]);
impl CellBlkIDLo {
  pub fn get(
    &self,
    pos: &crate::types::BlockPos,
  ) -> Option<&BlkIDLo> {
    self
      .0
      .get(pos.as_64tree_index() as usize)
  }
  pub fn get_mut(
    &mut self,
    pos: &crate::types::BlockPos,
  ) -> Option<&mut BlkIDLo> {
    self
      .0
      .get_mut(pos.as_64tree_index() as usize)
  }
  pub fn iter(&self) -> impl Iterator<Item = &BlkIDLo> {
    self.0.iter()
  }
  pub fn iter_mut(
    &mut self,
  ) -> impl Iterator<Item = &mut BlkIDLo> {
    self.0.iter_mut()
  }
}
impl Index<crate::types::BlockPos> for CellBlkIDLo {
  type Output = BlkIDLo;

  fn index(
    &self,
    index: crate::types::BlockPos,
  ) -> &Self::Output {
    &self.0[index.as_64tree_index() as usize]
  }
}
impl IndexMut<crate::types::BlockPos> for CellBlkIDLo {
  fn index_mut(
    &mut self,
    index: crate::types::BlockPos,
  ) -> &mut Self::Output {
    &mut self.0[index.as_64tree_index() as usize]
  }
}

/// ブロックの下位バイトを格納するChunk構造体
/// 16メートル立方単位のチャンク、4Ki要素(=64^2要素)
#[repr(C, align(64))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChunkBlkIDLo(pub [CellBlkIDLo; 64]);
impl ChunkBlkIDLo {}

/// 16メートル立方単位のチャンク、4Ki要素(=64^2要素)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Chunk {
  blk_id_lo: Option<Box<ChunkBlkIDLo>>,
}

/// 64メートル立方単位のセクタ、256Ki要素(=64^3要素)
pub struct Sector {
  chunks: [Chunk; 64],
}

/// 空間を表すディメンション、セクタを束ねて世界を表現する
pub struct Dimension {
  sectors: Vec<Option<Sector>>,
}

pub struct World {
  name: String,
}
impl World {
  /// ワールドそのものの初期化
  pub fn initialize(
    name: &str,
  ) -> Result<Self, WorldInitError> {
    // ワールド名の正常性チェック
    if !name
      .chars()
      .fold(true, |cond, char| {
        cond
          && (char.is_alphanumeric()
            || char == '-'
            || char == '_'
            || char == ' ')
      })
    {
      return Err(WorldInitError::BadWorldName);
    }

    Ok(Self {
      name: name.to_string(),
    })
  }
}
