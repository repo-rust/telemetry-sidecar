mod line_protocol;

use crate::line_protocol::Measurement;
use anyhow::Context;
use std::path::Path;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UnixListener;
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let metrics_socket_path = &telemetry_sidecar::unix_domain_socket_path();

    if Path::new(metrics_socket_path).exists() {
        tokio::fs::remove_file(metrics_socket_path)
            .await
            .context(format!(
                "Failed to remove socket file '{}'",
                metrics_socket_path
            ))?;
    }

    let listener = UnixListener::bind(metrics_socket_path).context(format!(
        "Can't listen on socket file '{}'",
        metrics_socket_path
    ))?;

    println!(
        "Listening on Unix domain socket with path: {}",
        metrics_socket_path
    );

    let cancellation_token = CancellationToken::new();

    tokio::spawn(graceful_shutdown_listener(cancellation_token.clone()));

    tokio::spawn(metrics_publisher(cancellation_token.clone()));

    //
    // Accept Unix socket connection (we expect to have only one per sidecar) and handle all
    // write operations in the main thread
    //
    let (stream, _) = listener.accept().await.context("Can't accept connection")?;

    println!("New client connected");

    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    let cancellation_token_copy = cancellation_token.clone();
    loop {
        tokio::select! {
            _ = cancellation_token_copy.cancelled() => {
                println!("Main read loop interrupted");
                break;
            }
            result = lines.next_line() => {
                if let Ok(Some(mut line)) = result {
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
                else {
                    println!("Read loop error");
                    break;
                }
            }
        }
    }

    println!("Client connection closed.");

    Ok(())
}

///
/// Starts a background task that listens for the SIGTERM signal and notifies other tasks
/// to shut down using a CancellationToken.
///
async fn graceful_shutdown_listener(cancellation_token: CancellationToken) {
    println!("Graceful shutdown listener started");

    match signal(SignalKind::terminate()) {
        Ok(mut sigterm) => {
            sigterm.recv().await;
            println!("SIGTERM received!");
            cancellation_token.cancel();
        }
        Err(err) => {
            eprintln!("Unable to listen for SIGTERM signal: {}", err);
            // we also shut down in case of error
            cancellation_token.cancel();
        }
    }
}

///
/// Metrics publisher async task.
///
async fn metrics_publisher(cancellation_token: CancellationToken) {
    println!("Metrics publisher started");
    loop {
        tokio::select! {
            _ = cancellation_token.cancelled() => {
                println!("Metrics publisher interrupted");
                break;
            }
            _ = tokio::time::sleep(Duration::from_secs(10)) => {
                 println!("Metrics publisher checking for new metrics to publish");
            }
        }
    }
}
