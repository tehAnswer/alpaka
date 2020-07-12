use serde::de::Deserialize;
use serde::Deserializer;

use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{Duration, UNIX_EPOCH};

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

pub fn to_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
  let opt: Option<String> = Deserialize::deserialize(deserializer)?;

  if let Some(raw_value) = opt {
    raw_value.parse::<f64>().map_err(serde::de::Error::custom)
  } else {
    Err(serde::de::Error::custom("Nothing to deserialize as f64"))
  }
}

pub fn to_datetime_utc<'de, D: Deserializer<'de>>(
  deserializer: D,
) -> Result<DateTime<Utc>, D::Error> {
  let value: u64 = Deserialize::deserialize(deserializer)?;
  let d = UNIX_EPOCH + Duration::from_nanos(value);
  // Create DateTime from SystemTime
  Ok(DateTime::<Utc>::from(d))
}
