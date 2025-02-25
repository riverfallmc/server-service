#![allow(dead_code)]

use adjust::{database::{postgres::Postgres, Database}, response::{HttpError, HttpMessage, HttpResult}};
use anyhow::anyhow;
use axum::{http::StatusCode, Json};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use crate::{models::server::{Server, ServerAdd, ServerUpdate}, repository::server::ServerRepository};
use super::client::ClientService;

pub type ServerList = Vec<Server>;

static SERVER_LIST: Lazy<Mutex<ServerList>> = Lazy::new(|| Mutex::new(ServerList::new()));

pub struct ServerService;

impl ServerService {
  /// Возвращает текущий список серверов
  pub async fn get_server_list() -> ServerList {
    // ну чёт копирование вектора ну чёт такооооое
    SERVER_LIST.lock().await.clone()
  }

  pub async fn get_server(
    id: i32
  ) -> HttpResult<Server> {
    let list = Self::get_server_list()
      .await;

    Ok(Json(list.get(id as usize)
      .ok_or_else(|| HttpError::new("Запрошенный сервер не был найден", Some(StatusCode::NOT_FOUND)))?.to_owned()))
  }

  pub async fn set_server_list(
    list: ServerList
  ) {
    *SERVER_LIST.lock().await = list;
  }

  pub async fn load_server_list(
    db: &mut Database<Postgres>
  ) -> HttpResult<()> {
    let list = ServerRepository::load(db)?;

    Self::set_server_list((*list).clone())
      .await;

    Ok(Json(()))
  }

  /// Добавляет сервер в мониторинг
  pub async fn add_server(
    db: &mut Database<Postgres>,
    server: ServerAdd
  ) -> HttpResult<HttpMessage> {
    let mut list = SERVER_LIST.lock()
      .await;

    #[allow(unused)]
    ClientService::client_exists(server.client.clone())
      .await?;

    let id = ServerRepository::add(db, &server)
      .map_err(|e| anyhow!("Не получилось добавить сервер: {e:?}"))?;

    list.push(server.with_id(*id));

    Ok(Json(HttpMessage::new(&format!("Сервер был успешно добавлен, и получил Id {}", *id))))
  }

  pub async fn update_server(
    db: &mut Database<Postgres>,
    id: i32,
    patch: ServerUpdate
  ) -> HttpResult<HttpMessage> {
    let mut list = SERVER_LIST.lock()
      .await;

    if patch.client.is_some() {
      #[allow(unused)]
      ClientService::client_exists(patch.client.clone().unwrap())
        .await?;
    }

    let server = ServerRepository::set(db, id, patch)?;

    if let Some(index) = list.iter().position(|v| v.id == id) {
      if let Some(value) = list.get_mut(index) {
        *value = (*server).clone();
      }
    }

    Ok(Json(HttpMessage::new("Сервер был успешно обновлён")))
  }

  pub async fn delete_server(
    db: &mut Database<Postgres>,
    id: i32,
  ) -> HttpResult<HttpMessage> {
    let mut list = SERVER_LIST.lock()
      .await;

    #[allow(unused)]
    ServerRepository::delete(db, id)
      .map_err(|e| anyhow!("Не получилось удалить сервер: {e:?}"))?;

    if let Some(index) = list.iter().position(|v| v.id == id) {
      list.remove(index);
    }

    Ok(Json(HttpMessage::new("Сервер был успешно удалён")))
  }
}