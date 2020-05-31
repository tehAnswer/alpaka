use serde_derive::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccountStatus {
  #[serde(rename = "ONBOARDING")]
  Onboarding,
  #[serde(rename = "SUBMISSION_FAILED")]
  SubmissionFailed,
  #[serde(rename = "SUBMITTED")]
  Submitted,
  #[serde(rename = "ACCOUNT_UPDATED")]
  AccountUpdated,
  #[serde(rename = "APPROVAL_PENDING")]
  ApprovalPending,
  #[serde(rename = "ACTIVE")]
  Active,
  #[serde(rename = "REJECTED")]
  Rejected,
}

impl Default for AccountStatus {
  fn default() -> AccountStatus {
    AccountStatus::Onboarding
  }
}
