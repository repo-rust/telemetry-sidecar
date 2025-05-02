use std::env;

pub mod metric_dao;

pub mod db_utils;

pub mod line_protocol;

pub fn unix_domain_socket_path() -> String {
    env::var("METRICS_UNIX_DOMAIN_SOCKET_PATH").unwrap_or("/tmp/metrics.sock".to_string())
}
