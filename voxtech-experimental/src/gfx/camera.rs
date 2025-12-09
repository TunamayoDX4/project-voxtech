use bytemuck::{Pod, Zeroable};
use wgpu::{util::DeviceExt, wgc::instance};

/// カメラのインスタンス
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CameraInstance {
  pub position: nalgebra::Point3<f64>,
  pub velocity: nalgebra::Vector3<f64>,
  pub rotation: nalgebra::UnitQuaternion<f64>,
}

/// カメラ用のコンフィグ
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CameraConfig {
  pub fovy: f64,
  pub near: f64,
  pub far: f64,
}
impl CameraConfig {
  pub fn uniform(
    &self,
    instance: &CameraInstance,
    window: &winit::window::Window,
  ) -> CameraUniform {
    // ビュー行列の生成
    let inner_size = window.inner_size();
    let aspect =
      inner_size.width as f64 / inner_size.height as f64;
    let target = instance.position
      + instance.rotation * nalgebra::Vector3::y();
    let up = instance.rotation * nalgebra::Vector3::z();
    let view = nalgebra::Matrix4::look_at_rh(
      &instance.position,
      &target,
      &up,
    );

    // プロジェクション行列の生成
    let proj = nalgebra::Perspective3::new(
      aspect, self.fovy, self.near, self.far,
    );
    let proj = proj.as_matrix();

    // 変換行列の生成
    let vp = proj * view;

    CameraUniform(vp.cast::<f32>().into())
  }
}

/// カメラ用のユニフォームバッファ
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct CameraUniform([[f32; 4]; 4]);

/// カメラ用のユニフォームのインスタンス
pub struct CameraUniformInstance {
  pub buffer: wgpu::Buffer,
  pub bindgroup_layout: wgpu::BindGroupLayout,
  pub bindgroup: wgpu::BindGroup,
  uniform: CameraUniform,
}
impl CameraUniformInstance {
  pub fn new(
    context: &super::WGPUContext,
    instance: &CameraInstance,
  ) -> Self {
    // カメラ行列自体の生成
    let uniform = context
      .camera
      .read()
      .uniform(instance, &context.window);

    // カメラ行列用バッファの初期化
    let buffer = context
      .device
      .create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: Some("Camera uniform buffer"),
          contents: bytemuck::cast_slice(&uniform.0),
          usage: wgpu::BufferUsages::UNIFORM
            | wgpu::BufferUsages::COPY_DST,
        },
      );

    // バインドグループのレイアウトの初期化
    let bindgroup_layout = context
      .device
      .create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
          label: Some("Camera bindgroup layout"),
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

    // バインドグループの初期化
    let bindgroup = context
      .device
      .create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Camera bindgroup"),
        layout: &bindgroup_layout,
        entries: &[wgpu::BindGroupEntry {
          binding: 0,
          resource: buffer.as_entire_binding(),
        }],
      });

    Self {
      buffer,
      bindgroup_layout,
      bindgroup,
      uniform,
    }
  }

  pub fn update(
    &mut self,
    context: &super::WGPUContext,
    instance: &CameraInstance,
  ) {
    self.uniform = context
      .camera
      .read()
      .uniform(instance, &context.window);
    context.queue.write_buffer(
      &self.buffer,
      0,
      bytemuck::cast_slice(&[self.uniform]),
    );
  }
}
