#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Calendar {
  pub date: String,
  pub open: String,
  pub close: String,
  pub session_open: String,
  pub session_close: String,
}
