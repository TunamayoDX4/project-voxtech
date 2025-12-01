//! # VoxTech-BLOCK-ARCH
//! VoxTechのブロックデータのアーキテクチャ
//!
//! 3軸構造でXは-西/+東・Yは-南/+北・Zは-下/+上で、
//! +方向のZ軸を起点とする。
//! ブロックデータは64分木を基本的なトポロジとする。

/// ブロックデータオブジェクト
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block {
  /// ブロックの種類識別子 実効15bit
  id: u16,

  /// 汎化ブロックタグ
  gen_tag: Option<u8>,

  /// システムタグ
  sys_tag: u8,

  /// ユーザタグ
  user_tag: Option<u8>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockLoID(u8);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockHiID(u8);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockSysTag(u8);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockUserTag(u8);

/// ブロックの種類データ
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockType {
  /// 空気ブロック(ID == 0)
  Air,

  /// 汎化ブロック(ID < 127 && ID != 0)
  GenericBlock,

  /// 通常ブロック(128 <= ID)
  NormalBlock,
}
