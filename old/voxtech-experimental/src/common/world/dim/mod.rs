use super::l3_region::L3Region;

/// # Dimension Key
/// 次元のキー
#[repr(C, align(32))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DimKey([u8; 32]);

/// # Dimension
/// 次元
pub struct Dim {
  regions: Vec<Option<L3Region>>,
}
