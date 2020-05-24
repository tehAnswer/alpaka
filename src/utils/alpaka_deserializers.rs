use serde::de::Deserialize;
use serde::Deserializer;

pub fn to_i64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
  let value: &str = Deserialize::deserialize(deserializer)?;
  serde_json::from_str(value).map_err(serde::de::Error::custom)
}

pub fn to_optional_f64<'de, D: Deserializer<'de>>(
  deserializer: D,
) -> Result<Option<f64>, D::Error> {
  let opt: Option<String> = Deserialize::deserialize(deserializer)?;

  if let Some(raw_value) = opt {
    raw_value
      .parse::<f64>()
      .map(Some)
      .map_err(serde::de::Error::custom)
  } else {
    Ok(None)
  }
}
