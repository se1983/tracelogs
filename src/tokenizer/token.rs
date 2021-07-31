use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct LogLineToken<'a> {
    gathered_timestamp: u128,
    // TODO: indices: vec[&str]
    log_source: &'a str,
    host_name: &'a str,
    // TODO: log_timestamp  -> if possible extract from data; default on gathered_timestamp
    pub raw_data: String,
}

impl<'a> LogLineToken<'a> {
    pub fn new(data: &str, log_source: &'a str, host_name: &'a str) -> LogLineToken<'a> {
        let gathered_timestamp = unixtime_now();
        let raw_data = String::from(data);
        LogLineToken {
            raw_data,
            gathered_timestamp,
            log_source,
            host_name,
        }
    }
}

fn unixtime_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros()
}
