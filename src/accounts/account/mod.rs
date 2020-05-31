use crate::utils::alpaka_deserializers::{to_f64, to_i64};
use crate::utils::alpaka_serializers::to_string;
use serde_derive::{Deserialize, Serialize};

pub mod account_status;
pub use account_status::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
  pub account_blocked: bool,
  pub account_number: String,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub buying_power: f64,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub cash: f64,
  pub created_at: String,
  pub currency: String,
  pub daytrade_count: i64,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub daytrading_buying_power: f64,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub equity: f64,
  pub id: String,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub initial_margin: f64,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub last_equity: f64,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub last_maintenance_margin: f64,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub long_market_value: f64,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub maintenance_margin: f64,
  #[serde(deserialize_with = "to_i64", serialize_with = "to_string")]
  pub multiplier: i64,
  pub pattern_day_trader: bool,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub portfolio_value: f64,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub regt_buying_power: f64,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub short_market_value: f64,
  pub shorting_enabled: bool,
  #[serde(deserialize_with = "to_f64", serialize_with = "to_string")]
  pub sma: f64,
  pub status: AccountStatus,
  pub trade_suspended_by_user: bool,
  pub trading_blocked: bool,
  pub transfers_blocked: bool,
}
