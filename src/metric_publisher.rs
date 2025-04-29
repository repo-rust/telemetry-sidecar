use crate::db_utils::create_connection;
use crate::line_protocol::Metric;
use rusqlite::Connection;

pub struct MetricPublisher {
    conn: Connection,
}

impl MetricPublisher {
    const SELECT_METRICS_QUERY: &'static str =
        "SELECT id, name, tags, value, timestamp FROM metric";

    const DELETE_METRIC_BY_ID: &'static str = "DELETE FROM metric WHERE id = ?1";

    pub fn new() -> anyhow::Result<Self, anyhow::Error> {
        let conn = create_connection()?;
        Ok(Self { conn })
    }

    pub fn publish_new_metrics(&mut self) -> anyhow::Result<(), anyhow::Error> {
        println!("Metrics publisher checking for new metrics to publish");

        let mut stmt = self.conn.prepare(MetricPublisher::SELECT_METRICS_QUERY)?;

        let metrics = stmt.query_map([], |row| {
            Ok(Metric {
                id: row.get(0)?,
                name: row.get(1)?,
                tags: row.get(2)?,
                value: row.get(3)?,
                timestamp: Some(row.get(4)?),
            })
        })?;

        for single_metric_res in metrics {
            let metric = single_metric_res?;

            println!("metric from db: {:?}, sending to metrics server", metric);

            self.conn
                .execute(MetricPublisher::DELETE_METRIC_BY_ID, [metric.id])?;
        }

        Ok(())
    }
}
