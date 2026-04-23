use super::WGPUCtx;
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

/// カメラのインスタンス
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Camera3DInstance {
  pub position: nalgebra::Point3<f64>,
  pub velocity: nalgebra::Vector3<f64>,
  pub rotation: nalgebra::UnitQuaternion<f64>,
}

/// カメラ用のコンフィグ
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Camera3DConfig {
  pub fovy: f64,
  pub near: f64,
  pub far: f64,
}
impl Camera3DConfig {
  pub fn uniform(
    &self,
    instance: &Camera3DInstance,
    scale: (u32, u32),
  ) -> Camera3DUniform {
    // ビュー行列の生成
    let aspect = scale.0 as f64 / scale.1 as f64;
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

    Camera3DUniform(vp.cast::<f32>().into())
  }
}

/// カメラ用のユニフォームバッファ
#[repr(C)]
#[derive(
  Debug, Clone, Copy, PartialEq, Pod, Zeroable,
)]
pub struct Camera3DUniform([[f32; 4]; 4]);

/// カメラ用のユニフォームのインスタンス
pub struct Camera3DUniformInstance {
  pub buffer: wgpu::Buffer,
  pub bindgroup_layout: wgpu::BindGroupLayout,
  pub bindgroup: wgpu::BindGroup,
  uniform: Camera3DUniform,
}
impl Camera3DUniformInstance {
  pub fn new(
    context: &WGPUCtx,
    config: &Camera3DConfig,
    instance: &Camera3DInstance,
  ) -> Self {
    let config_lock = context.config.lock();
    // カメラ行列自体の生成
    let uniform = config.uniform(
      instance,
      (
        config_lock.width,
        config_lock.height,
      ),
    );

    // カメラ行列用バッファの初期化
    let buffer = context
      .device
      .create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: Some("Camera3D uniform buffer"),
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
          label: Some("Camera3D bindgroup layout"),
          entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::TASK
              | wgpu::ShaderStages::MESH,
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
        label: Some("Camera3D bindgroup"),
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
    config: &Camera3DConfig,
    instance: &Camera3DInstance,
    context: &WGPUCtx,
  ) {
    let ctx_config = context.config.lock();
    self.uniform = config.uniform(
      instance,
      (
        ctx_config.width,
        ctx_config.height,
      ),
    );
    context.queue.write_buffer(
      &self.buffer,
      0,
      bytemuck::cast_slice(&[self.uniform]),
    );
  }

  pub fn rendering(
    &self,
    rpass: &mut wgpu::RenderPass,
  ) {
    rpass.set_bind_group(0, &self.bindgroup, &[]);
  }
}
