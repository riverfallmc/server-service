#![allow(dead_code)]

use adjust::{database::{postgres::Postgres, Database}, response::{HttpError, HttpResult}};
use axum::{http::StatusCode, Json};
use diesel::{insert_into, upsert::excluded, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use crate::{models::server::{Server, ServerAdd, ServerUpdate}, schema::server, service::server::ServerList};

pub struct ServerRepository;

impl ServerRepository {
  pub fn add(
    db: &mut Database<Postgres>,
    server: &ServerAdd
  ) -> HttpResult<i32> {
    let result = insert_into(server::table)
      .values(server)
      .get_result::<Server>(db)?;

    Ok(Json(result.id))
  }

  pub fn get(
    db: &mut Database<Postgres>,
    id: i32
  ) -> HttpResult<Server> {
    Ok(Json(server::table
      .filter(server::columns::id.eq(id))
      .first::<Server>(db)?))
  }

  pub fn set(
    db: &mut Database<Postgres>,
    id: i32,
    patch: ServerUpdate
  ) -> HttpResult<Server> {
    Ok(Json(diesel::update(server::table.filter(server::id.eq(id)))
      .set(patch)
      .get_result(db)
      .map_err(|_| HttpError::new("Не получилось обновить сервер", Some(StatusCode::BAD_REQUEST)))?))
  }

  pub fn delete(
    db: &mut Database<Postgres>,
    id: i32
  ) -> HttpResult<usize> {
    Ok(Json(diesel::delete(server::table.filter(server::id.eq(id)))
      .execute(db)
      .map_err(|_| HttpError::new("Не получилось удалить сервер", Some(StatusCode::BAD_REQUEST)))?))
  }

  pub fn load(
    db: &mut Database<Postgres>,
  ) -> HttpResult<ServerList> {
    Ok(Json(server::table
      .select(Server::as_select())
      .load(db)?))
  }

  pub fn save(
    db: &mut Database<Postgres>,
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

    Ok(Json(()))
  }
}