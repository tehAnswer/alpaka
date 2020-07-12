use crate::utils::NoBody;
use crate::{Alpaka, AlpakaError};

pub mod quote;
pub use quote::*;

pub struct Quotes<'a> {
  alpaka: Box<&'a Alpaka>,
}

impl<'a> Quotes<'a> {
  pub fn new(alpaka: Box<&'a Alpaka>) -> Quotes<'a> {
    Quotes { alpaka }
  }

  pub async fn get(&self, symbol: &str) -> Result<Quote, AlpakaError> {
    let client = *self.alpaka;
    let path = format!("v1/last_quote/stocks/{}", symbol);
    client
      .get::<NoBody, Quote>(&path, &None, Some("data"))
      .await
  }
}

#[cfg(test)]
mod tests {
  use crate::{Alpaka, AlpakaMode};
  use async_std::task;
  use mockito::{mock, Matcher};

  #[test]
  fn test_quotes_get() {
    let alpaka = Alpaka::new(
      String::from("api_key"),
      String::from("secret"),
      AlpakaMode::Paper,
    );

    let data = stringify!({
        "status": "success",
        "symbol": "SPY",
        "last": {
          "askprice": 279.1,
          "asksize": 1,
          "askexchange": 15,
          "bidprice": 287.05,
          "bidsize": 10,
          "bidexchange": 17,
          "timestamp": 1588770424970329400
        }
    });

    let quotes = alpaka.quotes();
    let mockz = mock("GET", "/v1/last_quote/stocks/CCL")
      .match_query(Matcher::Exact(String::from("")))
      .with_body(data)
      .create();

    let result = task::block_on(async { quotes.get("CCL").await });
    mockz.expect(1).assert();
    assert_eq!(result.is_ok(), true);
  }
}
