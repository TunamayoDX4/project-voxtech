use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex([f32; 4]);
impl Vertex {
  pub const ATTRIB: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![
    0 => Float32x4,
  ];

  pub fn desc<'a>(
  ) -> wgpu::VertexBufferLayout<'a> {
    wgpu::VertexBufferLayout {
      array_stride: std::mem::size_of::<Self>()
        as _,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &Self::ATTRIB,
    }
  }
}

pub const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];
pub const CUBE_VERTICES: [[Vertex; 4]; 6] = [
  // EAST FACE(-X)
  [
    Vertex([0., 0., 0., 1.]),
    Vertex([0., 1., 0., 1.]),
    Vertex([0., 1., 1., 1.]),
    Vertex([0., 0., 1., 1.]),
  ],
  // WEST FACE(+X)
  [
    Vertex([1., 1., 0., 1.]),
    Vertex([1., 0., 0., 1.]),
    Vertex([1., 0., 1., 1.]),
    Vertex([1., 1., 1., 1.]),
  ],
  // SOUTH FACE(-Y)
  [
    Vertex([1., 0., 0., 1.]),
    Vertex([0., 0., 0., 1.]),
    Vertex([0., 0., 1., 1.]),
    Vertex([1., 0., 1., 1.]),
  ],
  // NORTH FACE(+Y)
  [
    Vertex([0., 1., 0., 1.]),
    Vertex([1., 1., 0., 1.]),
    Vertex([1., 1., 1., 1.]),
    Vertex([0., 1., 1., 1.]),
  ],
  // BOTTOM FACE(-Z)
  [
    Vertex([0., 0., 0., 1.]),
    Vertex([1., 0., 0., 1.]),
    Vertex([1., 1., 0., 1.]),
    Vertex([0., 1., 0., 1.]),
  ],
  // TOP FACE(+Z)
  [
    Vertex([1., 0., 1., 1.]),
    Vertex([0., 0., 1., 1.]),
    Vertex([0., 1., 1., 1.]),
    Vertex([1., 1., 1., 1.]),
  ],
];
