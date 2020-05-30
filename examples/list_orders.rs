extern crate alpaka;
extern crate async_std;

use alpaka::{Alpaka, AlpakaError, AlpakaMode, OrdersFilter};
use async_std::task;

fn main() -> Result<(), AlpakaError> {
  let api_key: String = std::env::var("API_KEY").unwrap();
  let secret_key: String = std::env::var("SECRET_KEY").unwrap();

  let alpaka = Alpaka::new(api_key, secret_key, AlpakaMode::Paper);
  let orders = alpaka.orders();

  let order_filters = OrdersFilter {
    status: "closed".parse().unwrap(),
    ..Default::default()
  };

  task::block_on(async { println!("{:?}", orders.all(Some(order_filters)).await) });
  Ok(())
}
