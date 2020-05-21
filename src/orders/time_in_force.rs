use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
// https://alpaca.news/docs/trading-on-alpaca/orders/#time-in-force
pub enum TimeInForce {
  #[serde(rename = "day")]
  DAY,
  #[serde(rename = "gtc")]
  GTC,
  #[serde(rename = "opg")]
  OPG,
  #[serde(rename = "cls")]
  CLS,
  #[serde(rename = "iok")]
  IOK,
  #[serde(rename = "fok")]
  FOK,
}

impl Default for TimeInForce {
  fn default() -> Self {
    TimeInForce::DAY
  }
}

impl Display for TimeInForce {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let value = match self {
      TimeInForce::DAY => "day",
      TimeInForce::GTC => "gtc",
      TimeInForce::OPG => "opg",
      TimeInForce::CLS => "cls",
      TimeInForce::IOK => "iok",
      TimeInForce::FOK => "fok",
    };

    write!(f, "{:?}", value)
  }
}

#[cfg(test)]
mod tests {
  use crate::TimeInForce;

  #[test]
  fn test_time_in_force_default() {
    assert_eq!(TimeInForce::default(), TimeInForce::DAY);
  }

  #[test]
  fn test_time_in_force_day_serialize() {
    let time_in_force = serde_json::to_string(&TimeInForce::DAY).unwrap();
    assert_eq!(&time_in_force, "\"day\"");
  }

  #[test]
  fn test_time_in_force_day_deserialize() {
    let time_in_force: TimeInForce = serde_json::from_str("\"day\"").unwrap();
    assert_eq!(time_in_force, TimeInForce::DAY);
  }

  #[test]
  fn test_time_in_force_gtc_serialize() {
    let time_in_force = serde_json::to_string(&TimeInForce::GTC).unwrap();
    assert_eq!(&time_in_force, "\"gtc\"");
  }

  #[test]
  fn test_time_in_force_gtc_deserialize() {
    let time_in_force: TimeInForce = serde_json::from_str("\"gtc\"").unwrap();
    assert_eq!(time_in_force, TimeInForce::GTC);
  }

  #[test]
  fn test_time_in_force_opg_serialize() {
    let time_in_force = serde_json::to_string(&TimeInForce::OPG).unwrap();
    assert_eq!(&time_in_force, "\"opg\"");
  }

  #[test]
  fn test_time_in_force_opg_deserialize() {
    let time_in_force: TimeInForce = serde_json::from_str("\"opg\"").unwrap();
    assert_eq!(time_in_force, TimeInForce::OPG);
  }

  #[test]
  fn test_time_in_force_cls_deserialize() {
    let time_in_force = serde_json::to_string(&TimeInForce::CLS).unwrap();
    assert_eq!(&time_in_force, "\"cls\"");
  }

  #[test]
  fn test_time_in_force_cls_serialize() {
    let time_in_force: TimeInForce = serde_json::from_str("\"cls\"").unwrap();
    assert_eq!(time_in_force, TimeInForce::CLS);
  }

  #[test]
  fn test_time_in_force_iok_deserialize() {
    let time_in_force = serde_json::to_string(&TimeInForce::IOK).unwrap();
    assert_eq!(&time_in_force, "\"iok\"");
  }

  #[test]
  fn test_time_in_force_iok_serialize() {
    let time_in_force: TimeInForce = serde_json::from_str("\"iok\"").unwrap();
    assert_eq!(time_in_force, TimeInForce::IOK);
  }

  #[test]
  fn test_time_in_force_fok_deserialize() {
    let time_in_force = serde_json::to_string(&TimeInForce::FOK).unwrap();
    assert_eq!(&time_in_force, "\"fok\"");
  }

  #[test]
  fn test_time_in_force_fok_serialize() {
    let time_in_force: TimeInForce = serde_json::from_str("\"fok\"").unwrap();
    assert_eq!(time_in_force, TimeInForce::FOK);
  }
}
