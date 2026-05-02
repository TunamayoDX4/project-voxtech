use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

use crate::util::static_json_config::StaticJsonConfig;

pub static SERVER_CFG: LazyLock<ServerConfig> =
  LazyLock::new(|| {
    ServerConfig::load_or_init().expect(
      "server config initialize failure.",
    )
  });
#[derive(
  Debug, Clone, Serialize, Deserialize,
)]
pub struct ServerConfig {
  /// Cycle Per Sec
  cps: u32,
}
impl StaticJsonConfig for ServerConfig {
  fn get_file_path(
  ) -> std::borrow::Cow<'static, str> {
    Self::SERVER_CONFIG_PATH.into()
  }
}
impl ServerConfig {
  pub const SERVER_CONFIG_PATH: &'static str =
    "./server_config.json";
}
impl Default for ServerConfig {
  fn default() -> Self {
    Self { cps: 60 }
  }
}
