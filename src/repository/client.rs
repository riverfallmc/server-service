#![allow(dead_code)]

use axum::http::StatusCode;
use diesel::{dsl::count, insert_into, upsert::excluded, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use dixxxie::{connection::DbPooled, response::{HttpError, HttpResult}};
use crate::{models::client::{Client, ClientAdd, ClientUpdate}, schema::{client, server}, service::client::ClientList};

pub struct ClientRepository;

impl ClientRepository {
  pub fn add(
    db: &mut DbPooled,
    client: &ClientAdd
  ) -> HttpResult<i32> {
    let result = insert_into(client::table)
      .values(client)
      .get_result::<Client>(db)?;

    Ok(result.id)
  }

  pub fn get(
    db: &mut DbPooled,
    id: i32
  ) -> HttpResult<Client> {
    Ok(client::table
      .filter(client::columns::id.eq(id))
      .first::<Client>(db)?)
  }

  pub fn set(
    db: &mut DbPooled,
    id: i32,
    patch: ClientUpdate
  ) -> HttpResult<Client> {
    diesel::update(client::table.filter(client::id.eq(id)))
      .set(patch)
      .get_result(db)
      .map_err(|_| HttpError::new("Не получилось обновить сервер", Some(StatusCode::BAD_REQUEST)))
  }

  pub fn delete(
    db: &mut DbPooled,
    id: i32
  ) -> HttpResult<usize> {
    diesel::delete(client::table.filter(client::id.eq(id)))
      .execute(db)
      .map_err(|_| HttpError::new("Не получилось удалить сервер", Some(StatusCode::BAD_REQUEST)))
  }

  pub fn load(
    db: &mut DbPooled,
  ) -> HttpResult<ClientList> {
    Ok(client::table
      .select(Client::as_select())
      .load(db)?)
  }

  pub fn save(
    db: &mut DbPooled,
    list: ClientList
  ) -> HttpResult<()> {
    insert_into(client::table)
      .values(list)
      .on_conflict(client::columns::id)
      .do_update()
      .set((
        // треш
        client::columns::modloader.eq(excluded(client::columns::modloader)),
        client::columns::mods.eq(excluded(client::columns::mods)),
        client::columns::name.eq(excluded(client::columns::name)),
        client::columns::version.eq(excluded(client::columns::version)),
      ))
      .execute(db)?;

    Ok(())
  }

  pub fn find_uses(
    db: &mut DbPooled,
    client: String
  ) -> HttpResult<i64> {
    Ok(server::table
      .filter(server::client.eq(client))
      .select(count(server::client))
      .first(db)?)
  }
}