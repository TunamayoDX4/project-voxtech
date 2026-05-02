//! Tile renderer

use wgpu::{
  util::DeviceExt, Buffer, BufferUsages,
  CommandEncoder, PipelineLayout,
  RenderPipeline, ShaderSource, SurfaceTexture,
};

use crate::client::gfx::{
  util::*, wgpu_ctx::WGPUCtx,
};

pub mod tile_instance;
pub mod vertex;

pub struct TileRdr {
  vertex_buffer: [Buffer; 6],
  index_buffer: Buffer,
  pub instances:
    tile_instance::InstanceBufferArray,

  _pipeline_layout: PipelineLayout,
  pipeline: RenderPipeline,
}
impl TileRdr {
  pub fn rendering(
    &self,
    enc: &mut CommandEncoder,
    surface_texture: &SurfaceTexture,
    depth: &super::Texture,
    camera: &CameraBundle,
  ) {
    let view =
      surface_texture.texture.create_view(
        &wgpu::TextureViewDescriptor::default(),
      );
    let mut rpass = enc.begin_render_pass(
      &wgpu::RenderPassDescriptor {
        label: Some("tile render pass"),
        color_attachments: &[Some(
          wgpu::RenderPassColorAttachment {
            view: &view,
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
            view: &depth.view,
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
    rpass.set_pipeline(&self.pipeline);
    rpass.set_index_buffer(
      self.index_buffer.slice(..),
      wgpu::IndexFormat::Uint16,
    );
    rpass.set_bind_group(
      0,
      &camera.bindgroup,
      &[],
    );
    for entry in self.instances.iter() {
      rpass.set_vertex_buffer(
        1,
        entry.buffer().slice(..),
      );
      for cvb in self.vertex_buffer.iter() {
        rpass
          .set_vertex_buffer(0, cvb.slice(..));
        rpass.draw_indexed(
          0..vertex::INDICES.len() as _,
          0,
          0..entry.length(),
        )
      }
    }
  }

  pub fn new(
    wgpu_ctx: &WGPUCtx,
    camera: &CameraBundle,
  ) -> Self {
    let vertex_buffer: [_; 6] =
      std::array::from_fn(|i| {
        wgpu_ctx.device.create_buffer_init(
          &wgpu::util::BufferInitDescriptor {
            label: Some(&format!(
              "tile vertex buffer: [{i}]"
            )),
            contents: bytemuck::cast_slice(
              &vertex::CUBE_VERTICES[i],
            ),
            usage: BufferUsages::VERTEX,
          },
        )
      });
    let index_buffer =
      wgpu_ctx.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: Some("tile index buffer"),
          contents: bytemuck::cast_slice(
            vertex::INDICES,
          ),
          usage: BufferUsages::INDEX,
        },
      );
    let instances =
      tile_instance::InstanceBufferArray::default();
    let pipeline_layout =
      wgpu_ctx.device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
          label: Some(
            "tile renderer pipeline layout",
          ),
          bind_group_layouts: &[Some(
            &camera.bindgroup_layout,
          )],
          immediate_size: 0,
        },
      );
    let shader =
      wgpu_ctx.device.create_shader_module(
        wgpu::ShaderModuleDescriptor {
          label: Some(
            "tile renderer shader module",
          ),
          source: ShaderSource::Wgsl(
            include_str!("tile.wgsl").into(),
          ),
        },
      );
    let pipeline = wgpu_ctx
      .device
      .create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
          label: Some("tile renderer pipeline"),
          layout: Some(&pipeline_layout),
          primitive: wgpu::PrimitiveState {
            topology:
              wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
          },
          vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            compilation_options:
              wgpu::PipelineCompilationOptions {
                constants: &[],
                zero_initialize_workgroup_memory: false,
              },
            buffers: &[
              vertex::Vertex::desc(),
              tile_instance::Instance::desc(),
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
              format: wgpu_ctx.config.format,
              blend: Some(
                wgpu::BlendState::ALPHA_BLENDING,
              ),
              write_mask: wgpu::ColorWrites::ALL,
            })],
          }),
          depth_stencil: Some(
            wgpu::DepthStencilState {
              format: Texture::DEPTH_FORMAT,
              depth_write_enabled: Some(true),
              depth_compare: Some(
                wgpu::CompareFunction::LessEqual,
              ),
              stencil: wgpu::StencilState::default(),
              bias: wgpu::DepthBiasState::default(),
            },
          ),
          multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
          },
          multiview_mask: None,
          cache: None,
        },
      );
    Self {
      vertex_buffer,
      index_buffer,
      instances,
      _pipeline_layout: pipeline_layout,
      pipeline,
    }
  }
}
