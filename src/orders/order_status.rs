use crate::AlpakaError;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use std::str::FromStr;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum OrderStatus {
  #[serde(rename = "new")]
  New,
  #[serde(rename = "partially_filled")]
  PartiallyFilled,
  #[serde(rename = "filled")]
  Filled,
  #[serde(rename = "done_for_day")]
  DoneForDay,
  #[serde(rename = "canceled")]
  Canceled,
  #[serde(rename = "expired")]
  Expired,
  #[serde(rename = "replaced")]
  Replaced,
  #[serde(rename = "pending_cancel")]
  PendingCancel,
  #[serde(rename = "pending_replace")]
  PendingReplace,
  #[serde(rename = "accepted")]
  Accepted,
  #[serde(rename = "pending_new")]
  PendingNew,
  #[serde(rename = "accepted_for_bidding")]
  AcceptedForBidding,
  #[serde(rename = "stopped")]
  Stopped,
  #[serde(rename = "rejected")]
  Rejected,
  #[serde(rename = "suspended")]
  Suspended,
  #[serde(rename = "calculated")]
  Calculated,
}

impl Default for OrderStatus {
  fn default() -> Self {
    OrderStatus::New
  }
}

impl Display for OrderStatus {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let value = match self {
      OrderStatus::New => "new",
      OrderStatus::PartiallyFilled => "partially_filled",
      OrderStatus::Filled => "filled",
      OrderStatus::DoneForDay => "done_for_day",
      OrderStatus::Canceled => "canceled",
      OrderStatus::Expired => "expired",
      OrderStatus::Replaced => "replaced",
      OrderStatus::PendingCancel => "pending_cancel",
      OrderStatus::PendingReplace => "pending_replace",
      OrderStatus::Accepted => "accepted",
      OrderStatus::PendingNew => "pending_new",
      OrderStatus::AcceptedForBidding => "accepted_for_bidding",
      OrderStatus::Stopped => "stopped",
      OrderStatus::Rejected => "rejected",
      OrderStatus::Suspended => "suspended",
      OrderStatus::Calculated => "calculated",
    };

    write!(f, "{:?}", value)
  }
}

