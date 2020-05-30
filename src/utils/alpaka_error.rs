#[derive(Debug)]
pub enum AlpakaError {
  UnexpectedError(String),
  JsonError(serde_json::Error),
  RequestError(surf::Error),
  InvalidOrderStatus(String),
  InvalidOrderType(String),
  InvalidSide(String),
  InvalidTimeInForce(String),
  InvalidOrdersStatusFilter(String),
  InvalidOrdersDirectionFilter(String),
  UrlEncodeError(serde_urlencoded::ser::Error),
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
