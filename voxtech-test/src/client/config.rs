use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

use crate::util::static_json_config::StaticJsonConfig;

pub static COMMON_CLIENT_CFG: LazyLock<
  CommonClientConfig,
> = LazyLock::new(|| {
  CommonClientConfig::load_or_init().expect(
    "common client config initialize failure.",
  )
});
#[derive(
  Default, Debug, Clone, Serialize, Deserialize,
)]
pub struct CommonClientConfig {
  pub window: WindowConfig,
}
impl StaticJsonConfig for CommonClientConfig {
  fn get_file_path(
  ) -> std::borrow::Cow<'static, str> {
    Self::COMMON_CLIENT_CONFIG_PATH.into()
  }
}
impl CommonClientConfig {
  pub const COMMON_CLIENT_CONFIG_PATH:
    &'static str =
    "./common_client_config.json";
}

#[derive(
  Debug, Clone, Serialize, Deserialize,
)]
pub struct WindowConfig {
  pub scale: (u32, u32),
}
impl Default for WindowConfig {
  fn default() -> Self {
    Self { scale: (1280, 720) }
  }
}
