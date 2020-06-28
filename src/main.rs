extern crate termion;

use std::process::{Command, Stdio};
use std::str::FromStr;

use chrono::{NaiveDateTime};
use serde::{de, Deserialize, Deserializer};
use serde::export::fmt::Display;
use termion::{color, style};

// https://docs.rs/openssh/0.6.2/openssh/


fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s.replace("\"", "")).map_err(de::Error::custom)
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
struct JournalLogLine {
    #[serde(deserialize_with = "from_str")]
    _SOURCE_REALTIME_TIMESTAMP: i64,
    _HOSTNAME: String,
    _SYSTEMD_UNIT: Option<String>,
    MESSAGE: String,
}

impl JournalLogLine {
    pub fn date(&self) -> NaiveDateTime {
        let secs = (&self._SOURCE_REALTIME_TIMESTAMP / 1000000) as i64;
        let nsecs = (self._SOURCE_REALTIME_TIMESTAMP % 1000000) as u32;
        NaiveDateTime::from_timestamp(secs, nsecs)
    }

    pub fn header(&self) -> String {
        format!("{color}{unit}@{host} -- [{datetime}]{style_reset}",
                color = color::Fg(color::Yellow),
                style_reset = style::Reset,
                unit = &self._SYSTEMD_UNIT.clone().unwrap_or(String::from("")),
                host = &self._HOSTNAME,
                datetime = self.date()
        )
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


struct JournalDLog {
    lines: Vec<JournalLogLine>,
    line_idx: usize,
}

impl JournalDLog {
    pub fn new(unit: &str) -> Self {
        let unit_string = format!("--unit={}", unit);

        let pout = read_proc("journalctl", &[unit_string.as_str(), "--output=json", "--no-pager"]);
        let output = serde_json::Deserializer::from_str(&pout).into_iter::<JournalLogLine>();

        JournalDLog {
            lines: output.filter_map(Result::ok).collect(),
            line_idx: 0,
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

fn main() {
    for line in JournalDLog::new("sddm.service") {
        println!("{header}\n\t{msg}\n\n",
                 header = line.header(),
                 msg = line.MESSAGE);
    }
}
