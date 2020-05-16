use crate::{Alpaka, AlpakaError};
use serde_derive::{Deserialize, Serialize};

pub struct Orders<'a> {
  alpaka: Box<&'a Alpaka>,
}

#[derive(Serialize)]
pub enum Side {
  Sell,
  Buy,
}

#[derive(Serialize)]
// https://alpaca.markets/docs/trading-on-alpaca/orders/#time-in-force
pub enum TimeInForce {
  DAY,
  GTC,
  OPG,
  CLS,
  IOK,
  FOK,
}

#[derive(Deserialize)]
pub struct Order {}

#[derive(Serialize)]
pub struct TakeProfit {
  limit_price: f64,
}

#[derive(Serialize)]
pub struct StopLoss {
  stop_price: f64,
  limit_price: Option<f64>,
}

#[derive(Serialize)]
pub struct NewOrder {
  symbol: String,
  qty: i64,
  side: Side,
  time_in_force: TimeInForce,
  limit_price: Option<f64>,
  extended_hours: bool,
  take_profit: Option<TakeProfit>,
  stop_loss: Option<StopLoss>,
}

impl Orders<'_> {
  pub fn new(alpaka: Box<&Alpaka>) -> Orders {
    Orders { alpaka }
  }

  // pub fn retrieve(&self) -> Result<Vec<Order>, AlpakaError> {
  //   Ok(Vec::new())
  // }

  pub async fn create(&self, new_order: NewOrder) -> Result<Order, AlpakaError> {
    let client = *self.alpaka;
    client
      .post::<NewOrder, Order>("/v2/orders", &new_order, None)
      .await
  }
}
