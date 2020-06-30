use chrono::NaiveDateTime;
use termion::{color, style};
use std::str::FromStr;
use serde::export::fmt::Display;
use serde::{Deserializer, Deserialize};
use std::error::Error;
use std::process::{Command, Stdio};
use openssh::{Session, KnownHosts};

pub trait Tracer {
    fn date(&self) -> NaiveDateTime;
    fn service(&self) -> String;
    fn hostname(&self) -> String;

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

pub(super) fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    use serde::de::Error;

    let s = String::deserialize(deserializer)?;
    T::from_str(&s.replace("\"", "")).map_err(Error::custom)
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