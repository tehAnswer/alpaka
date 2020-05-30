use crate::AlpakaError;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use std::str::FromStr;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum OrdersDirectionFilter {
  #[serde(rename = "asc")]
  Asc,
  #[serde(rename = "desc")]
  Desc,
}

impl Default for OrdersDirectionFilter {
  fn default() -> Self {
    OrdersDirectionFilter::Desc
  }
}

impl Display for OrdersDirectionFilter {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let value = match self {
      OrdersDirectionFilter::Asc => "asc",
      OrdersDirectionFilter::Desc => "desc",
    };

    write!(f, "{:?}", value)
  }
}

impl FromStr for OrdersDirectionFilter {
  type Err = AlpakaError;
  fn from_str(
    s: &str,
  ) -> std::result::Result<
    crate::orders::orders_direction_filter::OrdersDirectionFilter,
    crate::utils::alpaka_error::AlpakaError,
  > {
    match s {
      "asc" => Ok(OrdersDirectionFilter::Asc),
      "desc" => Ok(OrdersDirectionFilter::Desc),
      _ => Err(AlpakaError::InvalidOrdersDirectionFilter(s.to_owned())),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::OrdersDirectionFilter;

  #[test]
  fn test_orders_direction_filter_deserialize_asc() {
    let direction: OrdersDirectionFilter = serde_json::from_str("\"asc\"").unwrap();
    assert_eq!(direction, OrdersDirectionFilter::Asc)
  }

  #[test]
  fn test_orders_direction_filter_serialize_asc() {
    let json = serde_json::to_string(&OrdersDirectionFilter::Asc).unwrap();
    assert_eq!(json, "\"asc\"")
  }

  #[test]
  fn test_orders_direction_filter_from_str_asc() {
    assert_eq!(
      "asc".parse::<OrdersDirectionFilter>().unwrap(),
      OrdersDirectionFilter::Asc
    )
  }

  #[test]
  fn test_orders_direction_filter_deserialize_desc() {
    let direction: OrdersDirectionFilter = serde_json::from_str("\"desc\"").unwrap();
    assert_eq!(direction, OrdersDirectionFilter::Desc)
  }

  #[test]
  fn test_orders_direction_filter_serialize_desc() {
    let json = serde_json::to_string(&OrdersDirectionFilter::Desc).unwrap();
    assert_eq!(json, "\"desc\"")
  }

  #[test]
  fn test_orders_direction_filter_from_str_desc() {
    assert_eq!(
      "desc".parse::<OrdersDirectionFilter>().unwrap(),
      OrdersDirectionFilter::Desc
    )
  }
}
