use crate::db_utils::create_connection;
use crate::line_protocol::Metric;
use anyhow::Context;
use rusqlite::Connection;

pub(crate) struct MetricDao {
    conn: Connection,
}

impl MetricDao {
    const DROP_METRIC_TABLE: &'static str = "DROP TABLE IF EXISTS metric";

    const CREATE_METRIC_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS metric (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        tags TEXT,
        value TEXT NOT NULL,
        timestamp NUMERIC
    )";

    const INSERT_METRIC_SQL: &'static str =
        "INSERT INTO metric (name, tags, value, timestamp) VALUES (?1, ?2, ?3, ?4)";

    const SELECT_METRICS_QUERY: &'static str =
        "SELECT id, name, tags, value, timestamp FROM metric";

    const DELETE_METRIC_BY_ID: &'static str = "DELETE FROM metric WHERE id = ?1";

    pub fn new() -> anyhow::Result<Self, anyhow::Error> {
        let conn = create_connection()?;
        Ok(Self { conn })
    }

    pub(crate) fn create_db_tables(&self) -> anyhow::Result<(), anyhow::Error> {
        let conn = create_connection()?;

        conn.execute(MetricDao::DROP_METRIC_TABLE, [])
            .context("Can't drop 'metric' table")?;

        conn.execute(MetricDao::CREATE_METRIC_TABLE, [])
            .context("Can't create 'metric' table")?;

        Ok(())
    }

    pub(crate) fn insert_metric(&self, metric: Metric) -> anyhow::Result<(), anyhow::Error> {
        self.conn.execute(
            MetricDao::INSERT_METRIC_SQL,
            [
                metric.get_name(),
                metric.get_tags(),
                metric.get_value(),
                metric.get_timestamp().to_string(),
            ],
        )?;

        println!("Metric saved in SQLite");

        Ok(())
    }

    pub(crate) fn list_metrics(&self) -> anyhow::Result<Vec<Metric>, anyhow::Error> {
        let mut stmt = self.conn.prepare(MetricDao::SELECT_METRICS_QUERY)?;

        let metrics = stmt.query_map([], |row| {
            Ok(Metric {
                id: row.get(0)?,
                name: row.get(1)?,
                tags: row.get(2)?,
                value: row.get(3)?,
                timestamp: Some(row.get(4)?),
            })
        })?;

        let mut metrics_vec = Vec::new();

        for single_metric_res in metrics {
            metrics_vec.push(single_metric_res?);
        }

        Ok(metrics_vec)
    }

    pub(crate) fn delete_metric_by_id(&self, metric_id: u64) -> anyhow::Result<(), anyhow::Error> {
        self.conn
            .execute(MetricDao::DELETE_METRIC_BY_ID, [metric_id])?;

        Ok(())
    }
}
