use crate::Asset;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Watchlist {
  pub id: String,
  pub account_id: String,
  pub created_at: String,
  pub updated_at: String,
  pub name: String,
  #[serde(default = "default_assets")]
  pub assets: Vec<Asset>,
}

fn default_assets() -> Vec<Asset> {
  vec![]
}
