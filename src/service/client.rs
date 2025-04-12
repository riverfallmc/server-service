#![allow(dead_code)]

use adjust::{database::{postgres::Postgres, Database}, response::{HttpError, HttpMessage, HttpResult}};
use anyhow::{anyhow, Context};
use axum::{http::StatusCode, Json};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use crate::{models::client::{Client, ClientAdd, ClientUpdate}, repository::client::ClientRepository};

pub type ClientList = Vec<Client>;

static CLIENT_LIST: Lazy<Mutex<ClientList>> = Lazy::new(|| Mutex::new(ClientList::new()));

pub struct ClientService;

impl ClientService {
  /// Возвращает текущий список серверов
  pub async fn get_client_list() -> ClientList {
    // ну чёт копирование вектора ну чёт такооооое
    CLIENT_LIST.lock().await.clone()
  }

  pub async fn get_client(
    id: i32
  ) -> HttpResult<Client> {
    let list = Self::get_client_list()
      .await;

    Ok(Json(list.get(id as usize)
      .ok_or_else(|| HttpError::new("Запрошенный клиент не был найден", Some(StatusCode::NOT_FOUND)))?.to_owned()))
  }

  pub async fn get_client_by_name(
    name: String
  ) -> HttpResult<Client> {
    let list = Self::get_client_list()
      .await;

    Ok(Json(list.iter().find(|v| v.name == name)
      .ok_or_else(|| HttpError::new("Запрошенный клиент не был найден", Some(StatusCode::NOT_FOUND)))?.to_owned()))
  }


  pub async fn client_exists(
    name: String
  ) -> HttpResult<Client> {
      // проверяем что клиент есть в бд
      // из нашего кэээша (не томми)
    ClientService::get_client_by_name(name)
      .await
      .map_err(|_| HttpError::new("Клиент не существует", Some(StatusCode::CONFLICT)))
  }

  pub async fn set_client_list(
    list: ClientList
  ) {
    *CLIENT_LIST.lock().await = list;
  }

  pub async fn load_client_list(
    db: &mut Database<Postgres>
  ) -> HttpResult<()> {
    let list = ClientRepository::load(db)?;

    Self::set_client_list(list.to_vec())
      .await;

    Ok(Json(()))
  }

  /// Добавляет клиент
  pub async fn add_client(
    db: &mut Database<Postgres>,
    client: ClientAdd
  ) -> HttpResult<HttpMessage> {
    let mut list = CLIENT_LIST.lock()
      .await;

    let id = ClientRepository::add(db, &client)
      .map_err(|e| anyhow!("Не получилось добавить клиент: {e:?}"))?;

    list.push(client.with_id(id));

    Ok(Json(HttpMessage::new(&format!("Клиент был успешно добавлен, и получил Id {id}"))))
  }

  pub async fn update_client(
    db: &mut Database<Postgres>,
    id: i32,
    patch: ClientUpdate
  ) -> HttpResult<HttpMessage> {
    let mut list = CLIENT_LIST.lock()
      .await;

    let client = ClientRepository::set(db, id, patch)
      .map_err(|e| anyhow!("Не получилось обновить клиент: {e:?}"))?;

    if let Some(index) = list.iter().position(|v| v.id == id) {
      if let Some(value) = list.get_mut(index) {
        *value = client.clone();
      }
    }

    Ok(Json(HttpMessage::new("Клиент был успешно обновлён")))
  }

  pub async fn delete_client(
    db: &mut Database<Postgres>,
    id: i32,
  ) -> HttpResult<HttpMessage> {
    let mut list = CLIENT_LIST.lock()
      .await;

    let name = list.iter().find(|client| client.id == id)
      .context(anyhow::anyhow!("Клиент не был найден в кэше"))?
      .clone()
      .name;

    // проверяем то, что этот клиент не использует ни один сервер
    if ClientRepository::find_uses(db, name)? > 0 {
      return Err(HttpError::new("Не получилось удалить клиент: он используется", Some(StatusCode::CONFLICT)))
    }

    #[allow(unused)]
    ClientRepository::delete(db, id)
      .map_err(|e| anyhow!("Не получилось удалить клиент: {e:?}"))?;

    if let Some(index) = list.iter().position(|v| v.id == id) {
      list.remove(index);
    }

    Ok(Json(HttpMessage::new("Клиент был успешно удалён")))
  }
}