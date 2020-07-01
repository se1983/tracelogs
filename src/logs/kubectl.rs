use chrono::NaiveDateTime;
use regex::Regex;

use crate::logs::Tracer;

pub struct KubeLogLine {
    timestamp: i64,
    hostname: String,
    service: Option<String>,
    pub(crate) message: String,
}



struct KubeLog {
    lines: Vec<KubeLogLine>,
    line_idx: usize,
}

impl KubeLog {
    fn new() -> Self {

        let datetime = r"[0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[1-2][0-9]|3[0-1]) (2[0-3]|[01][0-9]):[0-5][0-9]:[0-5][0-9],[0-9][0-9][0-9]";
        let host = r"([^\s]+)";
        let service = r"([^\s]+)";
        let message = r"(.*)";

        let log_pattern = format!("(?P<datetime>({d})) (?P<hostname>({h}))] (?P<service>({s})) (?P<message>({m}))",
                                  d = datetime, h = host, s = service, m = message);

        let re = Regex::new(&log_pattern).unwrap();

        let log_line = "[2006-02-08 22:20:02,165 192.168.0.1] fbloggs  Protocol problem: connection reset";
    }
}

impl Iterator for KubeLog {
    type Item = KubeLogLine;
    fn next(&mut self) -> Option<Self::Item> {
        if self.line_idx >= self.lines.len() { return None; }
        let line = self.lines[self.line_idx].clone();
        self.line_idx += 1;
        Some(line)
    }
}
