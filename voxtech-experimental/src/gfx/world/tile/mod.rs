//! Tile描画用のレンダラ

use std::num::NonZero;

use wgpu::{
  util::DeviceExt, Buffer, BufferUsages,
  PipelineLayout, RenderPass, RenderPipeline,
};

use crate::common::Dir;

use super::{camera3d, RenderTarget, Texture, WGPUCtx};

pub mod types;

pub struct OpaqueTileInstances {
  instance: Vec<types::BakedInstance>,
  buffer: Option<Buffer>,
}
impl OpaqueTileInstances {
  pub fn new(
    ctx: &WGPUCtx,
    instance: Vec<types::BakedInstance>,
  ) -> Self {
    let buffer = ctx.device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("Opaque tile instances array"),
        contents: bytemuck::cast_slice(
          instance.as_slice(),
        ),
        usage: wgpu::BufferUsages::VERTEX,
      },
    );
    Self {
      instance,
      buffer: Some(buffer),
    }
  }

  pub fn new_empty(capacity: usize) -> Self {
    let instance = Vec::with_capacity(capacity);
    Self {
      instance,
      buffer: None,
    }
  }

  pub fn write_with<R>(
    &mut self,
    f: impl FnOnce(&mut Vec<types::BakedInstance>) -> R,
  ) -> R {
    self.instance.clear();
    f(&mut self.instance)
  }

  pub fn write(
    &mut self,
    instance: impl Iterator<Item = types::BakedInstance>,
  ) {
    self.instance.clear();
    instance.for_each(|i| self.instance.push(i));
  }

  pub fn update(&mut self, ctx: &WGPUCtx) {
    self.buffer = if self.instance.len() != 0 {
      Some(ctx.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: Some("Opaque tile instances array"),
          contents: bytemuck::cast_slice(
            self.instance.as_slice(),
          ),
          usage: wgpu::BufferUsages::VERTEX,
        },
      ))
    } else {
      None
    };
  }

  pub fn rendering(
    &self,
    rpass: &mut RenderPass,
  ) -> Option<NonZero<u32>> {
    if let Some(buffer) = self.buffer.as_ref() {
      rpass.set_vertex_buffer(1, buffer.slice(..));
      NonZero::new(self.instance.len() as _)
    } else {
      None
    }
  }
}

pub struct OpaqueTileRdr {
  _rpipe_layout: PipelineLayout,
  rpipe: RenderPipeline,
}
impl OpaqueTileRdr {
  pub fn new(
    ctx: &WGPUCtx,
    camera: &super::camera3d::Camera3DUniformInstance,
    chunk_uniform: &super::chunk::uniform::ChunkUniformLayout,
  ) -> Self {
    let rpipe_layout = ctx
      .device
      .create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
          label: Some(
            "Opaque tile render pipeline layout",
          ),
          bind_group_layouts: &[
            &camera.bindgroup_layout,
            &chunk_uniform.bindgroup_layout,
          ],
          immediate_size: 0,
        },
      );
    let shader = ctx.device.create_shader_module(
      wgpu::ShaderModuleDescriptor {
        label: Some("Opaque tile shader"),
        source: wgpu::ShaderSource::Wgsl(
          include_str!("tile_rdr.wgsl").into(),
        ),
      },
    );
    let rpipe = ctx
      .device
      .create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
          label: Some("Opaque tile render pipeline"),
          layout: Some(&rpipe_layout),
          primitive: wgpu::PrimitiveState {
            topology:
              wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
          },
          depth_stencil: Some(
            wgpu::DepthStencilState {
              format: super::Texture::DEPTH_FORMAT,
              depth_write_enabled: true,
              depth_compare:
                wgpu::CompareFunction::Less,
              stencil: wgpu::StencilState::default(),
              bias: wgpu::DepthBiasState::default(),
            },
          ),
          multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
          },
          cache: None,
          vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            compilation_options:
              wgpu::PipelineCompilationOptions {
                constants: &[],
                zero_initialize_workgroup_memory: false,
              },
            buffers: &[
              types::Vertex::desc(),
              types::BakedInstance::desc(),
            ],
          },
          fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options:
              wgpu::PipelineCompilationOptions {
                constants: &[],
                zero_initialize_workgroup_memory: false,
              },
            targets: &[Some(wgpu::ColorTargetState {
              format: ctx.config.format,
              blend: Some(wgpu::BlendState::REPLACE),
              write_mask: wgpu::ColorWrites::ALL,
            })],
          }),
          multiview_mask: None,
        },
      );

    Self {
      _rpipe_layout: rpipe_layout,
      rpipe,
    }
  }

  pub fn rendering(
    &mut self,
    render_target: &RenderTarget,
    depth_texture: &Texture,
    camera: &camera3d::Camera3DUniformInstance,
    tile: &TileShared,
    chunk: &super::chunk::ChunkStorage,
  ) {
    let mut encoder = render_target
      .ctx
      .device
      .create_command_encoder(
        &wgpu::CommandEncoderDescriptor {
          label: Some("World renderer command encoder"),
        },
      );
    {
      let mut rpass = encoder.begin_render_pass(
        &wgpu::RenderPassDescriptor {
          label: Some("World renderer pass"),
          color_attachments: &[Some(
            wgpu::RenderPassColorAttachment {
              view: &render_target.view,
              depth_slice: None,
              resolve_target: None,
              ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
              },
            },
          )],
          depth_stencil_attachment: Some(
            wgpu::RenderPassDepthStencilAttachment {
              view: &depth_texture.view,
              depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
              }),
              stencil_ops: None,
            },
          ),
          timestamp_writes: None,
          occlusion_query_set: None,
          multiview_mask: None,
        },
      );
      rpass.set_pipeline(&self.rpipe);
      rpass.set_bind_group(0, &camera.bindgroup, &[]);
      tile.set_index(&mut rpass);
      for dir in Dir::iter() {
        tile.set_vertex(&mut rpass, dir);
        chunk.rendering(&mut rpass, dir);
      }
    }
    render_target
      .ctx
      .queue
      .submit([encoder.finish()]);
  }
}

pub struct TileShared {
  vertices: [Buffer; Dir::COUNT as usize],
  indices: Buffer,
}
impl TileShared {
  pub fn new(ctx: &WGPUCtx) -> Self {
    let vertices = std::array::from_fn(|i| {
      ctx.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: Some(&format!(
            "Tile vertex buffer[{0}]",
            Dir::from(i as u8)
          )),
          contents: bytemuck::cast_slice(
            &types::TILE_VERTICES[i],
          ),
          usage: BufferUsages::VERTEX,
        },
      )
    });
    let indices = ctx.device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("Tile index buffer"),
        contents: bytemuck::cast_slice(
          types::TILE_INDICES,
        ),
        usage: BufferUsages::INDEX,
      },
    );
    Self { vertices, indices }
  }

  pub fn vertices(&self) -> &[Buffer; 6] {
    &self.vertices
  }
  pub fn indices(&self) -> &Buffer {
    &self.indices
  }

  #[inline]
  pub fn set_index(&self, rpass: &mut RenderPass) {
    rpass.set_index_buffer(
      self.indices.slice(..),
      wgpu::IndexFormat::Uint16,
    );
  }

  #[inline]
  pub fn set_vertex(
    &self,
    rpass: &mut RenderPass,
    dir: Dir,
  ) {
    rpass.set_vertex_buffer(
      0,
      self.vertices[dir as usize].slice(..),
    )
  }
}
