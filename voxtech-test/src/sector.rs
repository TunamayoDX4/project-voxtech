use base64::{
  display::Base64Display, prelude::BASE64_STANDARD_NO_PAD,
};

pub struct SectorServer {}

pub struct Sector {
  pub pos: [i64; 3],
}
impl Sector {
  pub fn new(pos: [i64; 3]) -> Self {
    Self { pos }
  }
  pub fn generate_name(
    &self,
    buf: &mut impl std::fmt::Write,
  ) -> std::fmt::Result {
    let posb = [
      self.pos[0].to_le_bytes(),
      self.pos[1].to_le_bytes(),
      self.pos[2].to_le_bytes(),
    ];
    buf.write_fmt(format_args!(
      "{0}={1}={2}",
      Base64Display::new(
        &posb[0],
        &BASE64_STANDARD_NO_PAD,
      ),
      Base64Display::new(
        &posb[1],
        &BASE64_STANDARD_NO_PAD,
      ),
      Base64Display::new(
        &posb[2],
        &BASE64_STANDARD_NO_PAD,
      )
    ))
  }
}
