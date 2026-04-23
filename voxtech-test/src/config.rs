use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

pub static APP_CONFIG: LazyLock<AppConfig> =
  LazyLock::new(|| {
    AppConfig::initialize()
      .expect("failed app_config loading")
  });

#[derive(
  Debug, Clone, Copy, Serialize, Deserialize,
)]
pub struct WindowSetting {
  pub allow_resize: bool,
  pub size: (u32, u32),
}
impl Default for WindowSetting {
  fn default() -> Self {
    Self {
      allow_resize: true,
      size: (1280, 720),
    }
  }
}

#[derive(
  Debug, Clone, Copy, Default, Serialize, Deserialize,
)]
pub struct AppConfig {
  pub window: WindowSetting,
}
impl AppConfig {
  pub const CONFIG_FILE: &'static str = "app_conf.json";

  pub fn initialize()
  -> Result<Self, Box<dyn std::error::Error>> {
    match std::fs::File::open(Self::CONFIG_FILE)
      .map(std::io::BufReader::new)
      .map_err(|e| (e.kind(), e))
    {
      Ok(fp) => Ok(serde_json::from_reader(fp)?),
      Err((std::io::ErrorKind::NotFound, _)) => {
        let default = Self::default();
        let fp =
          std::fs::File::create(Self::CONFIG_FILE)?;
        let fp = std::io::BufWriter::new(fp);
        serde_json::to_writer_pretty(fp, &default)?;
        Ok(default)
      }
      Err((_, e)) => Err(e.into()),
    }
  }
}
