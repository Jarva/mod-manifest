use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::types::curseforge::enums::{FileReleaseType, FileStatus};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModFiles {
  pub data: Vec<File>,
  pub pagination: Pagination,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
  pub id: i64,
  pub game_id: i64,
  pub mod_id: i64,
  pub is_available: bool,
  pub display_name: String,
  pub file_name: String,
  pub release_type: FileReleaseType,
  pub file_status: FileStatus,
  pub file_date: DateTime<Utc>,
  pub game_versions: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
  pub index: i64,
  pub page_size: i64,
  pub result_count: i64,
  pub total_count: i64,
}
