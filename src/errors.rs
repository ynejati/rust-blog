use actix_web::error::BlockingError;
use actix_web::web::HttpResponse;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::{DatabaseError, NotFound};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
  RecordAlreadyExists,
  RecordNotFound,
  DatabaseError(diesel::result::Error),
  OperationCanceled,
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      AppError::RecordAlreadyExists => write!(f, "This record violates a unique constraint"),
      AppError::RecordNotFound => write!(f, "This record does not exist"),
      AppError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
      AppError::OperationCanceled => write!(f, "The running operation was canceled"),
    }
  }
}

// Type conversions From and Into (implicit)
impl From<diesel::result::Error> for AppError {
  fn from (e: diesel::result::Error) -> Self {
    match e {
      DatabaseError(UniqueViolation, _) => AppError::RecordAlreadyExists,
      NotFound => AppError::RecordNotFound,
      _ => AppError::DatabaseError(e),
    }
  }
}

// Blocking error into operation canceled
impl From<BlockingError<AppError>> for AppError {
  fn from(e: BlockingError<AppError>) -> Self {
    match e {
      BlockingError::Error(inner) => inner,
      BlockingError::Canceled => AppError::OperationCanceled,
    }
  }
}
