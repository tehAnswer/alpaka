extern crate alpaka;
extern crate async_std;

use alpaka::{Alpaka, AlpakaError, AlpakaMode};
use async_std::task;

fn main() -> Result<(), AlpakaError> {
  let api_key: String = std::env::var("API_KEY").unwrap();
  let secret_key: String = std::env::var("SECRET_KEY").unwrap();

  let alpaka = Alpaka::new(api_key, secret_key, AlpakaMode::Paper);
  let assets = alpaka.assets();

  task::block_on(async {
    let result = assets.all().await;
    if let Ok(returned_assets) = result {
      println!("assets: {}", returned_assets.len());
      if let Some(asset) = returned_assets.get(0) {
        let asset_result = assets.get(&asset.symbol).await;
        println!("{:#?}", asset_result);
      } else {
        println!("No assets found");
      }
    } else {
    }
  });
  Ok(())
}
