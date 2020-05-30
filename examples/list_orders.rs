extern crate alpaka;
extern crate async_std;

use alpaka::{Alpaka, AlpakaError, AlpakaMode, Order, OrdersFilter};
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

  task::block_on(async {
    let returned_orders: Vec<Order> = orders.all(Some(order_filters)).await.unwrap();
    println!("returned_orders.len() => {}", returned_orders.len());

    if let Some(first_order) = returned_orders.get(0) {
      let order: Order = orders.get(&first_order.id).await.unwrap();
      println!("{:#?}", order)
    }
  });
  Ok(())
}
