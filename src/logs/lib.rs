use chrono::NaiveDateTime;
use termion::{color, style};

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