use serde::{
  Deserialize,
  Serialize,
};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
  pub entry_id: String,
  pub link_url: String,
  pub local_path: String,
  pub status: i32,
}
