use crate::db_utils::create_connection;
use crate::line_protocol::Metric;
use anyhow::Context;
use rusqlite::Connection;

pub struct MetricStorage {
    conn: Connection,
}

impl MetricStorage {
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

    pub fn new() -> anyhow::Result<Self, anyhow::Error> {
        let conn = create_connection()?;

        conn.execute(MetricStorage::DROP_METRIC_TABLE, [])
            .context("Can't drop 'metric' table")?;

        conn.execute(MetricStorage::CREATE_METRIC_TABLE, [])
            .context("Can't create 'metric' table")?;

        Ok(Self { conn })
    }

    pub fn store_metric(&self, value: Metric) -> anyhow::Result<(), anyhow::Error> {
        println!("Metric received: {:?}", value);

        self.conn.execute(
            MetricStorage::INSERT_METRIC_SQL,
            [
                value.get_name(),
                value.get_tags(),
                value.get_value(),
                value.get_timestamp().to_string(),
            ],
        )?;

        println!("Metric saved in SQLite");

        Ok(())
    }
}
