use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::client;

#[derive(Queryable, Insertable, Selectable, Deserialize, Serialize, Clone)]
#[diesel(table_name = client)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Client {
  #[diesel(sql_type = diesel::sql_types::Integer)]
  pub id: i32,
  #[diesel(sql_type = diesel::sql_types::Text)]
  pub name: String,
  #[diesel(sql_type = diesel::sql_types::Text)]
  pub description: String,
  #[diesel(sql_type = diesel::sql_types::Text)]
  pub modloader: String,
  #[diesel(sql_type = diesel::sql_types::Text)]
  pub version: String,
  #[diesel(sql_type = diesel::sql_types::Array<Nullable<Text>>)]
  pub mods: Vec<Option<String>>
}

#[derive(Queryable, Insertable, Selectable, Deserialize, Serialize, Clone)]
#[diesel(table_name = client)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClientAdd {
  #[diesel(sql_type = diesel::sql_types::Text)]
  name: String,
  #[diesel(sql_type = diesel::sql_types::Text)]
  description: String,
  #[diesel(sql_type = diesel::sql_types::Text)]
  modloader: String,
  #[diesel(sql_type = diesel::sql_types::Text)]
  version: String,
  #[diesel(sql_type = diesel::sql_types::Array<Nullable<Text>>)]
  mods: Vec<Option<String>>
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = client)]
pub struct ClientUpdate {
  description: Option<String>,
  modloader: Option<String>,
  version: Option<String>,
  mods: Option<Vec<Option<String>>>,
}

impl ClientAdd {
  pub fn with_id(self, id: i32) -> Client {
    Client {
      id,
      description: self.description,
      name: self.name,
      modloader: self.modloader,
      version: self.version,
      mods: self.mods
    }
  }
}