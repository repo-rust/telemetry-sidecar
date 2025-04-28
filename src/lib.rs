use std::env;

pub fn unix_domain_socket_path() -> String {
    env::var("METRICS_UNIX_DOMAIN_SOCKET_PATH").unwrap_or("/tmp/metrics.sock".to_string())
}
