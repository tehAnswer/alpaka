use crate::{OrderType, Side, TimeInForce};
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct TakeProfit {
  pub limit_price: f64,
}

#[derive(Serialize)]
pub struct StopLoss {
  pub stop_price: f64,
  pub limit_price: Option<f64>,
}

#[derive(Serialize, Default)]
pub struct NewOrder {
  pub symbol: String,
  pub qty: i64,
  pub side: Side,
  pub time_in_force: TimeInForce,
  pub order_type: OrderType,
  pub limit_price: Option<f64>,
  pub extended_hours: bool,
  pub take_profit: Option<TakeProfit>,
  pub stop_loss: Option<StopLoss>,
}
