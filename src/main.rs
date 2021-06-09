#![allow(dead_code)]

extern crate tokio;

use tokenizer::LogLineTokenizer;
use regex::Regex;


mod reader;
mod tokenizer;


fn main() {

    let mut tokenizer = LogLineTokenizer::new(Regex::new(r"(?m)^\[").unwrap());

    tokenizer.push("this will not be tokenized\n");
    tokenizer.push("also this will [not]");
    tokenizer.push("do anything\n");
    tokenizer.push("[this will trigger it!");
    tokenizer.push("\n[");

}

