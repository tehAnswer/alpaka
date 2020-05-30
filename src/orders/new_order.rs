use crate::utils::alpaka_serializers::to_string;
use crate::{OrderType, Side, TimeInForce};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TakeProfit {
  pub limit_price: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StopLoss {
  pub stop_price: f64,
  pub limit_price: Option<f64>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NewOrder {
  pub symbol: String,
  #[serde(serialize_with = "to_string")]
  pub qty: i64,
  pub side: Side,
  pub time_in_force: TimeInForce,
  #[serde(rename = "type")]
  pub order_type: OrderType,
  pub limit_price: Option<f64>,
  pub extended_hours: bool,
  pub take_profit: Option<TakeProfit>,
  pub stop_loss: Option<StopLoss>,
}
