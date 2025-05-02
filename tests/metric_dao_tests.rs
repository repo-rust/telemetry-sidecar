use chrono::Utc;
use rand::{rng, Rng};
use rusqlite::Connection;
use telemetry_sidecar::line_protocol::Metric;
use telemetry_sidecar::metric_dao::MetricDao;

#[test]
fn test_create_db_tables() {
    let conn = Connection::open_in_memory().unwrap();
    let dao = MetricDao::new(conn);
    assert!(dao.create_db_tables().is_ok());
}

#[test]
fn test_insert_metric() {
    let conn = Connection::open_in_memory().unwrap();
    let dao = MetricDao::new(conn);
    assert!(dao.create_db_tables().is_ok());

    assert!(
        dao.insert_metric(Metric {
            id: None,
            name: "http_requests_total".to_string(),
            tags: "method=\"post\",code=\"200\",region=\"us-ashburn-1\"".to_string(),
            value: "123".to_string(),
            timestamp: Some(1745825678238)
        })
        .is_ok()
    );
}

#[test]
fn test_list_metrics() {
    let conn = Connection::open_in_memory().unwrap();
    let dao = MetricDao::new(conn);
    assert!(dao.create_db_tables().is_ok());

    insert_random_metrics(&dao, 5);

    let metrics = dao.list_metrics().expect("Can't list metrics");

    assert_eq!(metrics.len(), 5);
}

#[test]
fn test_delete_metric_by_id() {
    let conn = Connection::open_in_memory().unwrap();
    let dao = MetricDao::new(conn);
    assert!(dao.create_db_tables().is_ok());

    insert_random_metrics(&dao, 5);

    let metrics = dao.list_metrics().expect("Can't list metrics");
    assert_eq!(metrics.len(), 5);

    assert!(dao.delete_metric_by_id(metrics[0].id.unwrap()).is_ok());
    assert!(dao.delete_metric_by_id(metrics[2].id.unwrap()).is_ok());

    let metrics = dao.list_metrics().expect("Can't list metrics");
    assert_eq!(metrics.len(), 3);
}

fn insert_random_metrics(dao: &MetricDao, metrics_count: u32) {
    let mut rand = rng();
    let timestamp_in_nanos = Utc::now().timestamp_nanos_opt().unwrap() as u64;

    for _ in 0..metrics_count {
        assert!(
            dao.insert_metric(Metric {
                id: None,
                name: "http_requests_total".to_string(),
                tags: "method='post',code='200',region='us-ashburn-1'".to_string(),
                value: rand.random_range(1..100).to_string(),
                timestamp: Some(timestamp_in_nanos)
            })
            .is_ok()
        );
    }
}
