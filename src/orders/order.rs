use crate::utils::alpaka_deserializers::{to_i64, to_optional_f64};
use crate::{OrderStatus, OrderType, Side, TimeInForce};
use serde_derive::{Deserialize, Serialize};

#[derive(Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Order {
  pub id: String,
  pub client_order_id: String,
  pub created_at: String,
  pub updated_at: String,
  pub submitted_at: String,
  pub filled_at: Option<String>,
  pub expired_at: Option<String>,
  pub canceled_at: Option<String>,
  pub failed_at: Option<String>,
  pub replaced_at: Option<String>,
  pub replaced_by: Option<String>,
  pub replaces: Option<String>,
  pub asset_id: String,
  pub symbol: String,
  pub asset_class: String,
  #[serde(deserialize_with = "to_i64")]
  pub qty: i64,
  #[serde(deserialize_with = "to_i64")]
  pub filled_qty: i64,
  pub order_class: Option<String>,
  #[serde(rename = "type")]
  pub order_type: OrderType,
  pub side: Side,
  pub time_in_force: TimeInForce,
  #[serde(deserialize_with = "to_optional_f64")]
  pub limit_price: Option<f64>,
  #[serde(deserialize_with = "to_optional_f64")]
  pub stop_price: Option<f64>,
  #[serde(deserialize_with = "to_optional_f64")]
  pub filled_avg_price: Option<f64>,
  pub status: OrderStatus,
  pub extended_hours: bool,
  pub legs: Option<Vec<Order>>,
}
