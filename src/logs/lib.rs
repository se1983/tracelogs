use chrono::NaiveDateTime;

pub trait Tracer {
    fn date(&self) -> NaiveDateTime;
    fn header(&self) -> String;
}