use anyhow::Context;
use rusqlite::Connection;
use std::env;

pub(crate) fn create_connection() -> anyhow::Result<Connection, anyhow::Error> {
    let db_name = env::var("DATABASE_URL").context("'DATABASE_URL' not specified")?;

    println!("Using DATABASE_URL: {}", db_name);

    let conn = Connection::open(db_name.clone())
        .context(format!("Can't open connection to db {}", db_name))?;

    Ok(conn)
}
