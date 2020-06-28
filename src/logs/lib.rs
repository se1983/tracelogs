use chrono::NaiveDateTime;
use termion::{color, style};
use std::str::FromStr;
use serde::export::fmt::Display;
use serde::{Deserializer, Deserialize};

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

pub(crate) fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    use serde::de::Error;

    let s = String::deserialize(deserializer)?;
    T::from_str(&s.replace("\"", "")).map_err(Error::custom)
}