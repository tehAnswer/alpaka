use crate::utils::NoBody;
use crate::{Alpaka, AlpakaError, Order};

pub mod position;
pub use position::*;

pub struct Positions<'a> {
  alpaka: Box<&'a Alpaka>,
}

impl<'a> Positions<'a> {
  pub fn new(alpaka: Box<&'a Alpaka>) -> Positions<'a> {
    Positions { alpaka }
  }

  pub async fn get(&self, id: &str) -> Result<Position, AlpakaError> {
    let client = *self.alpaka;
    let path = format!("v2/positions/{}", id);
    client.get::<NoBody, Position>(&path, &None, None).await
  }

  pub async fn all(&self) -> Result<Vec<Position>, AlpakaError> {
    let client = *self.alpaka;
    client
      .get::<NoBody, Vec<Position>>("v2/positions", &None, None)
      .await
  }

  pub async fn close_all(&self) -> Result<(), AlpakaError> {
    let client = *self.alpaka;
    client
      .delete::<NoBody, serde_json::Value>("v2/positions", &None, None)
      .await
      .map(|_| ())
  }

  pub async fn close(&self, id: &str) -> Result<Order, AlpakaError> {
    let client = *self.alpaka;
    let path = format!("v2/positions/{}", id);
    client.delete::<NoBody, Order>(&path, &None, None).await
  }
}

#[cfg(test)]
mod tests {
  use crate::{Alpaka, AlpakaMode, Order, Position};
  use async_std::task;
  use mockito::{mock, Matcher};
  use serde::Serialize;

  fn to_json<T: Serialize>(data: &T) -> String {
    serde_json::to_string(data).unwrap()
  }

  #[test]
  fn test_positions_get() {
    let alpaka = Alpaka::new(
      String::from("api_key"),
      String::from("secret"),
      AlpakaMode::Paper,
    );

    let positions = alpaka.positions();
    let position = Position::default();
    let mockz = mock("GET", "/v2/positions/:id")
      .match_query(Matcher::Exact(String::from("")))
      .with_body(to_json(&position))
      .create();

    let result = task::block_on(async { positions.get(":id").await });
    mockz.expect(1).assert();
    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn test_positions_close() {
    let alpaka = Alpaka::new(
      String::from("api_key"),
      String::from("secret"),
      AlpakaMode::Paper,
    );

    let positions = alpaka.positions();
    let order = Order::default();
    let mockz = mock("DELETE", "/v2/positions/:id")
      .match_query(Matcher::Exact(String::from("")))
      .with_body(to_json(&order))
      .create();

    let result = task::block_on(async { positions.close(":id").await });
    mockz.expect(1).assert();
    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn test_positions_close_all() {
    let alpaka = Alpaka::new(
      String::from("api_key"),
      String::from("secret"),
      AlpakaMode::Paper,
    );

    let positions = alpaka.positions();
    let mockz = mock("DELETE", "/v2/positions")
      .match_query(Matcher::Exact(String::from("")))
      .with_body("{}")
      .create();

    let result = task::block_on(async { positions.close_all().await });
    mockz.expect(1).assert();
    println!("{:?}", result);
    assert_eq!(result.is_ok(), true);
  }
}
