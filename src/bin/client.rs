use anyhow::Context;
use chrono::Utc;
use std::time::Duration;
use telemetry_sidecar::unix_domain_socket_path;
use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    println!("Client will be started with initial delay of 3 seconds...");
    sleep(Duration::from_secs(3)).await;

    let metrics_socket_path = &unix_domain_socket_path();

    let mut stream = UnixStream::connect(metrics_socket_path)
        .await
        .context("failed to connect to metrics server")?;

    /*
    https://github.com/prometheus/docs/blob/main/content/docs/instrumenting/exposition_formats.md
    */

    for i in 0..1_000_000 {
        // Unix timestamp in ms
        let timestamp_ms = Utc::now().timestamp_millis();

        let mut metric = format!(
            "http_requests_total{{method=\"post\",code=\"200\",region=\"us-ashburn-1\"}} 123 {}\n",
            timestamp_ms
        );

        if i % 10 == 0 {
            metric = format!(
                "{{method=\"post\",code=\"200\",region=\"us-ashburn-1\"}} 123 {}\n",
                timestamp_ms
            );

            stream
                .write_all(metric.as_bytes())
                .await
                .context("Failed to send message")?;

            println!("Bad metric {} sent", i + 1,);
        } else {
            stream
                .write_all(metric.as_bytes())
                .await
                .context("Failed to send message")?;

            println!("Metric {} sent", i + 1,);
        }

        sleep(Duration::from_secs(1)).await;
    }

    stream
        .shutdown()
        .await
        .context("Failed to shut down properly")?;

    println!("All metrics sent!!!");

    Ok(())
}
