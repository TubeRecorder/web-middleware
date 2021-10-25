use serde::{
  Deserialize,
  Serialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigEntry {
  pub max_download_connections: u16,
  pub download_period_mins: u16,
}
