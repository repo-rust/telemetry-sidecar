use anyhow::Context;
use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let metrics_socket_path = "/tmp/metrics.sock";

    let mut stream = UnixStream::connect(metrics_socket_path)
        .await
        .context("failed to connect to metrics server")?;

    /*
    https://docs.influxdata.com/influxdb/cloud/reference/syntax/line-protocol/
    */

    let metric = "http_requests_total{method=\"post\",code=\"200\",region=\"us-ashburn-1\"} 123 1745825678238\n";

    stream
        .write_all(metric.as_bytes())
        .await
        .context("failed to send message")?;

    stream.shutdown().await.context("failed to shutdown")?;

    println!("Message sent!");

    Ok(())
}
