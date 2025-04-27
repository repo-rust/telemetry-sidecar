use anyhow::Context;
use std::path::Path;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UnixListener;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let metrics_socket_path = "/tmp/metrics.sock";

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

    loop {
        let (stream, _) = listener.accept().await.context("Can't accept connection")?;

        println!("New connection started");

        tokio::spawn(async move {
            println!("New connection established");

            let reader = BufReader::new(stream);
            let mut lines = reader.lines();

            while let Ok(Some(mut line)) = lines.next_line().await {
                /*
                https://docs.influxdata.com/influxdb/cloud/reference/syntax/line-protocol/
                 */

                if line.ends_with('\n') {
                    line.pop();
                }

                println!("Received line: {}", line);
            }

            println!("Connection closed.");
        });
    }
}
