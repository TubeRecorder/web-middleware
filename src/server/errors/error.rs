use std::{
  error,
  fmt::{
    Debug,
    Display,
    Formatter,
    Result as fmtResult,
  },
};

use tonic::Status;

/// The base error for the service.
#[derive(Debug, PartialEq)]
pub enum Error {
  /// An unknown error has been encountered.
  UnknownError(String),

  /// A technical error has been encountered.
  TechnicalError(String),

  /// Invalid argument error
  InvalidArgument(String),
}

impl Display for Error {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> fmtResult {
    match self {
      Error::UnknownError(message) => {
        write!(f, "UnknownError: {}", message)
      },
      Error::TechnicalError(message) => {
        write!(f, "TechnicalError: {}", message)
      },
      Error::InvalidArgument(message) => {
        write!(f, "InvalidArgument: {}", message)
      },
    }
  }
}

impl error::Error for Error {}

impl From<Error> for Status {
  fn from(err: Error) -> Self {
    match err {
      Error::UnknownError(message) => Status::unknown(message),
      Error::TechnicalError(message) => Status::internal(message),
      Error::InvalidArgument(message) => {
        Status::invalid_argument(message)
      },
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Self {
    Error::UnknownError(format!(
      "unknown std::io error: {}",
      err
    ))
  }
}

impl From<log::SetLoggerError> for Error {
  fn from(err: log::SetLoggerError) -> Self {
    Error::UnknownError(format!(
      "unknown log::SetLoggerError error: {}",
      err
    ))
  }
}
