use crate::utils::alpaka_deserializers::to_datetime_utc;
use chrono::{DateTime, Utc};
use std::default::Default;

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(from = "RawQuote")]
pub struct Quote {
  pub symbol: String,
  pub ask_price: f64,
  pub ask_size: i64,
  pub ask_exchange: i64,
  pub bid_price: f64,
  pub bid_size: i64,
  pub bid_exchange: i64,
  pub timestamp: DateTime<Utc>,
}

impl Default for Quote {
  fn default() -> Quote {
    let timestamp = Utc::now();
    Quote {
      symbol: "".to_owned(),
      ask_price: 0.0,
      ask_size: 0,
      ask_exchange: 0,
      bid_price: 0.0,
      bid_size: 0,
      bid_exchange: 0,
      timestamp,
    }
  }
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct LastQuote {
  #[serde(rename = "askprice")]
  pub ask_price: f64,
  #[serde(rename = "asksize")]
  pub ask_size: i64,
  #[serde(rename = "askexchange")]
  pub ask_exchange: i64,
  #[serde(rename = "bidprice")]
  pub bid_price: f64,
  #[serde(rename = "bidsize")]
  pub bid_size: i64,
  #[serde(rename = "bidexchange")]
  pub bid_exchange: i64,
  #[serde(deserialize_with = "to_datetime_utc")]
  pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct RawQuote {
  pub symbol: String,
  pub last: LastQuote,
}

impl From<RawQuote> for Quote {
  fn from(value: RawQuote) -> Self {
    let RawQuote { symbol, last } = value;
    Self {
      symbol,
      ask_price: last.ask_price,
      ask_size: last.ask_size,
      ask_exchange: last.ask_exchange,
      bid_price: last.bid_price,
      bid_size: last.bid_size,
      bid_exchange: last.bid_exchange,
      timestamp: last.timestamp,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::Quote;
  use chrono::prelude::*;
  #[test]
  fn test_deserialize_quote() {
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

    let result: Result<Quote, serde_json::Error> = serde_json::from_str(data);
    assert_eq!(result.is_ok(), true);

    let value = result.unwrap();

    assert_eq!(&value.symbol, "SPY");
    assert_eq!(value.ask_price, 279.1);
    assert_eq!(value.ask_size, 1);
    assert_eq!(value.ask_exchange, 15);
    assert_eq!(value.bid_price, 287.05);
    assert_eq!(value.bid_size, 10);
    assert_eq!(value.bid_exchange, 17);
    assert_eq!(value.timestamp.year(), 2020);
  }
}
