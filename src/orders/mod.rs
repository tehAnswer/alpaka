pub mod order;
pub use order::*;

pub mod new_order;
pub use new_order::*;

pub mod order_type;
pub use order_type::*;

pub mod order_status;
pub use order_status::*;

pub mod side;
pub use side::*;

pub mod time_in_force;
pub use time_in_force::*;

use crate::{Alpaka, AlpakaError};

pub struct Orders<'a> {
  alpaka: Box<&'a Alpaka>,
}

impl<'a> Orders<'a> {
  pub fn new(alpaka: Box<&'a Alpaka>) -> Orders<'a> {
    Orders { alpaka }
  }

  // pub fn retrieve(&self) -> Result<Vec<Order>, AlpakaError> {
  //   Ok(Vec::new())
  // }

  pub async fn create(&self, new_order: NewOrder) -> Result<Order, AlpakaError> {
    let client = *self.alpaka;
    client
      .post::<NewOrder, Order>("v2/orders", &new_order, None)
      .await
  }
}

#[cfg(test)]
mod tests {
  use crate::{Alpaka, AlpakaMode, NewOrder, Order};
  use async_std::task;
  use mockito::{mock, Matcher};
  use serde::Serialize;

  fn create_new_order(symbol: &str, qty: i64) -> NewOrder {
    NewOrder {
      symbol: symbol.to_string(),
      qty,
      ..Default::default()
    }
  }

  fn to_json<T: Serialize>(data: &T) -> String {
    serde_json::to_string(data).unwrap()
  }

  #[test]
  fn test_orders_create() {
    let alpaka = Alpaka::new(
      String::from("api_key"),
      String::from("secret"),
      AlpakaMode::Paper,
    );
    let orders = alpaka.orders();

    let new_order = create_new_order("GOOG", 2);
    let returned_order = Order::default();
    let mockz = mock("POST", "/v2/orders")
      .match_body(Matcher::Exact(to_json(&new_order)))
      .with_body(to_json(&returned_order))
      .create();

    let result = task::block_on(async { orders.create(new_order).await });
    mockz.expect(1).assert();
    assert_eq!(result.is_ok(), true);
  }
}
