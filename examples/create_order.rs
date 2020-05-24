extern crate alpaka;
extern crate async_std;

use alpaka::{Alpaka, AlpakaError, AlpakaMode, NewOrder, Order};
use async_std::task;

fn main() -> Result<(), AlpakaError> {
  let api_key: String = std::env::var("API_KEY").unwrap();
  let secret_key: String = std::env::var("SECRET_KEY").unwrap();

  let alpaka = Alpaka::new(api_key, secret_key, AlpakaMode::Paper);
  let orders = alpaka.orders();

  let new_order = NewOrder {
    symbol: String::from("CCL"),
    qty: 1,
    side: "buy".parse().unwrap(),
    ..Default::default()
  };

  let result: Result<Order, AlpakaError> = task::block_on(async { orders.create(new_order).await });
  println!("{:?}", result);
  Ok(())
}
