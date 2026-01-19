pub trait AutoLoadConfig:
  Default + serde::de::DeserializeOwned + serde::Serialize
{
  fn config_file_name() -> std::borrow::Cow<'static, str>;
  fn allow_default() -> bool;
  fn initialize() -> Result<Self, ConfigLoadError> {
    let config_file_name = Self::config_file_name();
    let fp = match std::fs::File::open(
      config_file_name.as_ref(),
    )
    .map_err(|e| (e.kind(), e))
    {
      // コンフィグファイルがなければ？
      Err((std::io::ErrorKind::NotFound, _)) => {
        let default = Self::default();
        let fp = std::fs::File::create(
          config_file_name.as_ref(),
        )
        .map(std::io::BufWriter::new)
        .map_err(ConfigLoadError::IOError)?;

        // デフォコンフィグを書き出す
        serde_json::to_writer_pretty(fp, &default)
          .map_err(ConfigLoadError::SerdeError)?;

        // デフォルトが許可されてればおｋ
        return if Self::allow_default() {
          Ok(default)
        } else {
          Err(
            ConfigLoadError::DefaultConfigNotAllowed(
              config_file_name,
            ),
          )
        };
      }
      // コンフィグファイルがあれば普通にロード
      Ok(cfg_file) => Ok(std::io::BufReader::new(
        cfg_file,
      )),
      // IOエラーの時のフォールバック
      Err((_, e)) => Err(ConfigLoadError::IOError(e)),
    }?;
    let config = serde_json::from_reader(fp)
      .map_err(ConfigLoadError::SerdeError)?;
    Ok(config)
  }
}

#[derive(Debug)]
pub enum ConfigLoadError {
  DefaultConfigNotAllowed(
    std::borrow::Cow<'static, str>,
  ),
  IOError(std::io::Error),
  SerdeError(serde_json::Error),
}
impl std::fmt::Display for ConfigLoadError {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      ConfigLoadError::DefaultConfigNotAllowed(
        config_name,
      ) => f.write_fmt(format_args!(
        "default config file: {config_name} created. \n\n\
        please modify this config and restart this program.\
        ")),
      ConfigLoadError::IOError(error) => {
        f.write_fmt(format_args!(
          "config loading error(OS I/O). {error}"
        ))
      }
      ConfigLoadError::SerdeError(error) => f
        .write_fmt(format_args!(
          "config parsing error(Serde). {error}"
        )),
    }
  }
}
