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
    let metric = "cpu,region=us-ashburn-1 usage=0.5 1556813561098000000\n";

    stream
        .write_all(metric.as_bytes())
        .await
        .context("failed to send message")?;

    stream.shutdown().await.context("failed to shutdown")?;

    println!("Message sent!");

    Ok(())
}
