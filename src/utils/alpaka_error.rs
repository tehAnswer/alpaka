#[derive(Debug)]
pub enum AlpakaError {
  UnexpectedError(String),
  JsonError(serde_json::Error),
  RequestError(surf::Error),
}

impl From<serde_json::Error> for AlpakaError {
  fn from(err: serde_json::Error) -> AlpakaError {
    AlpakaError::JsonError(err)
  }
}

impl From<surf::Error> for AlpakaError {
  fn from(err: surf::Error) -> AlpakaError {
    AlpakaError::RequestError(err)
  }
}
