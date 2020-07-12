use crate::utils::NoBody;
use crate::{Alpaka, AlpakaError};

pub mod calendar;
pub use calendar::*;

pub struct Calendars<'a> {
  alpaka: Box<&'a Alpaka>,
}

impl<'a> Calendars<'a> {
  pub fn new(alpaka: Box<&'a Alpaka>) -> Calendars<'a> {
    Calendars { alpaka }
  }

  pub async fn get(&self) -> Result<Vec<Calendar>, AlpakaError> {
    let client = *self.alpaka;
    let path = "v2/calendar";
    client
      .get::<NoBody, Vec<Calendar>>(&path, &None, None)
      .await
  }
}

#[cfg(test)]
mod tests {
  use crate::{Alpaka, AlpakaMode, Calendar};
  use async_std::task;
  use mockito::{mock, Matcher};
  use serde::Serialize;

  fn to_json<T: Serialize>(data: &T) -> String {
    serde_json::to_string(data).unwrap()
  }
  #[test]
  fn test_calendar_get() {
    let alpaka = Alpaka::new(
      String::from("api_key"),
      String::from("secret"),
      AlpakaMode::Paper,
    );

    let calendars = alpaka.calendars();
    let return_calendar = Calendar::default();
    let mockz = mock("GET", "/v2/calendar")
      .match_query(Matcher::Exact(String::from("")))
      .with_body(to_json(&vec![return_calendar]))
      .create();

    let result = task::block_on(async { calendars.get().await });
    mockz.expect(1).assert();
    assert_eq!(result.is_ok(), true);
  }
}
