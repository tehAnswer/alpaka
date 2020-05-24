use serde::Serializer;
use std::string::ToString;

pub fn to_string<T: ToString, S: Serializer>(data: &T, serializer: S) -> Result<S::Ok, S::Error> {
  serializer.serialize_str(&data.to_string())
}

pub fn opt_to_string<T: ToString, S: Serializer>(
  option: &Option<T>,
  serializer: S,
) -> Result<S::Ok, S::Error> {
  match option {
    Some(data) => serializer.serialize_str(&data.to_string()),
    None => serializer.serialize_none(),
  }
}
