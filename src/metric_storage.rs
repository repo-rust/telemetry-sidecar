use crate::line_protocol::Metric;
use crate::metric_dao::MetricDao;

pub(crate) struct MetricStorage {
    dao: MetricDao,
}

impl MetricStorage {
    pub(crate) fn new(dao: MetricDao) -> Self {
        Self { dao }
    }

    pub(crate) fn store_metric(&self, metric: Metric) -> anyhow::Result<(), anyhow::Error> {
        println!("Metric received: {:?}", metric);

        self.dao.insert_metric(metric)?;

        Ok(())
    }
}
