use std::{
  borrow::Cow,
  fs::{create_dir_all, File},
  io::{BufReader, BufWriter},
  marker::PhantomData,
  path::PathBuf,
};

use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug)]
pub struct ConfigInitError<C: StaticJsonConfig>
{
  _dummy: PhantomData<C>,
  kind: ConfigInitErrorKind,
}
impl<C: StaticJsonConfig> std::fmt::Display
  for ConfigInitError<C>
{
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.write_fmt(format_args!(
      "{0} load or init error: {1}",
      std::any::type_name::<C>(),
      self.kind
    ))
  }
}
impl<C: StaticJsonConfig>
  From<ConfigInitErrorKind>
  for ConfigInitError<C>
{
  fn from(value: ConfigInitErrorKind) -> Self {
    Self {
      _dummy: PhantomData,
      kind: value,
    }
  }
}

#[derive(Debug)]
pub enum ConfigInitErrorKind {
  FileLoadError(std::io::Error),
  FileWriteError(std::io::Error),
  DirCreateError(std::io::Error),
  SerializeError(serde_json::Error),
  DeserializeError(serde_json::Error),
}
impl std::fmt::Display for ConfigInitErrorKind {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      Self::FileLoadError(error) => {
        f.write_fmt(format_args!(
          "config file loading error: {error}"
        ))
      }
      Self::FileWriteError(error) => {
        f.write_fmt(format_args!(
          "config file writing error: {error}"
        ))
      }
      Self::DirCreateError(error) => {
        f.write_fmt(format_args!(
          "config directory creating error: {error}"
        ))
      }
      Self::SerializeError(error) => {
        f.write_fmt(format_args!(
          "config file serialize error: {error}"
        ))
      }
      Self::DeserializeError(error) => {
        f.write_fmt(format_args!(
          "config file deserialize error: {error}"
        ))
      }
    }
  }
}
pub trait StaticJsonConfig:
  Sized + DeserializeOwned + Serialize + Default
{
  fn get_file_path() -> Cow<'static, str>;
  fn load_or_init(
  ) -> Result<Self, ConfigInitError<Self>> {
    let path = Self::get_file_path();
    let path: &str = &path;
    let path = PathBuf::from(path);
    match File::open(&path)
      .map(BufReader::new)
      .map(serde_json::from_reader)
      .map_err(|e| (e.kind(), e))
    {
      Ok(Ok(config)) => Ok(config),

      Err((
        std::io::ErrorKind::NotFound,
        _,
      )) => {
        if let Some(parent) = path.parent() {
          create_dir_all(parent).map_err(
            ConfigInitErrorKind::DirCreateError,
          )?;
        }
        let default = Self::default();
        File::create(&path)
          .map(BufWriter::new)
          .map(|w| {
            serde_json::to_writer_pretty(w, &default)
          })
          .map_err(ConfigInitErrorKind::FileWriteError)?
          .map_err(
            ConfigInitErrorKind::DeserializeError,
          )?;

        Ok(default)
      }

      Ok(Err(e)) => Err(
        ConfigInitErrorKind::DeserializeError(
          e,
        )
        .into(),
      ),

      Err((_, e)) => Err(
        ConfigInitErrorKind::FileLoadError(e)
          .into(),
      ),
    }
  }
}
