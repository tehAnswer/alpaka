use crate::utils::alpaka_deserializers::{to_f64, to_i64};
use crate::utils::alpaka_serializers::to_string;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
  pub asset_id: String,
  pub symbol: String,
  pub exchange: String,
  pub asset_class: String,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub avg_entry_price: f64,
  #[serde(deserialize_with = "to_i64", serialize_with = "to_string")]
  pub qty: i64,
  pub side: String,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub market_value: f64,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub cost_basis: f64,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub unrealized_pl: f64,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub unrealized_plpc: f64,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub unrealized_intraday_pl: f64,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub unrealized_intraday_plpc: f64,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub current_price: f64,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub lastday_price: f64,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub change_today: f64,
}
