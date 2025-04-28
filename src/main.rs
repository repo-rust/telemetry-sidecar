mod lib;
mod line_protocol;

use crate::lib::unix_domain_socket_path;
use crate::line_protocol::Measurement;
use anyhow::Context;
use std::path::Path;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UnixListener;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let metrics_socket_path = &unix_domain_socket_path();

    if Path::new(metrics_socket_path).exists() {
        tokio::fs::remove_file(metrics_socket_path)
            .await
            .context(format!(
                "Failed to remove socket file '{}'",
                metrics_socket_path
            ))?;
    }

    let listener = UnixListener::bind(&metrics_socket_path).context(format!(
        "Can't listen on socket file '{}'",
        metrics_socket_path
    ))?;

    println!(
        "Listening on Unix domain socket with path: {}",
        metrics_socket_path
    );

    // Spawn metrics publisher thread
    tokio::spawn(async move {
        println!("Metrics publisher started");
        loop {
            sleep(Duration::from_secs(10)).await;
            println!("Metrics publisher checking for new metrics to publish");
        }
    });

    loop {
        //
        // Accept Unix socket connection (we expect to have only one per sidecar) and handle all
        // write operations in the main thread
        //
        let (stream, _) = listener.accept().await.context("Can't accept connection")?;

        println!("New connection established");

        let reader = BufReader::new(stream);
        let mut lines = reader.lines();

        while let Ok(Some(mut line)) = lines.next_line().await {
            if line.ends_with('\n') {
                line.pop();
            }

            match Measurement::new(&line) {
                Ok(measurement) => {
                    println!("Received measurement: {:?}", measurement);
                }
                Err(error_msg) => {
                    println!("Error during measurement processing: {}", error_msg);
                }
            }
        }

        println!("Connection closed.");
    }
}
