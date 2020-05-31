use crate::{Alpaka, AlpakaError};

pub mod account;
pub use account::*;

pub struct Accounts<'a> {
  alpaka: Box<&'a Alpaka>,
}

impl<'a> Accounts<'a> {
  pub fn new(alpaka: Box<&'a Alpaka>) -> Accounts<'a> {
    Accounts { alpaka }
  }

  pub async fn get(&self) -> Result<Account, AlpakaError> {
    let client = *self.alpaka;
    client
      .get::<Option<bool>, Account>("v2/account", &None, None)
      .await
  }
}

#[cfg(test)]
mod tests {
  use crate::{Account, Alpaka, AlpakaMode};
  use async_std::task;
  use mockito::{mock, Matcher};
  use serde::Serialize;

  fn to_json<T: Serialize>(data: &T) -> String {
    serde_json::to_string(data).unwrap()
  }

  #[test]
  fn test_orders_get() {
    let alpaka = Alpaka::new(
      String::from("api_key"),
      String::from("secret"),
      AlpakaMode::Paper,
    );

    let accounts = alpaka.accounts();
    let account = Account::default();
    let mockz = mock("GET", "/v2/account")
      .match_query(Matcher::Exact(String::from("")))
      .with_body(to_json(&account))
      .create();

    let result = task::block_on(async { accounts.get().await });
    mockz.expect(1).assert();
    assert_eq!(result.is_ok(), true);
  }
}
