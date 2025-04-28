use crate::line_protocol::FieldValue::IntegerNumber;
use anyhow::{Context, bail};
use std::collections::HashMap;
use std::str::FromStr;

///
/// Prometheus measurement
/// https://github.com/prometheus/docs/blob/main/content/docs/instrumenting/exposition_formats.md
///
#[derive(Debug, PartialEq)]
pub struct Measurement {
    name: String,
    tags: HashMap<String, String>,
    value: FieldValue,
    timestamp: Option<u64>,
}

impl Measurement {
    ///
    /// Create measurement from single line
    ///
    pub fn new(line: &str) -> anyhow::Result<Self, anyhow::Error> {
        //TODO: pars here single line protocol value

        // http_requests_total{method=\"post\",code=\"200\",region=\"us-ashburn-1\"} 123 174582567823

        let parts = line.split_whitespace().collect::<Vec<&str>>();

        if !(parts.len() == 2 || parts.len() == 3) {
            bail!("Can't construct measurement from line '{}'", line);
        }

        let mut name = parts[0].to_string();
        let mut tags = HashMap::new();

        if let Some(start) = name.find("{") {
            if let Some(end) = name.rfind("}") {
                let tags_str = &name[start + 1..end];

                let tags_vec = tags_str.split(",").collect::<Vec<&str>>();

                for single_tag_str in tags_vec {
                    let mut tag_key_value = single_tag_str.split("=");
                    tags.insert(
                        tag_key_value.next().unwrap().to_string(),
                        Self::remove_double_quotes(tag_key_value.next().unwrap()).to_string(),
                    );
                }

                name = name[0..start].to_string();

                if name.is_empty() {
                    bail!("Can't construct measurement without name '{}'", line);
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

        let value =
            str::parse(parts[1]).context("Can't parse measurement value as IntegerNumber")?;

        Ok(Self {
            name,
            tags,
            value: IntegerNumber(value),
            timestamp,
        })
    }

    fn remove_double_quotes(s: &str) -> &str {
        if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
            &s[1..s.len() - 1]
        } else {
            s
        }
    }
}

#[derive(Debug, PartialEq)]
enum FieldValue {
    IntegerNumber(u64),
    // RealNumber(f64),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line_protocol::FieldValue::IntegerNumber;

    #[test]
    fn new_measurement_normal_case() {
        assert_eq!(
            Measurement::new(
                "http_requests_total{method=\"post\",code=\"200\",region=\"us-ashburn-1\"} 123 1745825678238"
            ).unwrap(),
            Measurement {
                name: "http_requests_total".to_string(),
                tags: HashMap::from([
                    ("method".to_string(), "post".to_string()),
                    ("code".to_string(), "200".to_string()),
                    ("region".to_string(), "us-ashburn-1".to_string())
                ]),
                value: IntegerNumber(123),
                timestamp: Some(1745825678238)
            }
        );
    }

    #[test]
    fn new_measurement_without_timestamp() {
        assert_eq!(
            Measurement::new("http_requests_total{method=\"post\",code=\"404\"} 789").unwrap(),
            Measurement {
                name: "http_requests_total".to_string(),
                tags: HashMap::from([
                    ("method".to_string(), "post".to_string()),
                    ("code".to_string(), "404".to_string()),
                ]),
                value: IntegerNumber(789),
                timestamp: None
            }
        );
    }

    #[test]
    fn new_measurement_without_tags() {
        assert_eq!(
            Measurement::new("http_requests_total 123 1745825678238").unwrap(),
            Measurement {
                name: "http_requests_total".to_string(),
                tags: HashMap::new(),
                value: IntegerNumber(123),
                timestamp: Some(1745825678238)
            }
        );
    }

    #[test]
    fn new_measurement_without_name_should_fail() {
        assert_eq!(
            Measurement::new("{method=\"post\",code=\"200\"} 123 1745825678238")
                .unwrap_err()
                .to_string(),
            "Can't construct measurement without name '{method=\"post\",code=\"200\"} 123 1745825678238'"
        );
    }

    #[test]
    fn new_measurement_without_value_should_fail() {
        assert_eq!(
            Measurement::new("http_requests_total{method=\"post\",code=\"200\"}")
                .unwrap_err()
                .to_string(),
            "Can't construct measurement from line 'http_requests_total{method=\"post\",code=\"200\"}'"
        );
    }
}
