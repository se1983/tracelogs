#![allow(dead_code)]

extern crate tokio;

use tokenizer::LogLineTokenizer;
use regex::Regex;


mod reader;
mod tokenizer;


fn main() {

    let mut tokenizer = LogLineTokenizer::new(Regex::new(r"(?m)^\[").unwrap());

    tokenizer.append("this will not be tokenized\n");
    tokenizer.append("also this will [not]");
    tokenizer.append("do anything\n");
    tokenizer.append("[this will trigger it!");
    tokenizer.append("\n[");

}

