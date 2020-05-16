pub enum AlpakaError {
  Unexpected(String),
  Json(serde_json::Error),
}

impl From<serde_json::Error> for AlpakaError {
  fn from(err: serde_json::Error) -> AlpakaError {
    AlpakaError::Json(err)
  }
}
