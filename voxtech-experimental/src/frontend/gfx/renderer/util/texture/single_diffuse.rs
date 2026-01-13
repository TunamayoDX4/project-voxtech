use super::WGPUCtx;
use image::RgbaImage;
use std::io::Read;

pub struct DiffuseTextureLayout {
  bindgroup_layout: wgpu::BindGroupLayout,
}
impl DiffuseTextureLayout {
  pub fn new(context: &WGPUCtx) -> Self {
    let bindgroup_layout = context
      .device
      .create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
          label: Some("Texture bindgroup layout"),
          entries: &[
            wgpu::BindGroupLayoutEntry {
              binding: 0,
              visibility: wgpu::ShaderStages::FRAGMENT,
              ty: wgpu::BindingType::Texture {
                sample_type:
                  wgpu::TextureSampleType::Float {
                    filterable: true,
                  },
                view_dimension:
                  wgpu::TextureViewDimension::D2,
                multisampled: false,
              },
              count: None,
            },
            wgpu::BindGroupLayoutEntry {
              binding: 1,
              visibility: wgpu::ShaderStages::FRAGMENT,
              ty: wgpu::BindingType::Sampler(
                wgpu::SamplerBindingType::Filtering,
              ),
              count: None,
            },
          ],
        },
      );
    Self { bindgroup_layout }
  }
}

pub struct DiffuseTexture {
  texture: super::Texture,
  bindgroup: wgpu::BindGroup,
}
impl DiffuseTexture {
  pub fn new_diffuse_from_image(
    context: &WGPUCtx,
    layout: &DiffuseTextureLayout,
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
    context: &WGPUCtx,
    layout: &DiffuseTextureLayout,
    diffuse_image: &RgbaImage,
  ) -> Self {
    let texture = super::Texture::new_diffuse(
      context,
      diffuse_image,
    );
    let bindgroup = context
      .device
      .create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Texture bindgroup"),
        layout: &layout.bindgroup_layout,
        entries: &[
          wgpu::BindGroupEntry {
            binding: 0,
            resource:
              wgpu::BindingResource::TextureView(
                &texture.view,
              ),
          },
          wgpu::BindGroupEntry {
            binding: 1,
            resource: wgpu::BindingResource::Sampler(
              &texture.sampler,
            ),
          },
        ],
      });

    Self { bindgroup, texture }
  }
}
