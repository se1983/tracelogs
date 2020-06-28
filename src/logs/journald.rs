extern crate termion;

use std::process::{Command, Stdio};
use std::str::FromStr;

use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};
use serde::export::fmt::Display;

use crate::logs::lib::Tracer;

// https://docs.rs/openssh/0.6.2/openssh/

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    use serde::de::Error;

    let s = String::deserialize(deserializer)?;
    T::from_str(&s.replace("\"", "")).map_err(Error::custom)
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct JournalLogLine {
    #[serde(deserialize_with = "from_str")]
    _SOURCE_REALTIME_TIMESTAMP: i64,
    _HOSTNAME: String,
    _SYSTEMD_UNIT: Option<String>,
    pub(crate) MESSAGE: String,
}

impl Tracer for JournalLogLine {
    fn date(&self) -> NaiveDateTime {
        let secs = (&self._SOURCE_REALTIME_TIMESTAMP / 1000000) as i64;
        let nsecs = (&self._SOURCE_REALTIME_TIMESTAMP % 1000000000) as u32;
        NaiveDateTime::from_timestamp(secs, nsecs)
    }

    fn service(&self) -> String {
        self._SYSTEMD_UNIT.clone().unwrap_or(String::from(""))
    }

    fn hostname(&self) -> String {
        self._SYSTEMD_UNIT.clone().unwrap_or(String::from(""))
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