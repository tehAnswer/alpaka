use serde::Serializer;
use std::string::ToString;

pub fn to_string<T: ToString, S: Serializer>(data: &T, serializer: S) -> Result<S::Ok, S::Error> {
  serializer.serialize_str(&data.to_string())
}
