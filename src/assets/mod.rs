use crate::{Alpaka, AlpakaError};

pub mod asset;
pub use asset::*;

use crate::NoBody;

pub struct Assets<'a> {
  alpaka: Box<&'a Alpaka>,
}

impl<'a> Assets<'a> {
  pub fn new(alpaka: Box<&'a Alpaka>) -> Assets<'a> {
    Assets { alpaka }
  }

  pub async fn all(&self) -> Result<Vec<Asset>, AlpakaError> {
    let client = *self.alpaka;
    client
      .get::<NoBody, Vec<Asset>>("v2/assets", &None, None)
      .await
  }

  pub async fn get(&self, id: &str) -> Result<Asset, AlpakaError> {
    let client = *self.alpaka;
    let path = format!("v2/assets/{}", id);
    client.get::<NoBody, Asset>(&path, &None, None).await
  }
}

#[cfg(test)]
mod tests {
  use crate::{Alpaka, AlpakaMode, Asset};
  use async_std::task;
  use mockito::{mock, Matcher};
  use serde::Serialize;

  fn to_json<T: Serialize>(data: &T) -> String {
    serde_json::to_string(data).unwrap()
  }

  #[test]
  fn test_assets_all() {
    let alpaka = Alpaka::new(
      String::from("api_key"),
      String::from("secret"),
      AlpakaMode::Paper,
    );

    let assets = alpaka.assets();
    let return_assets = vec![Asset::default()];
    let mockz = mock("GET", "/v2/assets")
      .match_query(Matcher::Exact(String::from("")))
      .with_body(to_json(&return_assets))
      .create();

    let result = task::block_on(async { assets.all().await });
    mockz.expect(1).assert();
    assert_eq!(result.is_ok(), true);
  }
  #[test]
  fn test_assets_get() {
    let alpaka = Alpaka::new(
      String::from("api_key"),
      String::from("secret"),
      AlpakaMode::Paper,
    );

    let assets = alpaka.assets();
    let return_asset = Asset {
      symbol: "V".to_string(),
      ..Default::default()
    };
    let mockz = mock("GET", "/v2/assets/V")
      .match_query(Matcher::Exact(String::from("")))
      .with_body(to_json(&return_asset))
      .create();

    let result = task::block_on(async { assets.get("V").await });
    mockz.expect(1).assert();
    assert_eq!(result.is_ok(), true);
  }
}
