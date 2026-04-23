use parking_lot::RwLock;

use crate::{
  common::{Dir, InnerBlockPos},
  gfx::world::chunk::ChunkStorageKey,
};

use super::l0_cell;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ChunkInfo {
  /// 不透明タイルが更新されているか？
  pub dirty_opq_tile: [bool; Dir::COUNT as usize],

  /// 可視性
  pub visibility: [bool; Dir::COUNT as usize],

  /// レンダラ・ストレージのキー
  pub rdr_storage_key: Option<ChunkStorageKey>,
}
impl Default for ChunkInfo {
  fn default() -> Self {
    Self {
      dirty_opq_tile: [false; Dir::COUNT as usize],
      visibility: [true; Dir::COUNT as usize],
      rdr_storage_key: None,
    }
  }
}

#[derive(Debug)]
pub struct Chunk {
  pub cell: Option<Box<[l0_cell::Cell; 64]>>,
}
impl Chunk {
  #[inline]
  pub fn chk_visible_face(
    sector_pos: InnerBlockPos,
    chunk_pos: InnerBlockPos,
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> [bool; Dir::COUNT as usize] {
    let stride = nalgebra::Vector3::new(
      (((sector_pos.0 >> 0) & 3) * 64
        | ((chunk_pos.0 >> 0) & 3) * 16) as f64,
      (((sector_pos.0 >> 2) & 3) * 64
        | ((chunk_pos.0 >> 2) & 3) * 16) as f64,
      (((sector_pos.0 >> 4) & 3) * 64
        | ((chunk_pos.0 >> 4) & 3) * 16) as f64,
    );
    [
      Self::chk_visible_west_face(
        stride,
        relative_camera_pos,
      ),
      Self::chk_visible_east_face(
        stride,
        relative_camera_pos,
      ),
      Self::chk_visible_south_face(
        stride,
        relative_camera_pos,
      ),
      Self::chk_visible_north_face(
        stride,
        relative_camera_pos,
      ),
      Self::chk_visible_bottom_face(
        stride,
        relative_camera_pos,
      ),
      Self::chk_visible_top_face(
        stride,
        relative_camera_pos,
      ),
    ]
  }

  /// 対象のリージョンの西面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_west_face(
    stride: nalgebra::Vector3<f64>,
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin =
      nalgebra::Point3::new(16., 8., 8.) + stride;
    let vp = relative_camera_pos - origin;
    vp.x <= f64::EPSILON
  }

  /// 対象のリージョンの東面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_east_face(
    stride: nalgebra::Vector3<f64>,
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin =
      nalgebra::Point3::new(0., 8., 8.) + stride;
    let vp = relative_camera_pos - origin;
    -f64::EPSILON <= vp.x
  }

  /// 対象のリージョンの南面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_south_face(
    stride: nalgebra::Vector3<f64>,
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin =
      nalgebra::Point3::new(8., 16., 8.) + stride;
    let vp = relative_camera_pos - origin;
    vp.y <= f64::EPSILON
  }

  /// 対象のリージョンの北面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_north_face(
    stride: nalgebra::Vector3<f64>,
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin =
      nalgebra::Point3::new(8., 0., 8.) + stride;
    let vp = relative_camera_pos - origin;
    -f64::EPSILON <= vp.y
  }

  /// 対象のリージョンの下面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_bottom_face(
    stride: nalgebra::Vector3<f64>,
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin =
      nalgebra::Point3::new(8., 8., 16.) + stride;
    let vp = relative_camera_pos - origin;
    vp.z <= f64::EPSILON
  }

  /// 対象のリージョンの上面を描画するかを判断する。
  #[inline]
  pub fn chk_visible_top_face(
    stride: nalgebra::Vector3<f64>,
    relative_camera_pos: &nalgebra::Point3<f64>,
  ) -> bool {
    let origin =
      nalgebra::Point3::new(8., 8., 0.) + stride;
    let vp = relative_camera_pos - origin;
    -f64::EPSILON <= vp.z
  }
}

#[derive(Debug, Clone)]
pub struct ChunkHalo {
  pub cell: Option<[l0_cell::CellHalo; 16]>,
}
impl Default for ChunkHalo {
  fn default() -> Self {
    Self { cell: None }
  }
}
impl ChunkHalo {
  #[inline]
  pub fn make_halo_west(chunk: &Chunk) -> Self {
    Self {
      cell: chunk.cell.as_ref().map(|c| {
        std::array::from_fn(|i| {
          l0_cell::CellHalo::make_halo_west(&c[i * 4])
        })
      }),
    }
  }
  #[inline]
  pub fn make_halo_east(chunk: &Chunk) -> Self {
    Self {
      cell: chunk.cell.as_ref().map(|c| {
        std::array::from_fn(|i| {
          l0_cell::CellHalo::make_halo_east(
            &c[i * 4 + 3],
          )
        })
      }),
    }
  }
  #[inline]
  pub fn make_halo_south(chunk: &Chunk) -> Self {
    Self {
      cell: chunk.cell.as_ref().map(|c| {
        std::array::from_fn(|i| {
          let broad = (i & !3) * 4;
          let narrow = i % 4;
          l0_cell::CellHalo::make_halo_south(
            &c[broad + narrow],
          )
        })
      }),
    }
  }
  #[inline]
  pub fn make_halo_north(chunk: &Chunk) -> Self {
    Self {
      cell: chunk.cell.as_ref().map(|c| {
        std::array::from_fn(|i| {
          let broad = (i & !3) * 4;
          let narrow = i % 4;
          l0_cell::CellHalo::make_halo_north(
            &c[broad + narrow + 12],
          )
        })
      }),
    }
  }
  #[inline]
  pub fn make_halo_bottom(chunk: &Chunk) -> Self {
    Self {
      cell: chunk.cell.as_ref().map(|c| {
        std::array::from_fn(|i| {
          l0_cell::CellHalo::make_halo_bottom(&c[i])
        })
      }),
    }
  }
  #[inline]
  pub fn make_halo_top(chunk: &Chunk) -> Self {
    Self {
      cell: chunk.cell.as_ref().map(|c| {
        std::array::from_fn(|i| {
          l0_cell::CellHalo::make_halo_top(&c[i + 48])
        })
      }),
    }
  }
}
