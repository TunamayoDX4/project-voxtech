use bytemuck::{Pod, Zeroable};

/// 頂点構造体
#[repr(C)]
#[derive(
  Debug, Clone, Copy, PartialEq, Pod, Zeroable,
)]
pub struct Vertex {
  pub position: [f32; 4],
  pub tex_coord: [f32; 2],
  pub color: [f32; 4],
}
impl Vertex {
  const ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x2, 2 => Float32x4];

  pub fn desc() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
      array_stride: std::mem::size_of::<Self>()
        as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &Self::ATTRIBS,
    }
  }
}

/// タイルの頂点配列
/// 面毎に反時計回りの頂点を4つ配置する。
pub const TILE_VERTICES: &[[Vertex; 4]] = &[
  // 西面(X-)
  [
    Vertex {
      position: [0., 1., 1., 1.],
      tex_coord: [0., 1.],
      color: [1., 0., 0., 1.],
    },
    Vertex {
      position: [0., 1., 0., 1.],
      tex_coord: [1., 1.],
      color: [1., 0., 0., 1.],
    },
    Vertex {
      position: [0., 0., 0., 1.],
      tex_coord: [1., 0.],
      color: [1., 0., 0., 1.],
    },
    Vertex {
      position: [0., 0., 1., 1.],
      tex_coord: [0., 0.],
      color: [1., 0., 0., 1.],
    },
  ],
  // 東面(X+)
  [
    Vertex {
      position: [1., 0., 1., 1.],
      tex_coord: [0., 1.],
      color: [0., 1., 1., 1.],
    },
    Vertex {
      position: [1., 0., 0., 1.],
      tex_coord: [1., 1.],
      color: [0., 1., 1., 1.],
    },
    Vertex {
      position: [1., 1., 0., 1.],
      tex_coord: [1., 0.],
      color: [0., 1., 1., 1.],
    },
    Vertex {
      position: [1., 1., 1., 1.],
      tex_coord: [0., 0.],
      color: [0., 1., 1., 1.],
    },
  ],
  // 南面(Y-)
  [
    Vertex {
      position: [0., 0., 1., 1.],
      tex_coord: [0., 1.],
      color: [0., 1., 0., 1.],
    },
    Vertex {
      position: [0., 0., 0., 1.],
      tex_coord: [1., 1.],
      color: [0., 1., 0., 1.],
    },
    Vertex {
      position: [1., 0., 0., 1.],
      tex_coord: [1., 0.],
      color: [0., 1., 0., 1.],
    },
    Vertex {
      position: [1., 0., 1., 1.],
      tex_coord: [0., 0.],
      color: [0., 1., 0., 1.],
    },
  ],
  // 北面(Y+)
  [
    Vertex {
      position: [1., 1., 1., 1.],
      tex_coord: [0., 1.],
      color: [1., 0., 1., 1.],
    },
    Vertex {
      position: [1., 1., 0., 1.],
      tex_coord: [1., 1.],
      color: [1., 0., 1., 1.],
    },
    Vertex {
      position: [0., 1., 0., 1.],
      tex_coord: [1., 0.],
      color: [1., 0., 1., 1.],
    },
    Vertex {
      position: [0., 1., 1., 1.],
      tex_coord: [0., 0.],
      color: [1., 0., 1., 1.],
    },
  ],
  // 下面(Z-)
  [
    Vertex {
      position: [0., 0., 0., 1.],
      tex_coord: [0., 1.],
      color: [0., 0., 1., 1.],
    },
    Vertex {
      position: [0., 1., 0., 1.],
      tex_coord: [1., 1.],
      color: [0., 0., 1., 1.],
    },
    Vertex {
      position: [1., 1., 0., 1.],
      tex_coord: [1., 0.],
      color: [0., 0., 1., 1.],
    },
    Vertex {
      position: [1., 0., 0., 1.],
      tex_coord: [0., 0.],
      color: [0., 0., 1., 1.],
    },
  ],
  // 上面(Z+)
  [
    Vertex {
      position: [0., 1., 1., 1.],
      tex_coord: [0., 1.],
      color: [1., 1., 0., 1.],
    },
    Vertex {
      position: [0., 0., 1., 1.],
      tex_coord: [1., 1.],
      color: [1., 1., 0., 1.],
    },
    Vertex {
      position: [1., 0., 1., 1.],
      tex_coord: [1., 0.],
      color: [1., 1., 0., 1.],
    },
    Vertex {
      position: [1., 1., 1., 1.],
      tex_coord: [0., 0.],
      color: [1., 1., 0., 1.],
    },
  ],
];

/// タイルの頂点インデックスバッファ
/// 2ポリゴンでタイルを描画する
pub const TILE_INDICES: &[u16] = &[
  0, 1, 2, // 1ポリゴン目
  0, 2, 3, // 2ポリゴン目
];

/// ブロックのタイルが向いている面
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileFace {
  /// 西(X-)
  WST = 0,
  /// 東(X+)
  EST = 1,
  /// 南(Y-)
  STH = 2,
  /// 北(Y+)
  NTH = 3,
  /// 下(Z-)
  BTM = 4,
  /// 上(Z+)
  TOP = 5,
  /// 未定義
  UNDEF = u8::MAX,
}
impl std::fmt::Display for TileFace {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.write_str(match self {
      TileFace::WST => "West",
      TileFace::EST => "East",
      TileFace::STH => "South",
      TileFace::NTH => "North",
      TileFace::BTM => "Bottom",
      TileFace::TOP => "North",
      TileFace::UNDEF => "Undefined",
    })
  }
}
impl From<u8> for TileFace {
  fn from(value: u8) -> Self {
    match value {
      0 => Self::WST,
      1 => Self::EST,
      2 => Self::STH,
      3 => Self::NTH,
      4 => Self::BTM,
      5 => Self::TOP,
      _ => Self::UNDEF,
    }
  }
}

/// インスタンス構造体
#[repr(C)]
#[derive(
  Debug, Clone, Copy, PartialEq, Pod, Zeroable,
)]
pub struct BakedInstance {
  /// ブロックのストライド
  /// 下位12bitのみを使用、
  pub stride: u32,
  pub tex_pos: [f32; 2],
  pub tex_scale: [f32; 2],
}
impl BakedInstance {
  const ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
    8 => Uint32,
    9 => Float32x2,
    10 => Float32x2,
  ];

  pub fn desc() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
      array_stride: std::mem::size_of::<Self>()
        as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Instance,
      attributes: &Self::ATTRIBS,
    }
  }
}
