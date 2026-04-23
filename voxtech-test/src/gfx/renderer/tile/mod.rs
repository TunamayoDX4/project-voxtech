use crate::gfx::WGPUCtx;

use super::super::util::*;
use wgpu::{
  BindGroup, BindGroupLayout, Buffer, CommandEncoder,
  PipelineLayout, RenderPipeline, SurfaceTexture,
  util::DeviceExt,
};

pub mod vertex;

pub struct TileRdr {
  cube_vertex_buffer: [Buffer; 6],
  index_buffer: Buffer,
  instances: Vec<vertex::Instance>,
  instance_buffer: Buffer,

  pub depth: Texture,

  _pipeline_layout: PipelineLayout,
  pipeline: RenderPipeline,
}
impl TileRdr {
  pub fn new(
    gfx: &WGPUCtx,
    camera: &CameraBundle,
  ) -> Self {
    let cube_vertex_buffer: [_; 6] =
      vertex::CUBE_VERTICES.map(|vertices| {
        gfx.device.create_buffer_init(
          &wgpu::util::BufferInitDescriptor {
            label: Some("cube vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
          },
        )
      });
    let index_buffer = gfx.device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("index buffer"),
        contents: bytemuck::cast_slice(vertex::INDICES),
        usage: wgpu::BufferUsages::INDEX,
      },
    );
    let instances = Vec::new();
    let instance_buffer =
      gfx.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: Some("instance buffer"),
          contents: bytemuck::cast_slice(&instances),
          usage: wgpu::BufferUsages::VERTEX,
        },
      );

    let depth = Texture::create_depth_texture(
      &gfx.device,
      &gfx.config,
    );

    let pipeline_layout = gfx
      .device
      .create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
          label: Some("tile renderer pipeline layout"),
          bind_group_layouts: &[Some(
            &camera.bindgroup_layout,
          )],
          immediate_size: 0,
        },
      );
    let shader = gfx.device.create_shader_module(
      wgpu::ShaderModuleDescriptor {
        label: Some("tile renderer main shader"),
        source: wgpu::ShaderSource::Wgsl(
          include_str!("tile.wgsl").into(),
        ),
      },
    );
    let pipeline = gfx
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
              vertex::Instance::desc(),
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
              format: gfx.config.format,
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
      cube_vertex_buffer,
      index_buffer,
      instances,
      instance_buffer,
      depth,
      _pipeline_layout: pipeline_layout,
      pipeline,
    }
  }

  pub fn update_instances(
    &mut self,
    gfx: &WGPUCtx,
    f: impl FnOnce(&mut Vec<vertex::Instance>),
  ) {
    f(&mut self.instances);
    self.instance_buffer =
      gfx.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: Some("instance buffer"),
          contents: bytemuck::cast_slice(
            &self.instances,
          ),
          usage: wgpu::BufferUsages::VERTEX,
        },
      );
  }

  pub fn surface_resize(
    &mut self,
    gfx: &WGPUCtx,
    _new_size: winit::dpi::PhysicalSize<u32>,
  ) {
    self.depth = Texture::create_depth_texture(
      &gfx.device,
      &gfx.config,
    );
  }

  pub fn rendering(
    &self,
    surface_texture: &SurfaceTexture,
    enc: &mut CommandEncoder,
    camera: &CameraBundle,
  ) {
    if !self.instances.is_empty() {
      let surface_texture_view = surface_texture
        .texture
        .create_view(
          &wgpu::TextureViewDescriptor::default(),
        );
      let mut rpass = enc.begin_render_pass(
        &wgpu::RenderPassDescriptor {
          label: Some("tile render pass"),
          color_attachments: &[Some(
            wgpu::RenderPassColorAttachment {
              view: &surface_texture_view,
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
              view: &self.depth.view,
              depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
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
      rpass.set_vertex_buffer(
        1,
        self.instance_buffer.slice(..),
      );
      rpass.set_bind_group(0, &camera.bindgroup, &[]);
      for cvb in self.cube_vertex_buffer.iter() {
        rpass.set_vertex_buffer(0, cvb.slice(..));
        rpass.draw_indexed(
          0..vertex::INDICES.len() as _,
          0,
          0..self.instances.len() as _,
        );
      }
    }
  }
}
