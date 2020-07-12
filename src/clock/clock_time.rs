use chrono::{DateTime, Utc};
use std::default::Default;

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ClockTime {
  pub timestamp: DateTime<Utc>,
  pub is_open: bool,
  pub next_open: DateTime<Utc>,
  pub next_close: DateTime<Utc>,
}

impl Default for ClockTime {
  fn default() -> ClockTime {
    let timestamp = Utc::now();

    ClockTime {
      timestamp,
      next_open: timestamp.clone(),
      next_close: timestamp.clone(),
      is_open: false,
    }
  }
}
