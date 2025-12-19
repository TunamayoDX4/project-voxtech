//! Tile描画用のレンダラ

use wgpu::{
  util::DeviceExt, Buffer, BufferUsages,
  PipelineLayout, RenderPipeline,
};

use crate::common::Dir;

use super::WGPUCtx;

pub mod types;

pub struct OpaqueTileInstances {
  instance: Vec<types::BakedInstance>,
  buffer: Buffer,
}
impl OpaqueTileInstances {
  pub fn new(
    ctx: &WGPUCtx,
    instance: impl Iterator<Item = types::BakedInstance>,
  ) -> Self {
    let instance = instance.collect::<Vec<_>>();
    let buffer = ctx.device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("Opaque tile instances array"),
        contents: bytemuck::cast_slice(
          instance.as_slice(),
        ),
        usage: wgpu::BufferUsages::VERTEX,
      },
    );
    Self { instance, buffer }
  }

  pub fn write(
    &mut self,
    instance: impl Iterator<Item = types::BakedInstance>,
  ) {
    self.instance.clear();
    instance.for_each(|i| self.instance.push(i));
  }

  pub fn update(&mut self, ctx: &WGPUCtx) {
    self.buffer = ctx.device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("Opaque tile instances array"),
        contents: bytemuck::cast_slice(
          self.instance.as_slice(),
        ),
        usage: wgpu::BufferUsages::VERTEX,
      },
    )
  }
}

pub struct OpaqueTileRdr {
  _rpipe_layout: PipelineLayout,
  rpipe: RenderPipeline,
}
impl OpaqueTileRdr {
  pub fn new(
    ctx: &WGPUCtx,
    tile: &TileShared,
    camera: &super::camera3d::Camera3DUniformInstance,
  ) -> Self {
    let rpipe_layout = ctx
      .device
      .create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
          label: Some(
            "Opaque tile render pipeline layout",
          ),
          bind_group_layouts: &[
            &camera.bindgroup_layout
          ],
          push_constant_ranges: &[],
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
            strip_index_format: Some(
              wgpu::IndexFormat::Uint16,
            ),
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
          multiview: None,
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
        },
      );

    Self {
      _rpipe_layout: rpipe_layout,
      rpipe,
    }
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
}
