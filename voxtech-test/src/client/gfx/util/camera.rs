use super::WGPUCtx;
use bytemuck::{Pod, Zeroable};
use nalgebra::{
  matrix, Point3, UnitQuaternion,
};
use wgpu::{
  BindGroup, BindGroupLayout, Buffer,
};
use winit::dpi::PhysicalSize;

pub struct CameraBundle {
  pub param: Camera,
  buffer: Buffer,
  pub bindgroup_layout: BindGroupLayout,
  pub bindgroup: BindGroup,
}
impl CameraBundle {
  pub fn new(
    gfx: &WGPUCtx,
    param: Camera,
    bindgroup_binding_id: u32,
  ) -> Self {
    let buffer = gfx.device.create_buffer(
      &wgpu::BufferDescriptor {
        label: Some("camera bundle buffer"),
        size: std::mem::size_of::<CameraUniform>()
          as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::UNIFORM
          | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
      },
    );
    let bindgroup_layout = gfx
      .device
      .create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
          label: Some("camera bundle bindgroup layout"),
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
    let bindgroup =
      gfx.device.create_bind_group(
        &wgpu::BindGroupDescriptor {
          label: Some(
            "camera bundle bindgroup",
          ),
          layout: &bindgroup_layout,
          entries: &[wgpu::BindGroupEntry {
            binding: bindgroup_binding_id,
            resource: buffer
              .as_entire_binding(),
          }],
        },
      );

    Self {
      param,
      buffer,
      bindgroup_layout,
      bindgroup,
    }
  }

  pub fn update(
    &mut self,
    gfx: &WGPUCtx,
    position: &Point3<f32>,
    rotation: &UnitQuaternion<f32>,
  ) {
    let uniform = self.param.make_uniform(
      position,
      rotation,
      gfx.window.inner_size(),
    );
    gfx.queue.write_buffer(
      &self.buffer,
      0,
      bytemuck::cast_slice(&[uniform]),
    )
  }
}

#[repr(C)]
#[derive(
  Debug, Clone, Copy, Default, Pod, Zeroable,
)]
pub struct CameraUniform([[f32; 4]; 4]);

#[derive(Debug, Clone, Copy)]
pub struct Camera {
  pub fovy: f32,
  pub znear: f32,
  pub zfar: f32,
}
impl Camera {
  pub fn make_uniform(
    &mut self,
    position: &Point3<f32>,
    rotation: &UnitQuaternion<f32>,
    window_scale: PhysicalSize<u32>,
  ) -> CameraUniform {
    let target = position
      + rotation * nalgebra::Vector3::x();
    let up = rotation * nalgebra::Vector3::z();
    let view = nalgebra::Isometry3::look_at_lh(
      position, &target, &up,
    )
    .to_homogeneous();
    let a = window_scale.width as f32
      / window_scale.height as f32;
    let zf = self.zfar;
    let zn = self.znear;
    let proj = matrix![
      1. / (f32::tan(self.fovy / 2.) * a), 0., 0., 0.;
      0., 1. / f32::tan(self.fovy / 2.), 0., 0.;
      0., 0., zf / (zf - zn), -(zf * zn) / (zf - zn);
      0., 0., 1., 0.;
    ];
    let vp = proj * view;
    CameraUniform(vp.into())
  }
}
