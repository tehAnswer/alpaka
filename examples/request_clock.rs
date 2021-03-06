extern crate alpaka;
extern crate async_std;

use alpaka::{Alpaka, AlpakaError, AlpakaMode, ClockTime};
use async_std::task;

fn main() -> Result<(), AlpakaError> {
  let api_key: String = std::env::var("API_KEY").unwrap();
  let secret_key: String = std::env::var("SECRET_KEY").unwrap();

  let alpaka = Alpaka::new(api_key, secret_key, AlpakaMode::Paper);
  let clock = alpaka.clock();

  let result: Result<ClockTime, AlpakaError> = task::block_on(async { clock.get().await });
  println!("{:?}", result);
  Ok(())
}
