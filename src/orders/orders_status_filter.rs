use crate::AlpakaError;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use std::str::FromStr;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum OrdersStatusFilter {
  #[serde(rename = "open")]
  Open,
  #[serde(rename = "closed")]
  Closed,
  #[serde(rename = "all")]
  All,
}

impl Default for OrdersStatusFilter {
  fn default() -> OrdersStatusFilter {
    OrdersStatusFilter::Open
  }
}

impl Display for OrdersStatusFilter {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let value = match self {
      OrdersStatusFilter::Open => "open",
      OrdersStatusFilter::Closed => "closed",
      OrdersStatusFilter::All => "all",
    };

    write!(f, "{:?}", value)
  }
}

impl FromStr for OrdersStatusFilter {
  type Err = AlpakaError;
  fn from_str(
    s: &str,
  ) -> std::result::Result<
    crate::orders::orders_status_filter::OrdersStatusFilter,
    crate::utils::alpaka_error::AlpakaError,
  > {
    match s {
      "open" => Ok(OrdersStatusFilter::Open),
      "closed" => Ok(OrdersStatusFilter::Closed),
      "all" => Ok(OrdersStatusFilter::All),
      _ => Err(AlpakaError::InvalidOrdersStatusFilter(s.to_owned())),
    }
  }
}

#[cfg(test)]
mod tests {

  use crate::OrdersStatusFilter;
  #[test]
  fn test_orders_status_filter_open_deserialize() {
    let value: OrdersStatusFilter = serde_json::from_str("\"open\"").unwrap();
    assert_eq!(value, OrdersStatusFilter::Open)
  }
  #[test]
  fn test_orders_status_filter_closed_deserialize() {
    let value: OrdersStatusFilter = serde_json::from_str("\"closed\"").unwrap();
    assert_eq!(value, OrdersStatusFilter::Closed)
  }
  #[test]
  fn test_orders_status_filter_all_deserialize() {
    let value: OrdersStatusFilter = serde_json::from_str("\"all\"").unwrap();
    assert_eq!(value, OrdersStatusFilter::All)
  }

  #[test]
  fn test_orders_status_filter_open_serialize() {
    let json = serde_json::to_string(&OrdersStatusFilter::Open).unwrap();
    assert_eq!("\"open\"", json)
  }
  #[test]
  fn test_orders_status_filter_closed_serialize() {
    let json = serde_json::to_string(&OrdersStatusFilter::Closed).unwrap();
    assert_eq!("\"closed\"", json)
  }
  #[test]
  fn test_orders_status_filter_all_serialize() {
    let json = serde_json::to_string(&OrdersStatusFilter::All).unwrap();
    assert_eq!("\"all\"", json)
  }
  #[test]
  fn test_orders_status_filters_open_from_str() {
    assert_eq!(
      "open".parse::<OrdersStatusFilter>().unwrap(),
      OrdersStatusFilter::Open
    )
  }
  #[test]
  fn test_orders_status_filters_closed_from_str() {
    assert_eq!(
      "closed".parse::<OrdersStatusFilter>().unwrap(),
      OrdersStatusFilter::Closed
    )
  }
  #[test]
  fn test_orders_status_filters_all_from_str() {
    assert_eq!(
      "all".parse::<OrdersStatusFilter>().unwrap(),
      OrdersStatusFilter::All
    )
  }

  #[test]
  fn test_orders_status_filters_default() {
    assert_eq!(OrdersStatusFilter::default(), OrdersStatusFilter::Open)
  }
}
