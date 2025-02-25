use axum::{extract::{Path, State}, routing::{delete, get, patch, post}, Json};
use adjust::{controller::Controller, response::{HttpMessage, HttpResult}};
use crate::{models::server::{Server, ServerAdd, ServerUpdate}, service::server::{ServerList, ServerService}, AppState};

pub struct ServerController;

impl ServerController {
  async fn get_server_list() -> Json<ServerList> {
    Json(ServerService::get_server_list()
      .await)
  }

  async fn get_server(
    Path(id): Path<i32>
  ) -> HttpResult<Server> {
    ServerService::get_server(id)
      .await
  }

  async fn add_server(
    State(state): State<AppState>,
    Json(server): Json<ServerAdd>
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres
      .get()?;

    ServerService::add_server(&mut db, server)
      .await
  }

  async fn update_server(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(patch): Json<ServerUpdate>
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres.get()?;

    ServerService::update_server(&mut db, id, patch)
      .await
  }

  async fn delete_server(
    State(state): State<AppState>,
    Path(id): Path<i32>
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres.get()?;

    ServerService::delete_server(&mut db, id)
      .await
  }
}

impl Controller<AppState> for ServerController {
  fn register(&self, router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
      .route("/servers", get(Self::get_server_list))
      .route("/server/{id}", get(Self::get_server))
      // добавление сервера
      .route("/server", post(Self::add_server))
      // патч сервера
      .route("/server/{id}", patch(Self::update_server))
      // удаление сервера
      .route("/server/{id}", delete(Self::delete_server))
  }

  fn new() -> anyhow::Result<Box<Self>> {
    Ok(Box::new(Self))
  }
}