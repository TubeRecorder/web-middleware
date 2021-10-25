use serde::{
  Deserialize,
  Serialize,
};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadEntry {
  pub entry_id: String,
  pub link_url: String,
  pub local_path: String,
  pub status: i32,
}
