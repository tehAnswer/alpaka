use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
  pub id: String,
  pub class: String,
  pub exchange: String,
  pub symbol: String,
  pub status: String,
  pub tradable: bool,
  pub marginable: bool,
  pub shortable: bool,
  pub easy_to_borrow: bool,
}
