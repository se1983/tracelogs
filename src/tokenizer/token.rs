#[derive(Debug, Clone)]
pub struct LogLineToken {
    pub raw_data: String, // TODO: gathered_timestamp
                          // TODO: indices: vec[&str]
                          // TODO: log_source: &str
                          // TODO: log_timestamp  -> if possible extract from data; default on gathered_timestamp
}

impl LogLineToken {
    pub fn new(data: &str) -> Self {
        LogLineToken {
            raw_data: String::from(data),
        }
    }
}
