//! チャンク描画関連のうちユニフォームバッファに関わるもの

use bytemuck::{Pod, Zeroable};
use wgpu::{
  util::DeviceExt, BindGroup, BindGroupLayout, Buffer,
};

use crate::common::BlockPos;

use super::WGPUCtx;

#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
pub struct ChunkUniform(pub [i32; 4]);
impl From<BlockPos> for ChunkUniform {
  fn from(value: BlockPos) -> Self {
    let v = value.cut_up(2);
    Self([
      *v.x() as i32,
      *v.y() as i32,
      *v.z() as i32,
      0,
    ])
  }
}

pub struct ChunkUniformInstance {
  buffer: Buffer,
  bindgroup: BindGroup,
  pub uniform: ChunkUniform,
}
impl ChunkUniformInstance {
  pub fn new(
    ctx: &WGPUCtx,
    layout: &ChunkUniformLayout,
    uniform: ChunkUniform,
  ) -> Self {
    let buffer = ctx.device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("Chunk uniform buffer"),
        contents: bytemuck::cast_slice(&uniform.0),
        usage: wgpu::BufferUsages::UNIFORM,
      },
    );
    let bindgroup = ctx.device.create_bind_group(
      &wgpu::BindGroupDescriptor {
        label: Some("Chunk uniform bindgroup"),
        layout: &layout.bindgroup_layout,
        entries: &[wgpu::BindGroupEntry {
          binding: 0,
          resource: buffer.as_entire_binding(),
        }],
      },
    );
    Self {
      buffer,
      bindgroup,
      uniform,
    }
  }

  pub fn update(&self, ctx: &WGPUCtx) {
    ctx.queue.write_buffer(
      &self.buffer,
      0,
      bytemuck::cast_slice(&self.uniform.0),
    );
  }

  pub fn rendering(
    &self,
    rpass: &mut wgpu::RenderPass,
  ) {
    rpass.set_bind_group(1, &self.bindgroup, &[]);
  }
}

pub struct ChunkUniformLayout {
  bindgroup_layout: BindGroupLayout,
}
impl ChunkUniformLayout {
  pub fn new(ctx: &WGPUCtx) -> Self {
    let bindgroup_layout = ctx
      .device
      .create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
          label: Some("Chunk uniform bindgroup layout"),
          entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
              ty: wgpu::BufferBindingType::Uniform,
              has_dynamic_offset: false,
              min_binding_size: None,
            },
            count: None,
          }],
        },
      );

    Self { bindgroup_layout }
  }
}
