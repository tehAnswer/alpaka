use crate::{OrderStatus, OrderType};
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
  pub id: String,
  pub client_order_id: String,
  pub created_at: String,
  pub updated_at: String,
  pub submitted_at: String,
  pub filled_at: String,
  pub expired_at: String,
  pub canceled_at: String,
  pub failed_at: String,
  pub replaced_at: String,
  pub replaced_by: String,
  pub replaces: Option<String>,
  pub asset_id: String,
  pub symbol: String,
  pub asset_class: String,
  pub qty: String,
  pub filled_qty: String,
  #[serde(rename = "type")]
  pub order_type: OrderType,
  pub side: String,
  pub time_in_force: String,
  pub limit_price: String,
  pub stop_price: String,
  pub filled_avg_price: String,
  pub status: OrderStatus,
  pub extended_hours: bool,
  pub legs: Option<Vec<Order>>,
}
