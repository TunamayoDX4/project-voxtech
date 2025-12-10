//! Texture
//! WGPUのテクスチャの抽象化オブジェクト

use super::WGPUContext;
use image::RgbaImage;
use wgpu::{
  AddressMode, BindGroup, BindGroupEntry,
  BindGroupLayout, BindGroupLayoutEntry,
  BindingResource, BindingType, Extent3d, FilterMode,
  Origin3d, SamplerBindingType, ShaderStages,
  TexelCopyBufferLayout, TexelCopyTextureInfo,
  TextureAspect, TextureDimension, TextureFormat,
  TextureSampleType, TextureUsages,
  TextureViewDimension,
};

pub struct TextureLayout {
  bindgroup_layout: BindGroupLayout,
}
impl TextureLayout {
  pub fn new(context: &WGPUContext) -> Self {
    let bindgroup_layout = context
      .device
      .create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
          label: Some("Texture bindgroup layout"),
          entries: &[
            BindGroupLayoutEntry {
              binding: 0,
              visibility: ShaderStages::FRAGMENT,
              ty: BindingType::Texture {
                sample_type: TextureSampleType::Float {
                  filterable: true,
                },
                view_dimension:
                  TextureViewDimension::D2,
                multisampled: false,
              },
              count: None,
            },
            BindGroupLayoutEntry {
              binding: 1,
              visibility: ShaderStages::FRAGMENT,
              ty: BindingType::Sampler(
                SamplerBindingType::Filtering,
              ),
              count: None,
            },
          ],
        },
      );
    Self { bindgroup_layout }
  }
}

pub struct Texture {
  bindgroup: BindGroup,
}
impl Texture {
  pub fn new_diffuse(
    context: &WGPUContext,
    layout: &TextureLayout,
    diffuse_image: &RgbaImage,
  ) -> Self {
    let dimensions = diffuse_image.dimensions();
    let texture_size = Extent3d {
      width: dimensions.0,
      height: dimensions.1,
      depth_or_array_layers: 1,
    };
    let diffuse_texture = context
      .device
      .create_texture(&wgpu::TextureDescriptor {
        label: Some("Diffuse texture object"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba8UnormSrgb,
        usage: TextureUsages::TEXTURE_BINDING
          | TextureUsages::COPY_DST,
        view_formats: &[],
      });
    context.queue.write_texture(
      TexelCopyTextureInfo {
        texture: &diffuse_texture,
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
      texture_size,
    );
    let diffuse_texture_view = diffuse_texture
      .create_view(
        &wgpu::TextureViewDescriptor::default(),
      );
    let diffuse_sampler = context
      .device
      .create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Diffuse texture sampler"),
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        address_mode_w: AddressMode::ClampToEdge,
        min_filter: FilterMode::Nearest,
        mag_filter: FilterMode::Nearest,
        mipmap_filter: FilterMode::Nearest,
        ..Default::default()
      });

    let bindgroup = context
      .device
      .create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Texture bindgroup"),
        layout: &layout.bindgroup_layout,
        entries: &[
          BindGroupEntry {
            binding: 0,
            resource: BindingResource::TextureView(
              &diffuse_texture_view,
            ),
          },
          BindGroupEntry {
            binding: 1,
            resource: BindingResource::Sampler(
              &diffuse_sampler,
            ),
          },
        ],
      });

    Self { bindgroup }
  }
}
