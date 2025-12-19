//! Texture
//! WGPUのテクスチャの抽象化オブジェクト

use std::io::Read;

use super::WGPUContext;
use image::RgbaImage;
use wgpu::{
  AddressMode, BindGroup, BindGroupEntry,
  BindGroupLayout, BindGroupLayoutEntry,
  BindingResource, BindingType, CompareFunction,
  Extent3d, FilterMode, Origin3d, Sampler,
  SamplerBindingType, ShaderStages,
  TexelCopyBufferLayout, TexelCopyTextureInfo,
  TextureAspect, TextureDimension, TextureFormat,
  TextureSampleType, TextureUsages, TextureView,
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

/// テクスチャ用のバインド構造体
pub struct Texture {
  pub size: Extent3d,
  pub texture: wgpu::Texture,
  pub view: TextureView,
  pub sampler: Sampler,
}
impl Texture {
  pub fn new_diffuse(
    context: &WGPUContext,
    diffuse_image: &RgbaImage,
  ) -> Self {
    let dimensions = diffuse_image.dimensions();
    let size = Extent3d {
      width: dimensions.0,
      height: dimensions.1,
      depth_or_array_layers: 1,
    };
    let texture = context.device.create_texture(
      &wgpu::TextureDescriptor {
        label: Some("Diffuse texture object"),
        size,
        mip_level_count: 1,
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
        mipmap_filter: FilterMode::Nearest,
        ..Default::default()
      },
    );
    Self {
      size,
      texture,
      view,
      sampler,
    }
  }

  pub const DEPTH_FORMAT: TextureFormat =
    TextureFormat::Depth32Float;
  pub fn new_depth(
    context: &WGPUContext,
    label: &str,
  ) -> Self {
    let size = Extent3d {
      width: context.config.width.max(1),
      height: context.config.height.max(1),
      depth_or_array_layers: 1,
    };
    let texture = context.device.create_texture(
      &wgpu::TextureDescriptor {
        label: Some(label),
        size,
        mip_level_count: 1,
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
        mipmap_filter: FilterMode::Nearest,
        lod_min_clamp: 0.0,
        lod_max_clamp: 100.0,
        compare: Some(CompareFunction::LessEqual),
        ..Default::default()
      },
    );

    Self {
      size,
      texture,
      view,
      sampler,
    }
  }
}

pub struct DiffuseTexture {
  texture: Texture,
  bindgroup: BindGroup,
}
impl DiffuseTexture {
  pub fn new_diffuse_from_image(
    context: &WGPUContext,
    layout: &TextureLayout,
    image_path: impl AsRef<std::path::Path>,
  ) -> crate::aliases::StdResult<Self> {
    let fp = std::fs::File::open(image_path)?;
    let len = fp.metadata()?.len();
    let mut fp = std::io::BufReader::new(fp);
    let mut bin = Vec::with_capacity(len as usize);
    fp.read_to_end(&mut bin)?;
    let dyn_image = image::load_from_memory(&bin)?;
    let tex = Self::new_diffuse(
      context,
      layout,
      &dyn_image.to_rgba8(),
    );
    Ok(tex)
  }
  pub fn new_diffuse(
    context: &WGPUContext,
    layout: &TextureLayout,
    diffuse_image: &RgbaImage,
  ) -> Self {
    let texture =
      Texture::new_diffuse(context, diffuse_image);
    let bindgroup = context
      .device
      .create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Texture bindgroup"),
        layout: &layout.bindgroup_layout,
        entries: &[
          BindGroupEntry {
            binding: 0,
            resource: BindingResource::TextureView(
              &texture.view,
            ),
          },
          BindGroupEntry {
            binding: 1,
            resource: BindingResource::Sampler(
              &texture.sampler,
            ),
          },
        ],
      });

    Self { bindgroup, texture }
  }
}
