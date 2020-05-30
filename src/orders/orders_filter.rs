use crate::{OrdersDirectionFilter, OrdersStatusFilter};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct OrdersFilter {
  pub status: OrdersStatusFilter,
  pub limit: i64,
  pub after: Option<chrono::NaiveDateTime>,
  pub until: Option<chrono::NaiveDateTime>,
  pub direction: OrdersDirectionFilter,
  pub nested: bool,
}
