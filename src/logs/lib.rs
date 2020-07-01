use std::error::Error;
use std::process::{Command, Stdio};

use chrono::NaiveDateTime;
use openssh::{KnownHosts, Session};
use strfmt::strfmt;
use termion::{color, style};
use regex::{Regex, Captures};
use std::collections::HashMap;


#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct LogLine {
    timestamp: i64,
    hostname: String,
    service: String,
    pub(crate) message: String,
}

impl LogLine {
    pub fn new(timestamp: i64, hostname: String, service: String, message: String) -> LogLine {
        LogLine { timestamp, hostname, service, message }
    }
}

impl Tracer for LogLine{
    fn date(&self) -> NaiveDateTime {
        let secs = (&self.timestamp / 1000000) as i64;
        let nsecs = (&self.timestamp % 1000000000) as u32;
        NaiveDateTime::from_timestamp(secs, nsecs)
    }
    fn service(&self) -> String {
        self.service.clone()
    }
    fn hostname(&self) -> String {
        self.hostname.clone()
    }

    fn message(&self) -> String {
        self.message.clone()
    }
}

pub(crate) struct Logs {
    lines: Vec<LogLine>,
    line_idx: usize
}

impl Logs {
    pub fn new(lines: Vec<LogLine>) -> Logs {
        Logs { lines, line_idx: 0 }
    }

    pub fn new_from<T>(source: T) -> Self where T: LogSource {
        Logs {
            lines: source.lines(),
            line_idx: 0
        }
    }

    pub fn merge(&mut self, other: Self) -> Self {
        let mut lines = [&self.lines[..], &other.lines[..]].concat();
        lines.sort();
        Self::new(lines)
    }

    pub fn filter_logs(&self, exclude: &[&str], include: &[&str]) -> Logs {
        let lines: Vec<LogLine> = self.lines.iter().cloned()
            .filter(|x| x.includes(include))
            .filter(|x| !x.excludes(exclude))
            .collect();

        Logs{ lines, line_idx: 0 }
    }
}

impl Iterator for Logs {
    type Item = LogLine;
    fn next(&mut self) -> Option<Self::Item> {
        match self.line_idx {
            idx if idx < self.lines.len() => {
                let line = self.lines[self.line_idx].clone();
                self.line_idx += 1;
                Some(line)
            }
            _ => None,
        }
    }
}


pub trait Tracer {
    fn date(&self) -> NaiveDateTime;
    fn service(&self) -> String;
    fn hostname(&self) -> String;
    fn message(&self) -> String;
    fn header(&self) -> String {
        format!("{color}{unit}@{host} -- [{datetime}]{style_reset}",
                color = color::Fg(color::Yellow),
                style_reset = style::Reset,
                unit = self.service(),
                host = self.hostname(),
                datetime = self.date()
        )
    }

    fn includes(&self, words: &[&str]) -> bool{
        words.iter().all(|word| self.message().contains(word))
    }

    fn excludes(&self, words: &[&str]) -> bool{
        words.iter().any(|word| self.message().contains(word))
    }

    fn print_line(&self) {
        println!("{header}\n\t{msg}\n\n",
                 header = self.header(),
                 msg = self.message()
        );
    }
}

pub(crate) trait LogSource {
    fn lines(&self) -> Vec<LogLine>;
}


pub(super) fn read_proc(process: &str, args: &[&str]) -> Result<String, Box<dyn Error>> {
    let ps = Command::new(process)
        .stdout(Stdio::piped())
        .args(args)
        .output()?;
    let output = String::from_utf8_lossy(&ps.stdout).parse()?;

    Ok(output)
}

#[tokio::main]
pub(super) async fn read_remote_proc(process: &str, args: &[&str], addr: &str) -> Result<String, Box<dyn Error>> {
    let session = Session::connect(addr, KnownHosts::Strict).await?;
    let ps = session.command(process)
        .args(args)
        .output()
        .await?;
    session.close().await?;
    let output = String::from_utf8_lossy(&ps.stdout).parse()?;

    Ok(output)
}

#[derive(Debug)]
pub struct RegExtractor {
    datetime: String,
    host: String,
    service: String,
    message: String,
    line_pattern: String,
    regex: Regex,
    strftime_pattern: String,
}

#[allow(dead_code)]
impl RegExtractor {
    pub(crate) fn new(datetime_schema: &str, host_schema: &str, service_schema: &str, message_schema: &str, line_schema: &str, strftime_pattern: &str) -> RegExtractor {
        let mut vars = HashMap::new();

        vars.insert("d".to_string(), datetime_schema);
        vars.insert("h".to_string(), host_schema);
        vars.insert("s".to_string(), service_schema);
        vars.insert("m".to_string(), message_schema);

        let formated_log_pattern = strfmt(line_schema, &vars).unwrap();
        let re = Regex::new(&formated_log_pattern).unwrap();

        RegExtractor {
            datetime: String::from(datetime_schema),
            host: String::from(host_schema),
            service: String::from(service_schema),
            message: String::from(message_schema),
            line_pattern: formated_log_pattern,
            regex: re,
            strftime_pattern: String::from(strftime_pattern),
        }
    }

    pub fn get_fields<'t>(&self, logline: &'t str) -> Option<Captures<'t>> {
        let captures = self.regex.captures(logline);
        captures
    }

    pub fn timestamp_micros(&self, strftime: &str) -> i64{
        let date_time = NaiveDateTime::parse_from_str(strftime, &self.strftime_pattern).unwrap();
        let timestamp = date_time.timestamp() * 1000000 + date_time.timestamp_subsec_micros() as i64;
        timestamp
    }


}
