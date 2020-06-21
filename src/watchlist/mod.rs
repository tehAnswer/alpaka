use crate::{Alpaka, AlpakaError, NoBody};
use serde_json::{json, Value};

pub mod watchlist;
pub use watchlist::*;

// pub mod new_watchlist;
// pub use new_watchlist::*;

// pub mod add_asset;
// pub use add_asset::*;

pub struct Watchlists<'a> {
  alpaka: Box<&'a Alpaka>,
}

impl<'a> Watchlists<'a> {
  pub fn new(alpaka: Box<&'a Alpaka>) -> Watchlists<'a> {
    Watchlists { alpaka }
  }

  pub async fn all(&self) -> Result<Vec<Watchlist>, AlpakaError> {
    let client = *self.alpaka;
    client
      .get::<NoBody, Vec<Watchlist>>("v2/watchlists", &None, None)
      .await
  }
  pub async fn create(&self, name: &str, assets: &[String]) -> Result<Watchlist, AlpakaError> {
    let client = *self.alpaka;
    let json = json!({ "name": name.to_owned(), "symbols": assets.to_owned() });
    client
      .post::<Value, Watchlist>("v2/watchlists", &json, None)
      .await
  }
  pub async fn get(&self, id: &str) -> Result<Watchlist, AlpakaError> {
    let client = *self.alpaka;
    let path = format!("v2/watchlists/{}", id);
    client.get::<NoBody, Watchlist>(&path, &None, None).await
  }

  pub async fn update(
    &self,
    id: &str,
    name: &str,
    assets: &[String],
  ) -> Result<Watchlist, AlpakaError> {
    let client = *self.alpaka;
    let path = format!("v2/watchlists/{}", id);
    let json = json!({ "name": name.to_owned(), "symbols": assets.to_owned() });
    client.put::<Value, Watchlist>(&path, &json, None).await
  }

  pub async fn add_asset(&self, id: &str, symbol: &str) -> Result<Watchlist, AlpakaError> {
    let client = *self.alpaka;
    let path = format!("v2/watchlists/{}", id);
    let json = json!({ "symbol": symbol.to_owned() });
    client.post::<Value, Watchlist>(&path, &json, None).await
  }

  pub async fn remove_asset(&self, id: &str, asset: &str) -> Result<(), AlpakaError> {
    let client = *self.alpaka;
    let path = format!("v2/watchlists/{}/{}", id, asset);
    client
      .delete::<NoBody, serde_json::Value>(&path, &None, None)
      .await
      .map(|_| ())
  }

  pub async fn delete(&self, id: &str) -> Result<(), AlpakaError> {
    let client = *self.alpaka;
    let path = format!("v2/watchlists/{}", id);
    client
      .delete::<NoBody, NoBody>(&path, &None, None)
      .await
      .map(|_| ())
  }
}

#[cfg(test)]
mod tests {
  use crate::{Alpaka, AlpakaMode, Watchlist};
  use async_std::task;
  use mockito::{mock, Matcher};
  use serde::Serialize;

  fn to_json<T: Serialize>(data: &T) -> String {
    serde_json::to_string(data).unwrap()
  }

  #[test]
  fn watchlist_all_test() {
    let alpaka = Alpaka::new(
      String::from("api_key"),
      String::from("secret"),
      AlpakaMode::Paper,
    );

    let watchlists = alpaka.watchlists();
    let watchlist = Watchlist::default();
    let mockz = mock("GET", "/v2/watchlists")
      .match_query(Matcher::Exact(String::from("")))
      .with_body(to_json(&vec![watchlist]))
      .create();

    let result = task::block_on(async { watchlists.all().await });
    mockz.expect(1).assert();
    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn watchlist_create_test() {
    let alpaka = Alpaka::new(
      String::from("api_key"),
      String::from("secret"),
      AlpakaMode::Paper,
    );

    let watchlists = alpaka.watchlists();
    let watchlist = Watchlist::default();
    let mockz = mock("POST", "/v2/watchlists")
      .match_body("{\"name\":\"xd\",\"symbols\":[\"CCL\"]}")
      .with_body(to_json(&watchlist))
      .create();

    let result = task::block_on(async { watchlists.create("xd", &[String::from("CCL")]).await });
    mockz.expect(1).assert();
    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn watchlist_get_test() {
    let alpaka = Alpaka::new(
      String::from("api_key"),
      String::from("secret"),
      AlpakaMode::Paper,
    );

    let watchlists = alpaka.watchlists();
    let watchlist = Watchlist::default();
    let mockz = mock("GET", "/v2/watchlists/:id")
      .match_query(Matcher::Exact(String::from("")))
      .with_body(to_json(&watchlist))
      .create();

    let result = task::block_on(async { watchlists.get(":id").await });
    mockz.expect(1).assert();
    assert_eq!(result.is_ok(), true);
  }
  #[test]
  fn watchlist_update_test() {
    let alpaka = Alpaka::new(
      String::from("api_key"),
      String::from("secret"),
      AlpakaMode::Paper,
    );

    let watchlists = alpaka.watchlists();
    let watchlist = Watchlist::default();
    let mockz = mock("PUT", "/v2/watchlists/:id")
      .match_body("{\"name\":\"xd\",\"symbols\":[\"CCL\"]}")
      .with_body(to_json(&watchlist))
      .create();

    let result =
      task::block_on(async { watchlists.update(":id", "xd", &[String::from("CCL")]).await });
    mockz.expect(1).assert();
    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn watchlist_add_asset_test() {
    let alpaka = Alpaka::new(
      String::from("api_key"),
      String::from("secret"),
      AlpakaMode::Paper,
    );

    let watchlists = alpaka.watchlists();
    let watchlist = Watchlist::default();
    let mockz = mock("POST", "/v2/watchlists/:id")
      .match_body("{\"symbol\":\"CCL\"}")
      .with_body(to_json(&watchlist))
      .create();

    let result = task::block_on(async { watchlists.add_asset(":id", "CCL").await });
    mockz.expect(1).assert();
    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn watchlist_remove_asset_test() {
    let alpaka = Alpaka::new(
      String::from("api_key"),
      String::from("secret"),
      AlpakaMode::Paper,
    );

    let watchlists = alpaka.watchlists();
    let mockz = mock("DELETE", "/v2/watchlists/:id/CCL")
      .match_query(Matcher::Exact(String::from("")))
      .with_body("")
      .with_status(204)
      .create();

    let result = task::block_on(async { watchlists.remove_asset(":id", "CCL").await });
    mockz.expect(1).assert();
    println!("{:?}", result);
    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn watchlist_delete_test() {
    let alpaka = Alpaka::new(
      String::from("api_key"),
      String::from("secret"),
      AlpakaMode::Paper,
    );

    let watchlists = alpaka.watchlists();
    let mockz = mock("DELETE", "/v2/watchlists/:id")
      .match_query(Matcher::Exact(String::from("")))
      .with_body("")
      .with_status(204)
      .create();

    let result = task::block_on(async { watchlists.delete(":id").await });
    mockz.expect(1).assert();
    println!("{:?}", result);
    assert_eq!(result.is_ok(), true);
  }
}
