//! Texture
//! WGPUのテクスチャの抽象化オブジェクト

pub mod single_diffuse;
pub mod texture_array;

use std::io::Read;

use super::WGPUCtx;

use image::RgbaImage;
use wgpu::{
  AddressMode, CompareFunction, Extent3d, FilterMode,
  MipmapFilterMode, Origin3d, Sampler,
  TexelCopyBufferLayout, TexelCopyTextureInfo,
  TextureAspect, TextureDimension, TextureFormat,
  TextureUsages, TextureView,
};

/// テクスチャ用のバインド構造体
pub struct Texture {
  pub size: Extent3d,
  pub mip_level_count: u32,
  pub texture: wgpu::Texture,
  pub view: TextureView,
  pub sampler: Sampler,
}
impl Texture {
  pub fn new_diffuse(
    context: &WGPUCtx,
    diffuse_image: &RgbaImage,
  ) -> Self {
    let dimensions = diffuse_image.dimensions();
    let size = Extent3d {
      width: dimensions.0,
      height: dimensions.1,
      depth_or_array_layers: 1,
    };
    let mip_level_count = 1;
    let texture = context.device.create_texture(
      &wgpu::TextureDescriptor {
        label: Some("Diffuse texture object"),
        size,
        mip_level_count,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba8UnormSrgb,
        usage: TextureUsages::TEXTURE_BINDING
          | TextureUsages::COPY_DST,
        view_formats: &[],
      },
    );
    context.queue.write_texture(
      TexelCopyTextureInfo {
        texture: &texture,
        mip_level: 0,
        origin: Origin3d::ZERO,
        aspect: TextureAspect::All,
      },
      &diffuse_image,
      TexelCopyBufferLayout {
        offset: 0,
        bytes_per_row: Some(4 * dimensions.0),
        rows_per_image: Some(dimensions.1),
      },
      size,
    );
    let view = texture.create_view(
      &wgpu::TextureViewDescriptor::default(),
    );
    let sampler = context.device.create_sampler(
      &wgpu::SamplerDescriptor {
        label: Some("Diffuse texture sampler"),
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        address_mode_w: AddressMode::ClampToEdge,
        min_filter: FilterMode::Nearest,
        mag_filter: FilterMode::Nearest,
        mipmap_filter: MipmapFilterMode::Nearest,
        ..Default::default()
      },
    );
    Self {
      size,
      mip_level_count,
      texture,
      view,
      sampler,
    }
  }

  pub const DEPTH_FORMAT: TextureFormat =
    TextureFormat::Depth32Float;
  pub fn new_depth(
    context: &WGPUCtx,
    label: &str,
  ) -> Self {
    let config = context.config.lock();
    let size = Extent3d {
      width: config.width,
      height: config.height,
      depth_or_array_layers: 1,
    };
    let mip_level_count = 1;
    let texture = context.device.create_texture(
      &wgpu::TextureDescriptor {
        label: Some(label),
        size,
        mip_level_count,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: Self::DEPTH_FORMAT,
        usage: TextureUsages::RENDER_ATTACHMENT
          | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
      },
    );
    let view = texture.create_view(
      &wgpu::TextureViewDescriptor::default(),
    );
    let sampler = context.device.create_sampler(
      &wgpu::SamplerDescriptor {
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        address_mode_w: AddressMode::ClampToEdge,
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Linear,
        mipmap_filter: MipmapFilterMode::Nearest,
        lod_min_clamp: 0.0,
        lod_max_clamp: 100.0,
        compare: Some(CompareFunction::LessEqual),
        ..Default::default()
      },
    );

    Self {
      size,
      mip_level_count,
      texture,
      view,
      sampler,
    }
  }
}

pub struct DepthTexture {
  texture: Texture,
}
impl DepthTexture {
  pub const DEPTH_FORMAT: TextureFormat =
    TextureFormat::Depth32Float;
  pub fn new_depth(
    context: &WGPUCtx,
    label: &str,
  ) -> Self {
    let texture = Texture::new_depth(context, label);
    Self { texture }
  }
}
