use axum::{extract::{Path, Query, State}, routing::{delete, get, patch, post}, Json};
use adjust::{controller::Controller, response::{HttpMessage, HttpResult}};
use serde::Deserialize;
use crate::{models::client::{Client, ClientAdd, ClientUpdate}, service::client::{ClientList, ClientService}, AppState};

#[derive(Deserialize)]
struct NameQuery {
  name: String
}

pub struct ClientController;

impl ClientController {
  async fn get_client_list() -> Json<ClientList> {
    Json(ClientService::get_client_list()
      .await)
  }

  async fn get_client(
    Path(id): Path<i32>,
  ) -> HttpResult<Client> {
    ClientService::get_client(id)
      .await
  }

  async fn get_client_by_name(
    Query(query): Query<NameQuery>
  ) -> HttpResult<Client> {
    ClientService::get_client_by_name(query.name)
      .await
  }

  async fn add_client(
    State(state): State<AppState>,
    Json(client): Json<ClientAdd>
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres
      .get()?;

    ClientService::add_client(&mut db, client)
      .await
  }

  async fn update_server(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(patch): Json<ClientUpdate>
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres.get()?;

    ClientService::update_client(&mut db, id, patch)
      .await
  }

  async fn delete_server(
    State(state): State<AppState>,
    Path(id): Path<i32>
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres.get()?;

    ClientService::delete_client(&mut db, id)
      .await
  }
}

impl Controller<AppState> for ClientController {
  fn new() -> anyhow::Result<Box<Self>> {
    Ok(Box::new(Self))
  }

  fn register(&self, router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
      .route("/clients", get(Self::get_client_list))
      .route("/client/{id}", get(Self::get_client))
      .route("/client", get(Self::get_client_by_name))
      // добавление клиента
      .route("/client", post(Self::add_client))
      // патч клиента
      .route("/client/{id}", patch(Self::update_server))
      // удаление клиента
      .route("/client/{id}", delete(Self::delete_server))
  }
}