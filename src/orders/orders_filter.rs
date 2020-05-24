#[derive(Serialize, Default, Debug)]
pub struct OrderFilter {
  pub status: Status,
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