impl FromStr for OrderStatus {
  type Err = AlpakaError;
  fn from_str(
    s: &str,
  ) -> std::result::Result<
    crate::orders::order_status::OrderStatus,
    crate::utils::alpaka_error::AlpakaError,
  > {
    match s {
      "new" => Ok(OrderStatus::New),
      "partially_filled" => Ok(OrderStatus::PartiallyFilled),
      "filled" => Ok(OrderStatus::Filled),
      "done_for_day" => Ok(OrderStatus::DoneForDay),
      "canceled" => Ok(OrderStatus::Canceled),
      "expired" => Ok(OrderStatus::Expired),
      "replaced" => Ok(OrderStatus::Replaced),
      "pending_cancel" => Ok(OrderStatus::PendingCancel),
      "pending_replace" => Ok(OrderStatus::PendingReplace),
      "accepted" => Ok(OrderStatus::Accepted),
      "pending_new" => Ok(OrderStatus::PendingNew),
      "accepted_for_bidding" => Ok(OrderStatus::AcceptedForBidding),
      "stopped" => Ok(OrderStatus::Stopped),
      "rejected" => Ok(OrderStatus::Rejected),
      "suspended" => Ok(OrderStatus::Suspended),
      "calculated" => Ok(OrderStatus::Calculated),
      _ => Err(AlpakaError::InvalidOrderStatus(s.to_owned())),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::OrderStatus;

  #[test]
  fn test_deserialize_order_status_new() {
    let order_status: OrderStatus = serde_json::from_str("\"new\"").unwrap();
    assert_eq!(order_status, OrderStatus::New);
  }

  #[test]
  fn test_deserialize_order_status_limit() {
    let order_status: OrderStatus = serde_json::from_str("\"partially_filled\"").unwrap();
    assert_eq!(order_status, OrderStatus::PartiallyFilled);
  }

  #[test]
  fn test_deserialize_order_status_stop() {
    let order_status: OrderStatus = serde_json::from_str("\"filled\"").unwrap();
    assert_eq!(order_status, OrderStatus::Filled);
  }

  #[test]
  fn test_deserialize_order_status_stop_limit() {
    let order_status: OrderStatus = serde_json::from_str("\"done_for_day\"").unwrap();
    assert_eq!(order_status, OrderStatus::DoneForDay);
  }

  #[test]
  fn test_deserialize_order_status_canceled() {
    let order_status: OrderStatus = serde_json::from_str("\"canceled\"").unwrap();
    assert_eq!(order_status, OrderStatus::Canceled)
  }

  #[test]
  fn test_deserialize_order_status_expired() {
    let order_status: OrderStatus = serde_json::from_str("\"expired\"").unwrap();
    assert_eq!(order_status, OrderStatus::Expired)
  }

  #[test]
  fn test_deserialize_order_status_replaced() {
    let order_status: OrderStatus = serde_json::from_str("\"replaced\"").unwrap();
    assert_eq!(order_status, OrderStatus::Replaced)
  }

  #[test]
  fn test_deserialize_order_status_pending_cancel() {
    let order_status: OrderStatus = serde_json::from_str("\"pending_cancel\"").unwrap();
    assert_eq!(order_status, OrderStatus::PendingCancel)
  }

  #[test]
  fn test_deserialize_order_status_pending_replace() {
    let order_status: OrderStatus = serde_json::from_str("\"pending_replace\"").unwrap();
    assert_eq!(order_status, OrderStatus::PendingReplace)
  }

  #[test]
  fn test_deserialize_order_status_accepted() {
    let order_status: OrderStatus = serde_json::from_str("\"accepted\"").unwrap();
    assert_eq!(order_status, OrderStatus::Accepted)
  }

  #[test]
  fn test_deserialize_order_status_pending_new() {
    let order_status: OrderStatus = serde_json::from_str("\"pending_new\"").unwrap();
    assert_eq!(order_status, OrderStatus::PendingNew)
  }

  #[test]
  fn test_deserialize_order_status_accepted_for_bidding() {
    let order_status: OrderStatus = serde_json::from_str("\"accepted_for_bidding\"").unwrap();
    assert_eq!(order_status, OrderStatus::AcceptedForBidding)
  }

  #[test]
  fn test_deserialize_order_status_stopped() {
    let order_status: OrderStatus = serde_json::from_str("\"stopped\"").unwrap();
    assert_eq!(order_status, OrderStatus::Stopped)
  }
  #[test]
  fn test_deserialize_order_status_rejected() {
    let order_status: OrderStatus = serde_json::from_str("\"rejected\"").unwrap();
    assert_eq!(order_status, OrderStatus::Rejected)
  }
  #[test]
  fn test_deserialize_order_status_suspended() {
    let order_status: OrderStatus = serde_json::from_str("\"suspended\"").unwrap();
    assert_eq!(order_status, OrderStatus::Suspended)
  }
  #[test]
  fn test_deserialize_order_status_calculated() {
    let order_status: OrderStatus = serde_json::from_str("\"calculated\"").unwrap();
    assert_eq!(order_status, OrderStatus::Calculated)
  }

  #[test]
  fn test_serialize_order_status_new() {
    let json = serde_json::to_string(&OrderStatus::New).unwrap();
    assert_eq!(&json, "\"new\"");
  }

  #[test]
  fn test_serialize_order_status_limit() {
    let json = serde_json::to_string(&OrderStatus::PartiallyFilled).unwrap();
    assert_eq!(&json, "\"partially_filled\"");
  }

  #[test]
  fn test_serialize_order_status_stop() {
    let json = serde_json::to_string(&OrderStatus::Filled).unwrap();
    assert_eq!(&json, "\"filled\"");
  }

  #[test]
  fn test_serialize_order_status_stop_limit() {
    let json = serde_json::to_string(&OrderStatus::DoneForDay).unwrap();
    assert_eq!(&json, "\"done_for_day\"");
  }

  #[test]
  fn test_serialize_order_status_canceled() {
    let json = serde_json::to_string(&OrderStatus::Canceled).unwrap();
    assert_eq!(&json, "\"canceled\"")
  }
  #[test]
  fn test_serialize_order_status_expired() {
    let json = serde_json::to_string(&OrderStatus::Expired).unwrap();
    assert_eq!(&json, "\"expired\"")
  }
  #[test]
  fn test_serialize_order_status_replaced() {
    let json = serde_json::to_string(&OrderStatus::Replaced).unwrap();
    assert_eq!(&json, "\"replaced\"")
  }
  #[test]
  fn test_serialize_order_status_pending_cancel() {
    let json = serde_json::to_string(&OrderStatus::PendingCancel).unwrap();
    assert_eq!(&json, "\"pending_cancel\"")
  }
  #[test]
  fn test_serialize_order_status_pending_replace() {
    let json = serde_json::to_string(&OrderStatus::PendingReplace).unwrap();
    assert_eq!(&json, "\"pending_replace\"")
  }
  #[test]
  fn test_serialize_order_status_accepted() {
    let json = serde_json::to_string(&OrderStatus::Accepted).unwrap();
    assert_eq!(&json, "\"accepted\"")
  }
  #[test]
  fn test_serialize_order_status_pending_new() {
    let json = serde_json::to_string(&OrderStatus::PendingNew).unwrap();
    assert_eq!(&json, "\"pending_new\"")
  }
  #[test]
  fn test_serialize_order_status_accepted_for_bidding() {
    let json = serde_json::to_string(&OrderStatus::AcceptedForBidding).unwrap();
    assert_eq!(&json, "\"accepted_for_bidding\"")
  }
  #[test]
  fn test_serialize_order_status_stopped() {
    let json = serde_json::to_string(&OrderStatus::Stopped).unwrap();
    assert_eq!(&json, "\"stopped\"")
  }
  #[test]
  fn test_serialize_order_status_rejected() {
    let json = serde_json::to_string(&OrderStatus::Rejected).unwrap();
    assert_eq!(&json, "\"rejected\"")
  }
  #[test]
  fn test_serialize_order_status_suspended() {
    let json = serde_json::to_string(&OrderStatus::Suspended).unwrap();
    assert_eq!(&json, "\"suspended\"")
  }
  #[test]
  fn test_serialize_order_status_calculated() {
    let json = serde_json::to_string(&OrderStatus::Calculated).unwrap();
    assert_eq!(&json, "\"calculated\"")
  }

  #[test]
  fn test_serialize_side_default() {
    assert_eq!(OrderStatus::default(), OrderStatus::New);
  }
  #[test]

  fn test_from_str_new() {
    assert_eq!("new".parse::<OrderStatus>().unwrap(), OrderStatus::New)
  }
  #[test]
  fn test_from_str_partially_filled() {
    assert_eq!(
      "partially_filled".parse::<OrderStatus>().unwrap(),
      OrderStatus::PartiallyFilled
    )
  }
  #[test]
  fn test_from_str_filled() {
    assert_eq!(
      "filled".parse::<OrderStatus>().unwrap(),
      OrderStatus::Filled
    )
  }
  #[test]
  fn test_from_str_done_for_day() {
    assert_eq!(
      "done_for_day".parse::<OrderStatus>().unwrap(),
      OrderStatus::DoneForDay
    )
  }
  #[test]
  fn test_from_str_canceled() {
    assert_eq!(
      "canceled".parse::<OrderStatus>().unwrap(),
      OrderStatus::Canceled
    )
  }
  #[test]
  fn test_from_str_expired() {
    assert_eq!(
      "expired".parse::<OrderStatus>().unwrap(),
      OrderStatus::Expired
    )
  }
  #[test]
  fn test_from_str_replaced() {
    assert_eq!(
      "replaced".parse::<OrderStatus>().unwrap(),
      OrderStatus::Replaced
    )
  }
  #[test]
  fn test_from_str_pending_cancel() {
    assert_eq!(
      "pending_cancel".parse::<OrderStatus>().unwrap(),
      OrderStatus::PendingCancel
    )
  }
  #[test]
  fn test_from_str_pending_replace() {
    assert_eq!(
      "pending_replace".parse::<OrderStatus>().unwrap(),
      OrderStatus::PendingReplace
    )
  }
  #[test]
  fn test_from_str_accepted() {
    assert_eq!(
      "accepted".parse::<OrderStatus>().unwrap(),
      OrderStatus::Accepted
    )
  }
  #[test]
  fn test_from_str_pending_new() {
    assert_eq!(
      "pending_new".parse::<OrderStatus>().unwrap(),
      OrderStatus::PendingNew
    )
  }
  #[test]
  fn test_from_str_accepted_for_bidding() {
    assert_eq!(
      "accepted_for_bidding".parse::<OrderStatus>().unwrap(),
      OrderStatus::AcceptedForBidding
    )
  }
  #[test]
  fn test_from_str_stopped() {
    assert_eq!(
      "stopped".parse::<OrderStatus>().unwrap(),
      OrderStatus::Stopped
    )
  }
  #[test]
  fn test_from_str_rejected() {
    assert_eq!(
      "rejected".parse::<OrderStatus>().unwrap(),
      OrderStatus::Rejected
    )
  }
  #[test]
  fn test_from_str_suspended() {
    assert_eq!(
      "suspended".parse::<OrderStatus>().unwrap(),
      OrderStatus::Suspended
    )
  }
  #[test]
  fn test_from_str_calculated() {
    assert_eq!(
      "calculated".parse::<OrderStatus>().unwrap(),
      OrderStatus::Calculated
    )
  }

  #[test]
  fn test_from_str_error() {
    assert_eq!("_".parse::<OrderStatus>().is_err(), true)
  }
}
