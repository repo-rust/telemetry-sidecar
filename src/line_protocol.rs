use anyhow::{bail, Context};
use std::str::FromStr;

///
/// InfluxDB
/// https://docs.influxdata.com/influxdb/v1/write_protocols/line_protocol_tutorial/
///
#[derive(Debug, PartialEq)]
pub struct Metric {
    pub id: Option<u64>,
    pub name: String,
    pub tags: String,
    pub value: String,
    pub timestamp: Option<u64>,
}

// TODO: remove below line in future
#[allow(dead_code)]
impl Metric {
    ///
    /// Create metric from a single line
    ///
    pub fn new(line: &str) -> anyhow::Result<Self, anyhow::Error> {
        // http_requests_total{method=\"post\",code=\"200\",region=\"us-ashburn-1\"} 123 174582567823

        let parts = line.split_whitespace().collect::<Vec<&str>>();

        if !(parts.len() == 2 || parts.len() == 3) {
            bail!("Can't construct metric from line '{}'", line);
        }

        let mut name = parts[0].to_string();
        let mut tags = String::new();

        if let Some(start) = name.find("{") {
            if let Some(end) = name.rfind("}") {
                tags.push_str(&name[start + 1..end]);
                name = name[0..start].to_string();

                if name.is_empty() {
                    bail!("Can't construct metric without name '{}'", line);
                }
            } else {
                bail!("Can't parse tags '{}'", line);
            }
        }

        let timestamp = if parts.len() == 3 {
            Some(u64::from_str(parts[2]).context("Can't parse timestamp")?)
        } else {
            None
        };

        let value = str::parse(parts[1]).context("Can't parse metric value as IntegerNumber")?;

        Ok(Self {
            id: None,
            name,
            tags,
            value,
            timestamp,
        })
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_tags(&self) -> String {
        self.tags.clone()
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp.unwrap_or(0)
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_metric_normal_case() {
        //TODO: change to "http_requests_total,region=us-ashburn-1,status=ok count=82 1465839830100400200"

        assert_eq!(
            Metric::new(
                "http_requests_total{method=\"post\",code=\"200\",region=\"us-ashburn-1\"} 123 1745825678238"
            ).unwrap(),
            Metric {
                id: None,
                name: "http_requests_total".to_string(),
                tags: "method=\"post\",code=\"200\",region=\"us-ashburn-1\"".to_string(),
                value: "123".to_string(),
                timestamp: Some(1745825678238)
            }
        );
    }

    #[test]
    fn new_metric_without_timestamp() {
        assert_eq!(
            Metric::new("http_requests_total{method=\"post\",code=\"404\"} 789").unwrap(),
            Metric {
                id: None,
                name: "http_requests_total".to_string(),
                tags: "method=\"post\",code=\"404\"".to_string(),
                value: "789".to_string(),
                timestamp: None
            }
        );
    }

    #[test]
    fn new_metric_without_tags() {
        assert_eq!(
            Metric::new("http_requests_total 123 1745825678238").unwrap(),
            Metric {
                id: None,
                name: "http_requests_total".to_string(),
                tags: String::new(),
                value: "123".to_string(),
                timestamp: Some(1745825678238)
            }
        );
    }

    #[test]
    fn new_metric_without_name_should_fail() {
        assert_eq!(
            Metric::new("{method=\"post\",code=\"200\"} 123 1745825678238")
                .unwrap_err()
                .to_string(),
            "Can't construct metric without name '{method=\"post\",code=\"200\"} 123 1745825678238'"
        );
    }

    #[test]
    fn new_metric_without_value_should_fail() {
        assert_eq!(
            Metric::new("http_requests_total{method=\"post\",code=\"200\"}")
                .unwrap_err()
                .to_string(),
            "Can't construct metric from line 'http_requests_total{method=\"post\",code=\"200\"}'"
        );
    }
}
