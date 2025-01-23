use serde::{Deserialize, Serialize};

pub mod client;
pub mod server;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Online {
  pub current: i16,
  pub max: i16
}