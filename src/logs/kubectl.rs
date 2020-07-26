use chrono::NaiveDateTime;

use crate::config::ConfigFile;
use crate::logs::{LogScheme, Tracer};
use crate::logs::lib::{LogLine, LogSource, read_proc, RegExtractor, split_keep};
use regex::Regex;

pub struct KubectlLogLine {
    timestamp: i64,
    hostname: String,
    service: Option<String>,
    message: String,
}

impl Tracer for KubectlLogLine {
    fn date(&self) -> NaiveDateTime {
        let secs = (&self.timestamp / 1000000) as i64;
        let nsecs = (&self.timestamp % 1000000000) as u32;
        NaiveDateTime::from_timestamp(secs, nsecs)
    }
    fn service(&self) -> String {
        self.service.clone().unwrap_or(String::from(""))
    }
    fn hostname(&self) -> String {
        self.hostname.clone()
    }
    fn message(&self) -> String {
        self.message.clone()
    }
}


pub struct KubectlLog {
    lines: Vec<KubectlLogLine>
}

impl KubectlLog {
    pub fn new(pod: &str, extractor: RegExtractor) -> KubectlLog {
        let args = &["logs", pod];

        let seperator = Regex::new(&extractor.split_pattern).expect("Invalid regex");
        let raw_ouput = read_proc("kubectl", args).unwrap();
        let splits: Vec<String> = split_keep(&seperator, &raw_ouput).chunks(2).map(|x|format!("{}{}", x[0], x[1])).collect();

        let lines: Vec<KubectlLogLine> = splits.iter()
            .map(|l| extractor.get_fields(&l))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .map(|x| KubectlLogLine {
                timestamp: extractor.timestamp_micros(&x["datetime"]),
                hostname: pod.to_string(),
                service: Some(x["service"].to_string()),
                message: x["message"].to_string(),
            }).collect();
        KubectlLog { lines }
    }

}


impl LogSource for KubectlLog {
    fn lines(&self) -> Vec<LogLine> {
        self.lines.iter().map(|l|
            LogLine::new(l.timestamp, l.hostname(), l.service(), l.message())
        ).collect()
    }
}


pub(crate) fn build_logs(config: &ConfigFile) -> Vec<KubectlLog> {
    config.targets.kubectl.iter().cloned().map(|t| {
        let scheme = LogScheme {
            date_time: t.regex.datetime,
            host: t.regex.host,
            service: t.regex.service,
            message: t.regex.message,
            whole_line: t.regex.log_pattern,
            split_pattern: t.regex.line_delimiter.pattern,
        };
        KubectlLog::new(&t.name, RegExtractor::new(scheme, &t.date_string))
    }).collect()
}









