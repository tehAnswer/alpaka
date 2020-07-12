use crate::utils::NoBody;
use crate::{Alpaka, AlpakaError};

pub mod clock_time;
pub use clock_time::*;

pub struct Clock<'a> {
  alpaka: Box<&'a Alpaka>,
}

impl<'a> Clock<'a> {
  pub fn new(alpaka: Box<&'a Alpaka>) -> Clock<'a> {
    Clock { alpaka }
  }

  pub async fn get(&self) -> Result<ClockTime, AlpakaError> {
    let client = *self.alpaka;
    let path = "v2/clock";
    client.get::<NoBody, ClockTime>(&path, &None, None).await
  }
}

#[cfg(test)]
mod tests {
  use crate::{Alpaka, AlpakaMode, ClockTime};
  use async_std::task;
  use mockito::{mock, Matcher};
  use serde::Serialize;

  fn to_json<T: Serialize>(data: &T) -> String {
    serde_json::to_string(data).unwrap()
  }
  #[test]
  fn test_clock_get() {
    let alpaka = Alpaka::new(
      String::from("api_key"),
      String::from("secret"),
      AlpakaMode::Paper,
    );

    let clock = alpaka.clock();
    let return_clock = ClockTime::default();
    let mockz = mock("GET", "/v2/clock")
      .match_query(Matcher::Exact(String::from("")))
      .with_body(to_json(&return_clock))
      .create();

    let result = task::block_on(async { clock.get().await });
    mockz.expect(1).assert();
    assert_eq!(result.is_ok(), true);
  }
}
