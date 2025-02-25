use adjust::controller::Controller;
use crate::{service::monitoring::MonitoringService, AppState};

pub struct MonitoringController;

impl Controller<AppState> for MonitoringController {
  fn new() -> anyhow::Result<Box<Self>> {
    MonitoringService::spawn_monitoring_thread();

    Ok(Box::new(Self))
  }

  fn register(&self, router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
  }
}