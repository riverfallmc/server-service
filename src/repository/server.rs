#![allow(dead_code)]

use axum::http::StatusCode;
use diesel::{insert_into, upsert::excluded, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use dixxxie::{connection::DbPooled, response::{HttpError, HttpResult}};
use crate::{models::server::{Server, ServerAdd, ServerUpdate}, schema::server, service::server::ServerList};

pub struct ServerRepository;

impl ServerRepository {
  pub fn add(
    db: &mut DbPooled,
    server: &ServerAdd
  ) -> HttpResult<i32> {
    let result = insert_into(server::table)
      .values(server)
      .get_result::<Server>(db)?;

    Ok(result.id)
  }

  pub fn get(
    db: &mut DbPooled,
    id: i32
  ) -> HttpResult<Server> {
    Ok(server::table
      .filter(server::columns::id.eq(id))
      .first::<Server>(db)?)
  }

  pub fn set(
    db: &mut DbPooled,
    id: i32,
    patch: ServerUpdate
  ) -> HttpResult<Server> {
    diesel::update(server::table.filter(server::id.eq(id)))
      .set(patch)
      .get_result(db)
      .map_err(|_| HttpError::new("Не получилось обновить сервер", Some(StatusCode::BAD_REQUEST)))
  }

  pub fn delete(
    db: &mut DbPooled,
    id: i32
  ) -> HttpResult<usize> {
    diesel::delete(server::table.filter(server::id.eq(id)))
      .execute(db)
      .map_err(|_| HttpError::new("Не получилось удалить сервер", Some(StatusCode::BAD_REQUEST)))
  }

  pub fn load(
    db: &mut DbPooled,
  ) -> HttpResult<ServerList> {
    Ok(server::table
      .select(Server::as_select())
      .load(db)?)
  }

  pub fn save(
    db: &mut DbPooled,
    list: ServerList
  ) -> HttpResult<()> {
    insert_into(server::table)
      .values(list)
      .on_conflict(server::columns::id)
      .do_update()
      .set((
        // треш
        server::columns::client.eq(excluded(server::columns::client)),
        server::columns::enabled.eq(excluded(server::columns::enabled)),
        server::columns::ip.eq(excluded(server::columns::ip)),
        server::columns::name.eq(excluded(server::columns::name)),
        server::columns::online.eq(excluded(server::columns::online)),
      ))
      .execute(db)?;

    Ok(())
  }
}