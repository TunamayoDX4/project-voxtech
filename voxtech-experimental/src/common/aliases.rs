/* --- std::error::Error関連 --- */
pub type StdError = Box<dyn std::error::Error>;
pub type StdSendError =
  Box<dyn std::error::Error + Send>;
pub type StdResult<T> = Result<T, StdError>;
pub type StdSendResult<T> = Result<T, StdSendError>;
