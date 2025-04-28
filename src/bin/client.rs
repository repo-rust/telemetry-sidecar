use anyhow::Context;
use chrono::Utc;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let metrics_socket_path = "/tmp/metrics.sock";

    let mut stream = UnixStream::connect(metrics_socket_path)
        .await
        .context("failed to connect to metrics server")?;

    /*
    https://github.com/prometheus/docs/blob/main/content/docs/instrumenting/exposition_formats.md
    */

    for i in 0..5 {
        // Unix timestamp in ms
        let timestamp_ms = Utc::now().timestamp_millis();

        let mut metric = format!(
            "http_requests_total{{method=\"post\",code=\"200\",region=\"us-ashburn-1\"}} 123 {}\n",
            timestamp_ms
        );

        if i == 2 {
            metric = format!(
                "{{method=\"post\",code=\"200\",region=\"us-ashburn-1\"}} 123 {}\n",
                timestamp_ms
            );
        }
        stream
            .write_all(metric.as_bytes())
            .await
            .context("Failed to send message")?;

        println!(
            "{} metric {} sent",
            if i == 2 { "BAD" } else { "Normal" },
            i + 1,
        );

        sleep(Duration::from_secs(1)).await;
    }

    stream
        .shutdown()
        .await
        .context("Failed to shut down properly")?;

    println!("All metrics sent!!!");

    Ok(())
}
