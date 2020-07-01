use std::error::Error;
use std::process::{Command, Stdio};

use chrono::NaiveDateTime;
use openssh::{KnownHosts, Session};

use termion::{color, style};



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