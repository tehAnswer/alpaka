use crate::AlpakaError;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use std::str::FromStr;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum OrderType {
  #[serde(rename = "market")]
  Market,
  #[serde(rename = "limit")]
  Limit,
  #[serde(rename = "stop")]
  Stop,
  #[serde(rename = "stop_limit")]
  StopLimit,
}

impl Default for OrderType {
  fn default() -> Self {
    OrderType::Market
  }
}

impl Display for OrderType {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let value = match self {
      OrderType::Market => "market",
      OrderType::Limit => "limit",
      OrderType::Stop => "stop",
      OrderType::StopLimit => "stop_limit",
    };

    write!(f, "{:?}", value)
  }
}

impl FromStr for OrderType {
  type Err = AlpakaError;
  fn from_str(
    s: &str,
  ) -> std::result::Result<
    crate::orders::order_type::OrderType,
    crate::utils::alpaka_error::AlpakaError,
  > {
    match s {
      "market" => Ok(OrderType::Market),
      "limit" => Ok(OrderType::Limit),
      "stop" => Ok(OrderType::Stop),
      "stop_limit" => Ok(OrderType::StopLimit),
      _ => Err(AlpakaError::InvalidOrderType(s.to_owned())),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::OrderType;

  #[test]
  fn test_deserialize_order_type_market() {
    let order_type: OrderType = serde_json::from_str("\"market\"").unwrap();
    assert_eq!(order_type, OrderType::Market);
  }

  #[test]
  fn test_deserialize_order_type_limit() {
    let order_type: OrderType = serde_json::from_str("\"limit\"").unwrap();
    assert_eq!(order_type, OrderType::Limit);
  }

  #[test]
  fn test_deserialize_order_type_stop() {
    let order_type: OrderType = serde_json::from_str("\"stop\"").unwrap();
    assert_eq!(order_type, OrderType::Stop);
  }

  #[test]
  fn test_deserialize_order_type_stop_limit() {
    let order_type: OrderType = serde_json::from_str("\"stop_limit\"").unwrap();
    assert_eq!(order_type, OrderType::StopLimit);
  }

  #[test]
  fn test_serialize_order_type_market() {
    let json = serde_json::to_string(&OrderType::Market).unwrap();
    assert_eq!(&json, "\"market\"");
  }

  #[test]
  fn test_serialize_order_type_limit() {
    let json = serde_json::to_string(&OrderType::Limit).unwrap();
    assert_eq!(&json, "\"limit\"");
  }

  #[test]
  fn test_serialize_order_type_stop() {
    let json = serde_json::to_string(&OrderType::Stop).unwrap();
    assert_eq!(&json, "\"stop\"");
  }

  #[test]
  fn test_serialize_order_type_stop_limit() {
    let json = serde_json::to_string(&OrderType::StopLimit).unwrap();
    assert_eq!(&json, "\"stop_limit\"");
  }

  #[test]
  fn test_serialize_side_default() {
    assert_eq!(OrderType::default(), OrderType::Market);
  }

  #[test]
  fn test_from_str_market() {
    assert_eq!("market".parse::<OrderType>().unwrap(), OrderType::Market);
  }
  #[test]
  fn test_from_str_limit() {
    assert_eq!("limit".parse::<OrderType>().unwrap(), OrderType::Limit);
  }
  #[test]
  fn test_from_str_stop() {
    assert_eq!("stop".parse::<OrderType>().unwrap(), OrderType::Stop);
  }
  #[test]
  fn test_from_str_stop_limit() {
    assert_eq!(
      "stop_limit".parse::<OrderType>().unwrap(),
      OrderType::StopLimit
    );
  }
  #[test]
  fn test_from_str_error() {
    assert_eq!("_".parse::<OrderType>().is_err(), true);
  }
}
