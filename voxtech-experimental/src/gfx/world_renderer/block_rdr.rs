//! Block-Renderer
//! ブロックレンダラ

use wgpu::{util::DeviceExt, Buffer};

pub struct BlockRenderInstance {
  pub instances: Vec<super::types::BakedInstance>,
  buffer: Buffer,
}
impl BlockRenderInstance {
  pub fn new(
    context: &super::super::WGPUContext,
  ) -> Self {
    let instances = (0..4096)
      .map(
        |i| super::types::BakedInstance {
          stride: i,
          tex_pos: [0., 0.],
          tex_scale: [0., 0.],
        },
      )
      .collect::<Vec<_>>();
    let buffer = context
      .device
      .create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: Some("Block render instances buffer"),
          contents: bytemuck::cast_slice(&instances),
          usage: wgpu::BufferUsages::VERTEX,
        },
      );
    Self { instances, buffer }
  }
  pub fn rendering(
    &self,
    render_pass: &mut wgpu::RenderPass,
  ) {
    render_pass
      .set_vertex_buffer(1, self.buffer.slice(..));
  }
}
