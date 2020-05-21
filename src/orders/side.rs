use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Side {
  #[serde(rename = "buy")]
  Buy,
  #[serde(rename = "sell")]
  Sell,
}

impl Default for Side {
  fn default() -> Self {
    Side::Buy
  }
}

impl Display for Side {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let value = match self {
      Side::Buy => "buy",
      Side::Sell => "sell",
    };

    write!(f, "{:?}", value)
  }
}

#[cfg(test)]
mod tests {
  use crate::Side;

  #[test]
  fn test_deserialize_side_buy() {
    let side: Side = serde_json::from_str("\"buy\"").unwrap();
    assert_eq!(side, Side::Buy);
  }

  #[test]
  fn test_deserialize_side_sell() {
    let side: Side = serde_json::from_str("\"sell\"").unwrap();
    assert_eq!(side, Side::Sell);
  }

  #[test]
  fn test_serialize_side_buy() {
    let json = serde_json::to_string(&Side::Buy).unwrap();
    assert_eq!(&json, "\"buy\"");
  }

  #[test]
  fn test_serialize_side_sell() {
    let json = serde_json::to_string(&Side::Sell).unwrap();
    assert_eq!(&json, "\"sell\"");
  }

  #[test]
  fn test_serialize_side_default() {
    assert_eq!(Side::default(), Side::Buy);
  }
}
