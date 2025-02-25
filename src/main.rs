use std::sync::Arc;

use adjust::{controllers, controller::Controller, database::{postgres::Postgres, Pool}, main, service::Service};
use controller::{server::ServerController, client::ClientController, monitoring::MonitoringController};
use service::{client::ClientService, server::ServerService};

mod repository;
mod controller;
mod service;
mod models;
mod schema;

#[derive(Default, Clone)]
pub struct AppState {
  postgres: Arc<Pool<Postgres>>
}

#[main]
async fn main() -> Service<'_, AppState> {
  let state = AppState::default();

  #[allow(unused)]
  ClientService::load_client_list(&mut state.postgres.get()?)
    .await.expect("failed to load client list");

  #[allow(unused)]
  ServerService::load_server_list(&mut state.postgres.get()?)
    .await.expect("failed to load client list");

  Service {
    name: "Server",
    state,
    controllers: controllers![ServerController, ClientController, MonitoringController],
    ..Default::default()
  }
}