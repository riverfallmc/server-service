use controller::{client::ClientController, monitoring::MonitoringController, server::ServerController};
use service::{client::ClientService, server::ServerService};
use dixxxie::{
  axum::{self, Router}, connection::{establish_connection, DbPool}, controller::ApplyControllerOnRouter, response::HttpResult, setup
};

mod repository;
mod controller;
mod service;
mod models;
mod schema;

#[allow(unused)]
#[derive(Clone)]
struct AppState {
  postgres: DbPool
}

#[tokio::main]
async fn main() -> HttpResult<()> {
  setup()?;

  let state = AppState {
    postgres: establish_connection()?
  };

  // TODO вообще ряльно не оч красиво что оно тут загружается, но ладно
  ClientService::load_client_list(&mut state.postgres.get()?)
    .await?;

  ServerService::load_server_list(&mut state.postgres.get()?)
    .await?;

  let router = Router::new()
    .apply_controller(ServerController)
    .apply_controller(ClientController)
    .apply_controller(MonitoringController)
    .with_state(state);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
    .await?;

  Ok(axum::serve(listener, router).await?)
}