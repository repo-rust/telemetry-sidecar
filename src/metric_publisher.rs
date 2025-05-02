use telemetry_sidecar::metric_dao::MetricDao;

pub(crate) struct MetricPublisher {
    dao: MetricDao,
}

impl MetricPublisher {
    pub(crate) fn new(dao: MetricDao) -> Self {
        Self { dao }
    }

    pub(crate) fn publish_new_metrics(&self) -> anyhow::Result<(), anyhow::Error> {
        println!("Metrics publisher checking for new metrics to publish");

        for single_metric in self.dao.list_metrics()? {
            //TODO: send a single metric to a metrics collector server using UDP as
            // a communication protocol

            println!(
                "metric from db: {:?}, sending to metrics server",
                single_metric
            );

            self.dao.delete_metric_by_id(
                single_metric
                    .id
                    .expect("Metric.id received from DB is None"),
            )?;
        }

        Ok(())
    }
}
