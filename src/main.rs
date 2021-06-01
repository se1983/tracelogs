#![allow(dead_code)]

extern crate tokio;

use regex::Regex;

mod reader;


struct LogLineTokenizer {
    separator: Regex,
    buffer: String
}

impl LogLineTokenizer{
    pub fn new(separator: Regex) -> LogLineTokenizer {
        let buffer = String::from("");
        LogLineTokenizer{separator, buffer}
    }

    pub fn append(&mut self, text: &str) {
        self.buffer.push_str(text);
        self.tokenize();
    }

    fn tokenize(&mut self) {
        let tokenized = split_text(&self.separator, &self.buffer);

        if let Some((last, elements)) = tokenized.split_last(){
            for log_line in elements {
                println!("new logline: {}", log_line);
            }
            self.buffer = String::from(*last)

        }

    }
}

fn split_text<'a>(separator: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut slices = Vec::new();
    let mut lhs: usize = 0;

    let mut indices: Vec<usize> = separator.find_iter(text).map(|m| m.start()).collect();
    indices.push(text.len());

    for rhs in indices {
        slices.push(&text[lhs..rhs]);
        lhs = rhs;
    }

    slices
}

fn main() {

    let mut tokenizer = LogLineTokenizer::new(Regex::new(r"(?m)^\[").unwrap());

    tokenizer.append("this will not be tokenized\n");
    tokenizer.append("also this will [not]");
    tokenizer.append("do anything\n");
    tokenizer.append("[this will trigger it!");
    tokenizer.append("\n[");



}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_text() {
        let separator = Regex::new(r"(?m)^\[").unwrap();
        let text = "Mary \n[had a \nlittle lamb\n[";
        assert_eq!(split_text(&separator, &text), vec!["Mary \n", "[had a \nlittle lamb\n", "["])
    }

    #[test]
    fn test_split_text_no_match() {
        let separator = Regex::new(r"(?m)^\[").unwrap();
        let text = "little lamb";
        assert_eq!(split_text(&separator, &text), vec!["little lamb"])
    }
}