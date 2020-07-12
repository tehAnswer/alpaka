use chrono::prelude::*;
use chrono::NaiveDate;
use std::default::Default;

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Calendar {
  pub date: NaiveDate,
  pub open: String,
  pub close: String,
  pub session_open: String,
  pub session_close: String,
}

impl Default for Calendar {
  fn default() -> Calendar {
    let date = Utc::today().naive_utc();
    Calendar {
      date,
      open: "00:00".to_owned(),
      close: "23:59".to_owned(),
      session_open: "0000".to_owned(),
      session_close: "2359".to_owned(),
    }
  }
}
