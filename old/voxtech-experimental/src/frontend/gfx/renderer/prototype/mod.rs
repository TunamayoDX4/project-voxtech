use std::sync::Arc;

use parking_lot::Mutex;
use wgpu::{PipelineLayout, RenderPipeline};

use super::{
  super::wgpu_ctx::WGPUCtx,
  util::{camera::*, texture::*},
};

pub struct PrototypeRenderer {
  pipeline_layout: PipelineLayout,
  pipeline: RenderPipeline,
  camera_config: Camera3DConfig,
  camera_instance: Arc<Mutex<Camera3DInstance>>,
  camera: Arc<Mutex<Camera3DUniformInstance>>,
  camera_update_recv: crossbeam::channel::Receiver<(
    [f64; 2],
    nalgebra::Point3<f64>,
  )>,
}
impl PrototypeRenderer {
  pub fn new(
    ctx: &WGPUCtx,
    camera_update_recv: crossbeam::channel::Receiver<(
      [f64; 2],
      nalgebra::Point3<f64>,
    )>,
    depth: &DepthTexture,
  ) -> Self {
    let depth =
      DepthTexture::new_depth(ctx, "depth texture");
    let camera_config = Camera3DConfig {
      fovy: 45. * (std::f64::consts::PI / 180.),
      near: 0.1,
      far: 5000.,
    };
    let camera_instance = Camera3DInstance {
      position: [0., -10., 0.].into(),
      rotation:
        nalgebra::UnitQuaternion::from_axis_angle(
          &nalgebra::UnitVector3::new_normalize(
            nalgebra::Vector3::y(),
          ),
          0.0,
        ),
    };
    let camera = Camera3DUniformInstance::new(
      ctx,
      &camera_config,
      &camera_instance,
    );
    let pipeline_layout = ctx
      .device
      .create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
          label: Some("Prototype Renderer layout"),
          bind_group_layouts: &[
            &camera.bindgroup_layout
          ],
          immediate_size: 0,
        },
      );
    let shader = ctx.device.create_shader_module(
      wgpu::include_wgsl!("prototype.wgsl"),
    );
    let pipeline = ctx.device.create_mesh_pipeline(
      &wgpu::MeshPipelineDescriptor {
        label: Some("Prototype Renderer"),
        layout: Some(&pipeline_layout),
        task: Some(wgpu::TaskState {
          module: &shader,
          entry_point: Some("task_main"),
          compilation_options: Default::default(),
        }),
        mesh: wgpu::MeshState {
          module: &shader,
          entry_point: Some("mesh_main"),
          compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
          module: &shader,
          entry_point: Some("frag_main"),
          compilation_options: Default::default(),
          targets: &[Some(
            ctx.config.lock().format.into(),
          )],
        }),
        primitive: wgpu::PrimitiveState {
          cull_mode: Some(wgpu::Face::Back),
          topology:
            wgpu::PrimitiveTopology::TriangleList,
          strip_index_format: None,
          front_face: wgpu::FrontFace::Ccw,
          unclipped_depth: false,
          polygon_mode: wgpu::PolygonMode::Fill,
          conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
          format: DepthTexture::DEPTH_FORMAT,
          depth_write_enabled: true,
          depth_compare: wgpu::CompareFunction::Less,
          stencil: wgpu::StencilState::default(),
          bias: wgpu::DepthBiasState::default(),
        }),
        multisample: Default::default(),
        multiview: None,
        cache: None,
      },
    );

    Self {
      pipeline_layout,
      pipeline,
      camera: Arc::new(Mutex::new(camera)),
      camera_config,
      camera_instance: Arc::new(Mutex::new(
        camera_instance,
      )),
      camera_update_recv,
    }
  }

  pub fn update(&self, context: &WGPUCtx) {
    let mut camera_instance_lock =
      self.camera_instance.lock();
    let mut camera_lock = self.camera.lock();
    while let Ok((rot, pos)) = self
      .camera_update_recv
      .try_recv()
    {
      camera_instance_lock.rotation =
        nalgebra::UnitQuaternion::from_axis_angle(
          &nalgebra::UnitVector3::new_normalize(
            nalgebra::Vector3::z(),
          ),
          rot[0],
        ) * nalgebra::UnitQuaternion::from_axis_angle(
          &nalgebra::UnitVector3::new_normalize(
            nalgebra::Vector3::x(),
          ),
          rot[1],
        );
      camera_instance_lock.position = pos;
    }
    camera_lock.update(
      context,
      &self.camera_config,
      &camera_instance_lock,
    );
  }

  pub fn rendering(
    &mut self,
    enc: &mut wgpu::CommandEncoder,
    target: &super::super::wgpu_ctx::RenderTarget,
    depth: &DepthTexture,
  ) {
    let mut rpass = enc.begin_render_pass(
      &wgpu::RenderPassDescriptor {
        label: Some("world renderer main render pass"),
        color_attachments: &[Some(
          wgpu::RenderPassColorAttachment {
            view: &target.view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
              load: wgpu::LoadOp::Clear(wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
              }),
              store: wgpu::StoreOp::Store,
            },
          },
        )],
        depth_stencil_attachment: Some(
          wgpu::RenderPassDepthStencilAttachment {
            view: &depth.texture.view,
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
    self
      .camera
      .lock()
      .rendering(&mut rpass);
    rpass.set_pipeline(&self.pipeline);
    rpass.draw_mesh_tasks(1, 1, 1);
  }
}
