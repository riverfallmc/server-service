use std::sync::Arc;

use adjust::{controllers, controller::Controller, database::{postgres::Postgres, Pool}, main, service::Service};
use controller::{server::ServerController, client::ClientController, monitoring::MonitoringController};

mod repository;
mod controller;
mod service;
mod models;
mod schema;

#[derive(Default, Clone)]
pub struct AppState {
  postgres: Arc<Pool<Postgres>>
}

// #[tokio::main]
// async fn main() -> HttpResult<()> {
//   setup()?;

//   let state = AppState {
//     postgres: establish_connection()?
//   };

//   // TODO вообще ряльно не оч красиво что оно тут загружается, но ладно
//   ClientService::load_client_list(&mut state.postgres.get()?)
//     .await?;

//   ServerService::load_server_list(&mut state.postgres.get()?)
//     .await?;

//   let router = Router::new()
//     .apply_controller(ServerController)
//     .apply_controller(ClientController)
//     .apply_controller(MonitoringController)
//     .with_state(state);

//   let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
//     .await?;

//   Ok(axum::serve(listener, router).await?)
// }

#[main]
async fn main() -> Service<'_, AppState> {
  Service {
    name: "Server",
    state: AppState::default(),
    controllers: controllers![ServerController, ClientController, MonitoringController],
    ..Default::default()
  }
}