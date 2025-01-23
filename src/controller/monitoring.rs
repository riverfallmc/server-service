use dixxxie::controller::Controller;
use crate::{service::monitoring::MonitoringService, AppState};

pub struct MonitoringController;

impl Controller<AppState> for MonitoringController {
  fn register(&self, router: axum::Router<AppState>) -> axum::Router<AppState> {
    MonitoringService::spawn_monitoring_thread();

    router
  }
}