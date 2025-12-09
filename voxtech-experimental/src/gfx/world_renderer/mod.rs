use wgpu::{
  util::DeviceExt, Buffer, PipelineLayout,
  RenderPipeline,
};

pub mod block_rdr;
pub mod types;

/// VoxTechのWorld用描画構造体
pub struct WorldRenderer {
  _pipeline_layout: PipelineLayout,
  pipeline: RenderPipeline,
  vertices: [Buffer; 6],
  indices: Buffer,
  camera: super::camera::CameraUniformInstance,
}
impl WorldRenderer {
  pub fn new(
    context: &super::WGPUContext,
    camera: &super::camera::CameraInstance,
  ) -> crate::StdResult<Self> {
    let camera =
      super::camera::CameraUniformInstance::new(
        context, camera,
      );
    let pipeline_layout = context
      .device
      .create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
          label: Some("World render pipeline Layout"),
          bind_group_layouts: &[
            &camera.bindgroup_layout
          ],
          push_constant_ranges: &[],
        },
      );
    let shader = context
      .device
      .create_shader_module(
        wgpu::ShaderModuleDescriptor {
          label: Some("World render shader module"),
          source: wgpu::ShaderSource::Wgsl(
            include_str!("world_renderer.wgsl").into(),
          ),
        },
      );
    let vertices = std::array::from_fn(|i| {
      context
        .device
        .create_buffer_init(
          &wgpu::util::BufferInitDescriptor {
            label: Some(&format!(
              "Block tile vertices buffer[{dir}]",
              dir = types::TileFace::from(i as u8)
            )),
            contents: bytemuck::cast_slice(
              &types::TILE_VERTICES[i],
            ),
            usage: wgpu::BufferUsages::VERTEX,
          },
        )
    });
    let indices = context
      .device
      .create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: Some("Block tile indices buffer"),
          contents: bytemuck::cast_slice(
            types::TILE_INDICES,
          ),
          usage: wgpu::BufferUsages::INDEX,
        },
      );
    let pipeline = context
      .device
      .create_render_pipeline(
        &wgpu::RenderPipelineDescriptor {
          label: Some("World render pipeline"),
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
          depth_stencil: None,
          multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
          },
          multiview: None,
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
              format: context.config.format,
              blend: Some(
                wgpu::BlendState::ALPHA_BLENDING,
              ),
              write_mask: wgpu::ColorWrites::ALL,
            })],
          }),
          cache: None,
        },
      );

    Ok(Self {
      _pipeline_layout: pipeline_layout,
      pipeline,
      vertices,
      indices,
      camera,
    })
  }
  pub fn update_camera(
    &mut self,
    context: &super::WGPUContext,
    camera: &super::camera::CameraInstance,
  ) {
    self
      .camera
      .update(context, camera);
  }
  pub fn rendering(
    &self,
    view: &wgpu::TextureView,
    context: &super::WGPUContext,
    block_rdr_instance: &[block_rdr::BlockRenderInstance],
  ) {
    let mut encoder = context
      .device
      .create_command_encoder(
        &wgpu::CommandEncoderDescriptor {
          label: Some("World renderer command encoder"),
        },
      );
    {
      let mut render_pass = encoder.begin_render_pass(
        &wgpu::RenderPassDescriptor {
          label: Some("World renderer pass"),
          color_attachments: &[Some(
            wgpu::RenderPassColorAttachment {
              view,
              depth_slice: None,
              resolve_target: None,
              ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(
                  wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                  },
                ),
                store: wgpu::StoreOp::Store,
              },
            },
          )],
          depth_stencil_attachment: None,
          timestamp_writes: None,
          occlusion_query_set: None,
        },
      );
      render_pass.set_pipeline(&self.pipeline);
      render_pass.set_bind_group(
        0,
        &self.camera.bindgroup,
        &[],
      );
      render_pass.set_index_buffer(
        self.indices.slice(..),
        wgpu::IndexFormat::Uint16,
      );
      for i in 0..block_rdr_instance.len() {
        block_rdr_instance[i]
          .rendering(&mut render_pass);
        for j in 0..self.vertices.len() {
          render_pass.set_vertex_buffer(
            0,
            self.vertices[j].slice(..),
          );
          render_pass.draw_indexed(
            0..types::TILE_INDICES.len() as u32,
            0,
            0..block_rdr_instance[i]
              .instances
              .len() as u32,
          );
        }
      }
    }
    context
      .queue
      .submit(std::iter::once(
        encoder.finish(),
      ));
  }
}
