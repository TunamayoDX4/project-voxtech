use super::WGPUCtx;
use wgpu::{
  AddressMode, Extent3d, Sampler,
  TextureFormat, TextureView,
};

pub struct Texture {
  pub texture: wgpu::Texture,
  pub view: TextureView,
  pub sampler: Sampler,
}
impl Texture {
  pub const DEPTH_FORMAT: TextureFormat =
    TextureFormat::Depth32Float;

  pub fn create_depth_texture(
    wgpu_ctx: &WGPUCtx,
  ) -> Self {
    let size = Extent3d {
      width: wgpu_ctx.config.width,
      height: wgpu_ctx.config.height,
      depth_or_array_layers: 1,
    };
    let desc = wgpu::TextureDescriptor {
      label: Some("Depth Texture"),
      size,
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: Self::DEPTH_FORMAT,
      usage:
        wgpu::TextureUsages::RENDER_ATTACHMENT
          | wgpu::TextureUsages::TEXTURE_BINDING,
      view_formats: &[],
    };
    let texture =
      wgpu_ctx.device.create_texture(&desc);
    let view = texture.create_view(
      &wgpu::TextureViewDescriptor::default(),
    );
    let desc = wgpu::SamplerDescriptor {
      address_mode_u: AddressMode::ClampToEdge,
      address_mode_v: AddressMode::ClampToEdge,
      address_mode_w: AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Linear,
      min_filter: wgpu::FilterMode::Linear,
      mipmap_filter:
        wgpu::MipmapFilterMode::Nearest,
      lod_min_clamp: 0.0,
      lod_max_clamp: 1.0,
      compare: None,
      ..Default::default()
    };
    let sampler =
      wgpu_ctx.device.create_sampler(&desc);
    Self {
      texture,
      view,
      sampler,
    }
  }
}
