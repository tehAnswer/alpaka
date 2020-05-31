extern crate alpaka;
extern crate async_std;

use alpaka::{Account, Alpaka, AlpakaError, AlpakaMode};
use async_std::task;

fn main() -> Result<(), AlpakaError> {
  let api_key: String = std::env::var("API_KEY").unwrap();
  let secret_key: String = std::env::var("SECRET_KEY").unwrap();

  let alpaka = Alpaka::new(api_key, secret_key, AlpakaMode::Paper);
  let accounts = alpaka.accounts();

  let result: Result<Account, AlpakaError> = task::block_on(async { accounts.get().await });
  println!("{:#?}", result);
  Ok(())
}
