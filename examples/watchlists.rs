extern crate alpaka;
extern crate async_std;

use alpaka::{Alpaka, AlpakaError, AlpakaMode, Watchlist};
use async_std::task;

fn main() -> Result<(), AlpakaError> {
  let api_key: String = std::env::var("API_KEY").unwrap();
  let secret_key: String = std::env::var("SECRET_KEY").unwrap();

  let alpaka = Alpaka::new(api_key, secret_key, AlpakaMode::Paper);

  let watchlists = alpaka.watchlists();
  let symbols = &[String::from("CCL")];

  let all_result: Result<Vec<Watchlist>, AlpakaError> =
    task::block_on(async { watchlists.all().await });
  println!("{:?}", all_result);

  println!("xd");
  let create_result: Result<Watchlist, AlpakaError> =
    task::block_on(async { watchlists.create("xd", symbols).await });
  println!("{:?}", create_result);

  let watchlist = create_result.unwrap();
  let add_asset_result: Result<Watchlist, AlpakaError> =
    task::block_on(async { watchlists.add_asset(&watchlist.id, "GOOG").await });

  println!("{:?}", add_asset_result);

  let remove_asset_result: Result<(), AlpakaError> =
    task::block_on(async { watchlists.remove_asset(&watchlist.id, "GOOG").await });

  println!("{:?}", remove_asset_result);

  let delete_result: Result<(), AlpakaError> =
    task::block_on(async { watchlists.delete(&watchlist.id).await });
  println!("{:?}", delete_result);

  Ok(())
}
