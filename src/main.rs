extern crate tokio;

mod reader;

use reader::local_processes;

use log::LevelFilter;
use regex::Regex;
use simple_logger::SimpleLogger;

fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(r.as_str()) {
        if last != index {
            result.push(&text[last..index]);
        }
        result.push(matched);
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    result
}

struct NewLineTokenizer {
    lines: Vec<String>,
    newline_rgx: Regex,
}

impl NewLineTokenizer {
    fn new() -> Self {
        let lines = vec![];
        let newline_rgx = Regex::new(r"^\[").unwrap();

        NewLineTokenizer { lines, newline_rgx }
    }

    fn append(&mut self, line: &str) -> Vec<String> {
        self.lines.push(String::from(line));
        let full_string: String = self.lines.join("\n");
        let mut lines = split_keep(&self.newline_rgx, &full_string);
        if lines.len() >= 2 {
            self.lines = vec![lines.pop().unwrap().to_string()];
        }

        lines.iter().map(|x| String::from(*x)).collect()
    }
}

#[tokio::main]
async fn main() {
    log::set_max_level(LevelFilter::Info);
    SimpleLogger::new().init().unwrap();
    local_processes::watch().await;
}
