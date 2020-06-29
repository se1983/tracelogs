use std::process::{Command, Stdio};

use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::logs::Tracer;

use super::lib::from_str;

// https://docs.rs/openssh/0.6.2/openssh/


#[derive(Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct JournalLogLine {
    #[serde(deserialize_with = "from_str", alias = "_SOURCE_REALTIME_TIMESTAMP")]
    timestamp: i64,
    #[serde(alias = "_HOSTNAME")]
    hostname: String,
    #[serde(alias = "_SYSTEMD_UNIT")]
    service: Option<String>,
    #[serde(alias = "MESSAGE")]
    pub(crate) message: String,
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
}

fn read_proc(process: &str, args: &[&str]) -> String {
    let output = Command::new(process)
        .stdout(Stdio::piped())
        .args(args)
        .output().unwrap();

    //println!("{}", String::from_utf8_lossy(&output.stdout));
    String::from_utf8_lossy(&output.stdout).parse().unwrap()
}


pub struct JournalDLog {
    lines: Vec<JournalLogLine>,
    line_idx: usize,
}

impl JournalDLog {
    pub fn new(unit: &str) -> Self {
        let unit_string = format!("--unit={}", unit);
        let pout = read_proc(
            "journalctl",
            &[unit_string.as_str(), "--output=json", "--no-pager"],
        );
        let output = serde_json::Deserializer::from_str(&pout).into_iter::<JournalLogLine>();

        JournalDLog {
            lines: output.filter_map(Result::ok).collect(),
            line_idx: 0,
        }
    }

    pub fn merge(&mut self, other: Self) -> Self {
        self.lines.extend(other.lines);
        self.lines.sort();

        Self {
            lines: self.lines.clone(),
            line_idx: self.line_idx,
        }
    }
}

impl Iterator for JournalDLog {
    type Item = JournalLogLine;
    fn next(&mut self) -> Option<Self::Item> {
        if self.line_idx >= self.lines.len() {
            return None;
        }

        let line = self.lines[self.line_idx].clone();
        self.line_idx += 1;
        Some(line)
    }
}