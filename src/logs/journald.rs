use std::str::FromStr;

use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};
use serde::export::fmt::Display;

use crate::config::ConfigFile;
use crate::logs::lib::{
    LogLine,
    LogSource,
    read_proc,
    read_remote_proc,
};
use crate::logs::Tracer;

#[derive(Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct JournalLogLine {
    #[serde(deserialize_with = "from_str", alias = "_SOURCE_REALTIME_TIMESTAMP")]
    timestamp: i64,
    #[serde(alias = "_HOSTNAME")]
    hostname: String,
    #[serde(alias = "_SYSTEMD_UNIT")]
    service: Option<String>,
    #[serde(alias = "MESSAGE")]
    message: String,
}

pub(super) fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    use serde::de::Error;

    let s = String::deserialize(deserializer)?;
    T::from_str(&s.replace("\"", "")).map_err(Error::custom)
}

impl Tracer for JournalLogLine {
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

pub struct JournalDLog {
    lines: Vec<JournalLogLine>,
}

impl JournalDLog {
    pub fn new(unit: &str, remote: Option<&str>) -> Self {
        let unit_string = format!("--unit={}", unit);
        let args = &[unit_string.as_str(), "--output=json", "--no-pager"];

        let pout = match remote {
            Some(addr) => read_remote_proc("journalctl", args, addr),
            _ => read_proc("journalctl", args)
        }.unwrap();

        let output = serde_json::Deserializer::from_str(&pout).into_iter::<JournalLogLine>();
        JournalDLog {
            lines: output.filter_map(Result::ok).collect(),
        }
    }
}

impl LogSource for JournalDLog {
    fn lines(&self) -> Vec<LogLine> {
        self.lines.iter().map(|l|
            LogLine::new(l.timestamp, l.hostname(), l.service(), l.message())
        ).collect()
    }
}


pub(crate) fn build_logs(config: &ConfigFile) -> Vec<JournalDLog> {
    config.targets.journald.iter().cloned().map(|t| {
        let host: Option<&str> = match t.host.as_str() {
            "localhost" => None,
            _ => Some(t.host.as_ref())
        };
        JournalDLog::new(&t.name, host)
    }).collect()
}
