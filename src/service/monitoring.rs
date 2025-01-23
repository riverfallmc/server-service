#![allow(dead_code)]

use crate::models::Online;
use dixxxie::response::{HttpError, HttpResult};
use mc_query::status::status_with_timeout;
use super::server::ServerService;

/// Промежуток времени (в секундах), в которое поток мониторинга будет проверять все сервера
/// Для меньшей нагрузки можно поставить 20, 25, 30 секунд. Больше не советую
const MONITORING_CHECK_INTERVAL: u8 = 15;

pub struct MonitoringService {}

impl MonitoringService {
  //! Сервера

  async fn fetch_server_data(
    ip: String
  ) -> HttpResult<Online> {
    let endpoint = ip.split(":")
      .collect::<Vec<&str>>();

    if endpoint.is_empty() {
      return Err(HttpError::new("Строка endpoint не содержит IP", None))
    }

    let stats = status_with_timeout(endpoint.first().unwrap(), endpoint.get(1).unwrap_or(&"25565").parse()?, std::time::Duration::from_secs(5))
      .await?;

    Ok(Online {
      current: stats.players.online as i16,
      max: stats.players.max as i16
    })
  }

  /// Спавнит поток, который будет следить за серверами каждые MONITORING_CHECK_INTERVAL секунд
  pub fn spawn_monitoring_thread() {
    tokio::spawn(async move {
      loop {
        let mut server_list = ServerService::get_server_list().await;

        for server in server_list.iter_mut() {
          let updated = match Self::fetch_server_data(server.ip.clone()).await {
            Ok(data) => data,
            Err(_) => {
              server.enabled = false;
              continue;
            }
          };

          server.enabled = true;
          server.online = serde_json::to_value(updated).unwrap_or_default();
        }

        ServerService::set_server_list(server_list)
          .await;

        // спим haxooooy
        tokio::time::sleep(std::time::Duration::from_secs(MONITORING_CHECK_INTERVAL as u64)).await;
      }
    });
  }
}