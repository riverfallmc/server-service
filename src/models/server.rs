use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::server;
use super::Online;

#[derive(Queryable, QueryableByName, Selectable, Insertable, Deserialize, Serialize, Clone)]
#[diesel(table_name = server)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Server {
  #[diesel(sql_type = diesel::sql_types::Integer)]
  pub id: i32,
  #[diesel(sql_type = diesel::sql_types::Text)]
  pub name: String,
  #[diesel(sql_type = diesel::sql_types::Bool)]
  pub enabled: bool,
  #[diesel(sql_type = diesel::sql_types::Text)]
  pub client: String,
  #[diesel(sql_type = diesel::sql_types::Jsonb)]
  pub online: serde_json::Value,
  #[diesel(sql_type = diesel::sql_types::Text)]
  pub ip: String
}

#[derive(Insertable, Deserialize, Serialize, Clone)]
#[diesel(table_name = server)]
pub struct ServerAdd {
  #[diesel(sql_type = diesel::sql_types::Text)]
  pub name: String,
  #[serde(skip_deserializing, default = "default_enabled")]
  #[diesel(sql_type = diesel::sql_types::Bool)]
  pub enabled: bool,
  #[diesel(sql_type = diesel::sql_types::Text)]
  pub client: String,
  #[diesel(sql_type = diesel::sql_types::Text)]
  pub ip: String,
  #[serde(skip_deserializing, default = "default_online")]
  #[diesel(sql_type = diesel::sql_types::Jsonb)]
  pub online: serde_json::Value,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = server)]
pub struct ServerUpdate {
  pub name: Option<String>,
  pub client: Option<String>,
  pub ip: Option<String>,
}

fn default_enabled() -> bool { false }

fn default_online() -> serde_json::Value {
  serde_json::to_value(Online {
    current: 0,
    max: 0
  }).unwrap()
}

impl ServerAdd {
  pub fn with_id(self, id: i32) -> Server {
    Server {
      id,
      name: self.name,
      enabled: self.enabled,
      client: self.client,
      online: self.online,
      ip: self.ip
    }
  }
}