#[repr(C, align(8))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureHandler {
  g_key: u32,
  i_key: u32,
}

pub struct TextureArray {
  generation: Vec<u32>,
  textures: Vec<Option<super::Texture>>,
}
