pub type StdError =
  Box<dyn std::error::Error + 'static>;
pub type StdResult<T> = Result<T, StdError>;

pub mod ray_check;
pub mod static_json_config;
